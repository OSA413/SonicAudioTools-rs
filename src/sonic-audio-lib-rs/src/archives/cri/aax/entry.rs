use crate::archives::cri::aax::entry_flag::CriAaxEntryFlag;

pub struct CriAaxEntry {
    pub flag: CriAaxEntryFlag,
    pub length: u32,
    pub position: u32,
}