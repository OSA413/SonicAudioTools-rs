use std::{fs::{self, create_dir_all}, path::{Path, PathBuf}};

use sonic_audio_lib_rs::archives::cri::{aax::{archive::CriAaxArchive, entry_flag::CriAaxEntryFlag}, cpk::archive::CriCpkArchive, mw::table::header::CriTableHeader};

fn find_cpk(path_to_search: &str) -> Option<PathBuf> {
    let path = &format!("{}.cpk", &path_to_search);
    let path = Path::new(path);
    if path.exists() {
        return Some(path.to_path_buf());
    }
    
    let path = &format!("{}.CPK", &path_to_search);
    let path = Path::new(path);
    if path.exists() {
        return Some(path.to_path_buf());
    }

    return None;
}

pub fn extract_csb(path: &str) {
    let path = Path::new(path);

    let extension = match path.extension() {
        Some(extension) => extension.to_ascii_lowercase(),
        None => panic!("No extension found"),
    };

    if extension == "csb" {
        let baseDirectory = path.parent().unwrap();
        let outputDirectoryName = baseDirectory.join(path.file_stem().unwrap());

        let mut cpk_archive: Option<CriCpkArchive> = None;
        let mut cpk_source: Option<Vec<u8>> = None;
        let found = find_cpk(&outputDirectoryName.to_string_lossy());

        let file_content = fs::read(path).unwrap();

        let mut reader = CriTableHeader::read_table(&file_content, Some(0), [0x40, 0x55, 0x54, 0x46]).unwrap();

        println!("{reader:?}");

        while reader.read() {
            if reader.get_field("name").unwrap().to_string(reader.row_index - 1).unwrap() == "SOUND_ELEMENT" {
                let table_length_and_position = reader.get_length_and_position(&file_content, "utf");
                let mut sdlReader = CriTableHeader::read_table(
                    &(file_content.as_slice())[table_length_and_position.1 as usize..table_length_and_position.0 as usize + table_length_and_position.1 as usize],
                    Some(0),
                    [0x40, 0x55, 0x54, 0x46]
                ).unwrap();

                while sdlReader.read()
                {
                    if sdlReader.get_field("fmt").unwrap().to_u16(sdlReader.row_index - 1).unwrap() != 0 {
                        panic!("The given CSB file contains an audio file which is not an ADX. Only CSB files with ADXs are supported.");
                    }

                    let streaming = sdlReader.get_field("stmflg").unwrap().to_u16(sdlReader.row_index - 1).unwrap() > 0;

                    if streaming && found.is_none() {
                        panic!("Cannot find the external .CPK file for this .CSB file. Please ensure that the external .CPK file is stored in the directory where the .CPK file is.");
                    }
                    else if streaming && found.is_some() && cpk_archive.is_none()
                    {
                        cpk_source = Some(fs::read(found.as_ref().unwrap()).unwrap());
                        cpk_archive = CriCpkArchive::read(&cpk_source.as_ref().unwrap(), 0).ok();
                    }

                    let sdlName = sdlReader.get_field("name").unwrap().to_string(sdlReader.row_index - 1).unwrap();
                    let destination_path = outputDirectoryName.join(&sdlName);
                    create_dir_all(&destination_path).unwrap();

                    if streaming {
                        let cpkEntry = cpk_archive.as_ref().unwrap().get_by_path(&sdlName.into_boxed_str());

                        match cpkEntry {
                            Some(cpkEntry) => {
                                let aaxSource = &cpk_source.as_ref().unwrap()[
                                    cpkEntry.position as usize..cpkEntry.position as usize + cpkEntry.length as usize
                                ];

                                let aax_archive = CriAaxArchive::read(aaxSource, Some(0)).unwrap();

                                for entry in aax_archive.entries {
                                    let adx_file_name = match entry.flag {
                                        CriAaxEntryFlag::Intro => "Intro.adx",
                                        CriAaxEntryFlag::Loop => "Loop.adx",
                                    };

                                    let data_start_pointer: usize = cpkEntry.position as usize + entry.position as usize;

                                    fs::write(
                                        destination_path.clone().join(adx_file_name),
                                        &cpk_source.as_ref().unwrap()[data_start_pointer..data_start_pointer + entry.length as usize],
                                    ).unwrap();
                                }
                            }
                            None => ()
                        }
                    } else {
                        let aaxPosition = sdlReader.get_length_and_position(&file_content, "data");
                        let aaxSource = &file_content[aaxPosition.1 as usize..aaxPosition.0 as usize + aaxPosition.1 as usize];
                        let aax_archive = CriAaxArchive::read(aaxSource, Some(0)).unwrap();

                        for entry in aax_archive.entries {
                            let adx_file_name = match entry.flag {
                                CriAaxEntryFlag::Intro => "Intro.adx",
                                CriAaxEntryFlag::Loop => "Loop.adx",
                            };

                            let data_start_pointer = table_length_and_position.1 + aaxPosition.1 + entry.position;

                            fs::write(
                                destination_path.join(adx_file_name),
                                &cpk_source.as_ref().unwrap()[data_start_pointer as usize..data_start_pointer as usize + entry.length as usize],
                            ).unwrap();
                        }
                    }
                }
            }
        }
    }
}