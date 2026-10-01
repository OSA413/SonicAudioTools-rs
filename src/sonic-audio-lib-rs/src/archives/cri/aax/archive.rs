use std::{ffi::OsStr, path::Path};

use common_binary::error::CommonBinaryError;

use crate::archives::cri::{aax::{archive_mode::CriAaxArchiveMode, entry::CriAaxEntry, entry_flag::CriAaxEntryFlag}, mw::table::header::CriTableHeader};

pub struct CriAaxArchive {
    pub mode: CriAaxArchiveMode,
    pub entries: Vec<CriAaxEntry>,
}

impl CriAaxArchive {
    pub fn read(source: &[u8], pointer: Option<usize>) -> Result<CriAaxArchive, CommonBinaryError> {
        let mut entries = Vec::new();
        let mut reader = CriTableHeader::read_table(source, pointer, [0x40, 0x55, 0x54, 0x46])?;
        let mode = match reader.table_name.as_str() {
            "AAX" => CriAaxArchiveMode::Adx,
            "ADPCM_WII" => CriAaxArchiveMode::Dsp,
            "SWLPCM" => CriAaxArchiveMode::Wav,
            value => {
                eprintln!("Unknown AAX type '{value}'. Please report the error with the file(s).");
                return Err(CommonBinaryError::SeeConsole());
            }
        };

        if mode == CriAaxArchiveMode::Dsp {
            eprintln!("DSP files aren't supported yet.");
            return Err(CommonBinaryError::SeeConsole());
        }

        while reader.read() {
            let (length, position) = reader.get_length_and_position(&source, "data");
            entries.push(CriAaxEntry {
                flag: CriAaxEntryFlag::from(reader.get_field("lpflg").unwrap().to_u8(reader.row_index as usize - 1)?),
                position,
                length
            });
        }

        Ok(CriAaxArchive {
            mode: mode,
            entries: entries
        })
    }

    pub fn add(&mut self, item: CriAaxEntry) {
        if self.entries.len() == 2 || self.entries.iter().find(|entry| entry.flag == item.flag).is_some() {
            return;
        }
        self.entries.push(item)
    }

    pub fn get_mode_extension(&self) -> &str {
        match self.mode {
            CriAaxArchiveMode::Adx => ".adx",
            CriAaxArchiveMode::Dsp => ".dsp",
            CriAaxArchiveMode::Wav => ".wav",
        }
    }

    pub fn set_mode_from_extension(&mut self, file_path: &OsStr) {
        self.mode = match Path::new(file_path).extension().unwrap().to_str().unwrap() {
            ".adx" => CriAaxArchiveMode::Adx,
            ".dsp" => CriAaxArchiveMode::Dsp,
            ".wav" => CriAaxArchiveMode::Wav,
            _ => panic!("This beat is not supported yet.")
        }
    }
}
