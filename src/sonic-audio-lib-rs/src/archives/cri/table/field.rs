use crate::archives::cri::table::field_flag::CriFieldFlag;

pub struct CriTableField
{
    pub flag: CriFieldFlag,
    pub name: String,
    pub position: u32,
    pub length: u32,
    pub offset: u32,
    pub value: Vec<u8>,
}