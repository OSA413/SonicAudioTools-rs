use common_binary::{error::CommonBinaryError};

use crate::archives::cri::mw::table::{field_flag::CriFieldFlag, field_value::CriFieldValue};

pub struct CriTableField
{
    pub flag: CriFieldFlag,
    pub name: String,
    pub position: u32,
    pub length: u32,
    pub offset: u32, //Is this really needed?
    pub values: Vec<CriFieldValue>,
}

impl CriTableField {
    pub fn to_u8(&self, row_index: usize) -> Result<u8, CommonBinaryError> {
        if self.flag.contains(CriFieldFlag::Byte) {
            match &self.values[row_index] {
                CriFieldValue::Byte(value) => Ok(value.clone()),
                _ => panic!(""),
            }
        } else {
            panic!("")
        }
    }

    pub fn to_u16(&self, row_index: usize) -> Result<u16, CommonBinaryError> {
        if self.flag.contains(CriFieldFlag::UInt16) {
            match &self.values[row_index] {
                CriFieldValue::UInt16(value) => Ok(value.clone()),
                _ => panic!(""),
            }
        } else {
            panic!("")
        }
    }

    pub fn to_u32(&self, row_index: usize) -> Result<u32, CommonBinaryError> {
        if self.flag.contains(CriFieldFlag::UInt32) {
            match &self.values[row_index] {
                CriFieldValue::UInt32(value) => Ok(value.clone()),
                _ => panic!(""),
            }
        } else {
            panic!("")
        }
    }

    pub fn to_u64(&self, row_index: usize) -> Result<u64, CommonBinaryError> {
        if self.flag.contains(CriFieldFlag::UInt64) {
            match &self.values[row_index] {
                CriFieldValue::UInt64(value) => Ok(value.clone()),
                _ => panic!(""),
            }
        } else {
            panic!("")
        }
    }

    pub fn to_string(&self, row_index: usize) -> Result<String, CommonBinaryError> {
        if self.flag.contains(CriFieldFlag::String) {
            match &self.values[row_index] {
                CriFieldValue::String(value) => Ok(value.clone()),
                _ => panic!(""),
            }
        } else {
            panic!("")
        }
    }
}