use common_binary::{binary_reader, endianness::Endianness, error::CommonBinaryError};

use crate::archives::cri::mw::table::field_flag::CriFieldFlag;

pub struct CriTableField
{
    pub flag: CriFieldFlag,
    pub name: String,
    pub position: u32,
    pub length: u32,
    pub offset: u32, //Is this really needed?
    pub value: Vec<u8>,
}

impl CriTableField {
    // Maybe redo in a more rustician/idiomatic way?
    pub fn to_u16(&self) -> Result<u16, CommonBinaryError> {
        // add more checks if the array is really of the value?
        binary_reader::u16::read(&self.value, 0, &Endianness::Big, "")
    }

    pub fn to_u32(&self) -> Result<u32, CommonBinaryError> {
        // add more checks if the array is really of the value?
        binary_reader::u32::read(&self.value, 0, &Endianness::Big, "")
    }

    pub fn to_u64(&self) -> Result<u64, CommonBinaryError> {
        // add more checks if the array is really of the value?
        binary_reader::u64::read(&self.value, 0, &Endianness::Big, "")
    }

    pub fn to_string(&self) -> Result<String, CommonBinaryError> {
        match binary_reader::string32::read(&self.value, 0, "") {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}