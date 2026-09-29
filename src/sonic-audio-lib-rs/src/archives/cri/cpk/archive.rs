use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use common_binary::error::CommonBinaryError;

use crate::archives::cri::{cpk::{entry::CriCpkEntry, mode::CriCpkMode}, mw::table::header::CriTableHeader};

pub struct CriCpkArchive
{
    pub align: u16, //default: 1
    pub mode: CriCpkMode,//default: CriCpkMode::FileName,
    pub enable_mask: bool,//defaulT: false;
    pub comment: String, 
    pub entries: Vec<CriCpkEntry>,
}

impl CriCpkArchive {
    pub fn read(source: &[u8], pointer: usize) -> Result<CriCpkArchive, CommonBinaryError> {
        let mut reader = CriTableHeader::read_table(source, Some(pointer), *b"CPK ")?;
        reader.read();

        let (mode, is_latest_version) = match reader.get_field("CpkMode") {
            Some(is_latest_version) => {
                (CriCpkMode::from(is_latest_version.to_u32(reader.row_index - 1)?), true)
            }
            None => {
                let tocEnabled = reader.get_field("TocOffset").is_some_and(|x| x.to_u64(reader.row_index - 1).unwrap() > 0);
                let itocEnabled = reader.get_field("ItocOffset").is_some_and(|x| x.to_u64(reader.row_index - 1).unwrap() > 0);

                if tocEnabled && !itocEnabled {
                    (CriCpkMode::FileName, false)
                } else if !tocEnabled && itocEnabled {
                    (CriCpkMode::Id, false)
                } else if tocEnabled && itocEnabled {
                    (CriCpkMode::FileNameAndId, false)
                } else {
                    (CriCpkMode::None, false)
                }
            }
        };

        // No need to waste time, stop right there.
        if mode == CriCpkMode::None {
            return Ok(CriCpkArchive {
                align: 1,
                mode,
                enable_mask: false,
                comment: "".to_string(),
                entries: vec![],
            })
        }

        // Why is this u64?
        let tocPosition = reader.get_field("TocOffset").unwrap().to_u64(reader.row_index - 1)?;
        let itocPosition = reader.get_field("ItocOffset").unwrap().to_u64(reader.row_index - 1)?;
        let etocPosition = reader.get_field("EtocOffset").unwrap().to_u64(reader.row_index - 1)?;
        let contentPosition = reader.get_field("ContentOffset").unwrap().to_u64(reader.row_index - 1)?;

        let align = reader.get_field("Align").unwrap().to_u16(reader.row_index - 1)?;

        let mut entries = vec![];

        if mode == CriCpkMode::FileName || mode == CriCpkMode::FileNameAndId
        {            
            let mut tocReader = CriTableHeader::read_table(source, Some(tocPosition as usize), *b"TOC ")?;
            let mut etocReader = CriTableHeader::read_table(source, Some(etocPosition as usize), *b"ETOC")?;

            while tocReader.read() {
                let mut entry = CriCpkEntry {
                    directory_name: tocReader.get_field("DirName").unwrap().to_string(tocReader.row_index - 1)?,
                    name: tocReader.get_field("FileName").unwrap().to_string(tocReader.row_index - 1)?,
                    length: tocReader.get_field("FileSize").unwrap().to_u32(tocReader.row_index - 1)?,
                    position: tocReader.get_field("FileOffset").unwrap().to_u32(tocReader.row_index - 1)?,
                    id: match is_latest_version {
                        true => tocReader.get_field("ID").unwrap().to_u32(tocReader.row_index - 1)?,
                        false => tocReader.get_field("Info").unwrap().to_u32(tocReader.row_index - 1)?,
                    },
                    comment: tocReader.get_field("UserString").unwrap().to_string(tocReader.row_index - 1)?,
                    uncompressed_length: tocReader.get_field("ExtractSize").unwrap().to_u32(tocReader.row_index - 1)?,
                    update_date_time: NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                        NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                    ),
                    is_compressed: false,
                };
                entry.is_compressed = entry.length != entry.uncompressed_length;

                if contentPosition < tocPosition {
                    entry.position += contentPosition as u32;
                } else {
                    entry.position += tocPosition as u32;
                }

                // etocReader.MoveToRow(tocReader.CurrentRow);
                //entry.UpdateDateTime = DateTimeFromCpkDateTime(etocReader.GetUInt64("UpdateDateTime"));

                entries.push(entry);
            }

            if mode == CriCpkMode::FileNameAndId && is_latest_version
            {
                todo!("flooring the dance panic");
                // using (CriTableReader itocReader = CriCpkSection.Open(source, itocPosition, "ITOC"))
                // {
                //     while (itocReader.Read())
                //     {
                //         entries[itocReader.GetInt32("TocIndex")].Id = (uint)itocReader.GetInt32("ID");
                //     }
                // }
            }
        } else if mode == CriCpkMode::Id {
            todo!("working working working")
            // using (CriTableReader itocReader = CriCpkSection.Open(source, itocPosition, "ITOC"))
            // {
            //     while (itocReader.Read())
            //     {
            //         if (itocReader.GetUInt32("FilesL") > 0)
            //         {
            //             using (CriTableReader dataReader = itocReader.GetTableReader("DataL"))
            //             {
            //                 while (dataReader.Read())
            //                 {
            //                     CriCpkEntry entry = new CriCpkEntry
            //                     {
            //                         Id = dataReader.GetUInt16("ID"),
            //                         Length = dataReader.GetUInt16("FileSize"),
            //                         UncompressedLength = dataReader.GetUInt16("ExtractSize")
            //                     };
            //                     entry.IsCompressed = entry.Length != entry.UncompressedLength;

            //                     entries.Add(entry);
            //                 }
            //             }
            //         }

            //         if (itocReader.GetUInt32("FilesH") > 0)
            //         {
            //             using (CriTableReader dataReader = itocReader.GetTableReader("DataH"))
            //             {
            //                 while (dataReader.Read())
            //                 {
            //                     CriCpkEntry entry = new CriCpkEntry
            //                     {
            //                         Id = dataReader.GetUInt16("ID"),
            //                         Length = dataReader.GetUInt32("FileSize"),
            //                         UncompressedLength = dataReader.GetUInt32("ExtractSize")
            //                     };
            //                     entry.IsCompressed = entry.Length != entry.UncompressedLength;

            //                     entries.Add(entry);
            //                 }
            //             }
            //         }
            //     }
            // }

            // long entryPosition = contentPosition;
            // foreach (CriCpkEntry entry in entries.OrderBy(entry => entry.Id))
            // {
            //     entryPosition = Helpers.Align(entryPosition, align);

            //     entry.Position = entryPosition;
            //     entryPosition += entry.Length;
            // }
        } else {
            eprintln!("Unimplemented CPK mode ({mode:?})");
            return Err(CommonBinaryError::SeeConsole());
        }

        let comment = reader.get_field("Comment").unwrap().to_string(reader.row_index - 1)?;

        Ok(CriCpkArchive { align: 1, mode, enable_mask: false, comment, entries })
    }

    pub fn get_by_path(&self, path: &str) -> Option<&CriCpkEntry> {
        let corrected_path = path.replace("\\", "/");

        return self.entries.iter().find(|entry| {
            let search = match entry.directory_name.is_empty() {
                true => entry.name.clone(),
                false => format!("{}{}", entry.directory_name.replace("\\", "/"), entry.name),
            };

            return search == corrected_path;
        });
    }
}