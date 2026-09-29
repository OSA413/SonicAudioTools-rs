use common_binary::{binary_reader, endianness::Endianness, error::CommonBinaryError};

use crate::archives::cri::mw::table::field_flag::CriFieldFlag;

#[derive(Debug)]
pub struct CriTableField
{
    pub flag: CriFieldFlag,
    pub name: String,
    pub position: u32,
    pub length: u32,
    pub offset: u32, //Is this really needed?
    pub value: Vec<Vec<u8>>,
}

impl CriTableField {

    pub fn get_convoluted_pointer(RowsPosition: usize, RowLength: usize, rowIndex: usize, fields: &Vec<CriTableField>, fieldIndex: usize) -> usize {
        0 + RowsPosition + (RowLength * rowIndex) + fields[fieldIndex].offset as usize
    }

    // Maybe redo in a more rustician/idiomatic way?
    pub fn to_u16(&self, row_index: usize) -> Result<u16, CommonBinaryError> {
        // add more checks if the array is really of the value?
        binary_reader::u16::read(&self.value[row_index], 0, &Endianness::Big, "")
    }

    pub fn to_u32(&self, row_index: usize) -> Result<u32, CommonBinaryError> {
        // add more checks if the array is really of the value?
        binary_reader::u32::read(&self.value[row_index], 0, &Endianness::Big, "")
    }

    pub fn to_u64(&self, row_index: usize) -> Result<u64, CommonBinaryError> {
        // add more checks if the array is really of the value?
        binary_reader::u64::read(&self.value[row_index], 0, &Endianness::Big, "")
    }

    pub fn to_string(&self, row_index: usize) -> Result<String, CommonBinaryError> {
        println!("{:?}", &self.value[row_index]);
        match binary_reader::string32::read(&self.value[row_index], 0, "") {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}