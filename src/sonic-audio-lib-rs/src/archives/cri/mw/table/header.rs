use core::panic;

use common_binary::{binary_reader, endianness::Endianness, error::CommonBinaryError};

use crate::{archives::cri::mw::table::{field::CriTableField, field_flag::CriFieldFlag, header_encoding_type::CriTableHeaderEncodingType}, has_flag::has_flag};

pub struct CriTableHeader {
    pub length: u32,
    pub unknown_byte: u8,
    pub encoding_type: CriTableHeaderEncodingType,
    pub rows_position: u16,
    pub string_pool_position: u32,
    pub data_pool_position: u32,
    pub table_name: String,
    pub field_count: u16,
    pub row_length: u32,
    pub row_count: u32,
    pub fields: Vec<CriTableField>,
    row_index: i32,
}

impl CriTableHeader {
    pub fn get_field(&self, field_name: &str) -> Option<&CriTableField> {
        self.fields.iter().find(|field| field.name == field_name)
    }

    pub fn go_to_value_ptr(&self, starting_pointer: Option<usize>, field_index: usize) -> usize {
        starting_pointer.unwrap_or(0) + (self.rows_position as usize) + (self.row_length as usize * self.row_index as usize) + self.fields[field_index].offset as usize
    }

    pub fn read(&mut self) -> bool {
        if (self.row_index + 1) as u32 >= self.row_count
        {
            return false;
        }

        self.row_index += 1;
        return true;
    }

    pub fn read_table(source: &[u8], pointer: Option<usize>, expected_segnature: [u8; 4]) -> Result<Self, CommonBinaryError> {
        let pointer = pointer.unwrap_or(0);

        if !source[pointer..pointer + 4].eq(&expected_segnature)
        {
            eprintln!("Invalid signature");
            return Err(CommonBinaryError::SeeConsole());
        }

        let header_length = binary_reader::u32::read(source, pointer + 0x04, &Endianness::Big, "")? + 0x8;
        let header_unknown_byte = binary_reader::u8::read(source, pointer + 0x08, "")?;
        let header_encoding_type = binary_reader::u8::read(source, pointer + 0x09, "")?;

        if header_unknown_byte != 0
        {
            eprintln!("Invalid byte for header_UnknownByte ({header_unknown_byte})");
            return Err(CommonBinaryError::SeeConsole());
        }

        match CriTableHeaderEncodingType::from(header_encoding_type) {
            CriTableHeaderEncodingType::ShiftJis => "shift-jis",
            CriTableHeaderEncodingType::Utf8 => "utf-8",
            CriTableHeaderEncodingType::Unknown(value) => {
                eprintln!("Unknown encoding type {value}");
                return Err(CommonBinaryError::SeeConsole())
            },
        };

        let header_rows_position = u16::try_from(binary_reader::u32::read(source, pointer + 0x0A, &Endianness::Big, "")? + 0x8).unwrap();
        let header_string_pool_position = binary_reader::u32::read(source, pointer + 0x0E, &Endianness::Big, "")? + 0x8;
        let header_data_pool_position = binary_reader::u32::read(source, pointer + 0x12, &Endianness::Big, "")? + 0x8;
        let header_table_name_pointer = binary_reader::u32::read(source, pointer + 0x16, &Endianness::Big, "")?;
        let header_field_count = binary_reader::u16::read(source, pointer + 0x1A, &Endianness::Big, "")?;
        let header_row_count = binary_reader::u32::read(source, pointer + 0x1E, &Endianness::Big, "")?;

        let mut offset= 0;
        let mut pointer = pointer + 0x22;

        let mut fields = Vec::with_capacity(header_field_count as usize);

        for _ in 0..header_field_count {
            let field_flag = binary_reader::u8::read(source, pointer, "")?;
            // Maybe redo as a builder pattern?
            let mut field_name_pointer = 0;
            let mut field_position = 0;
            let mut field_length = 0;
            let mut field_value = vec![0];
            let mut field_offset = 0;
            // /Maybe redo as a builder pattern?
            pointer += 0x01;

            if has_flag(field_flag, CriFieldFlag::Name as u8)
            {
                field_name_pointer = binary_reader::u32::read(source, pointer, &Endianness::Big, "")?;
                pointer += 0x04;
            }

            if has_flag(field_flag, CriFieldFlag::DefaultValue as u8) {
                if has_flag(field_flag, CriFieldFlag::Data as u8) {
                    field_position = binary_reader::u32::read(source, pointer, &Endianness::Big, "")?;
                    field_length = binary_reader::u32::read(source, pointer + 0x04, &Endianness::Big, "")?;
                    pointer += 0x08;
                }
                else
                {
                    // TODO?
                    todo!("TODO: #1 {}", field_flag);
                    // Probably a vec<u8>
                    // field_Value = ReadValue(field.Flag);
                }
            }

            // Not even per row, and not even constant value? Then there's no storage.
            else if !has_flag(field_flag, CriFieldFlag::RowStorage as u8)
                && !has_flag(field_flag, CriFieldFlag::DefaultValue as u8)
            {
                field_value = vec![0];
            }

            // Row storage, calculate the offset
            if has_flag(field_flag, CriFieldFlag::RowStorage as u8)
            {
                // Is this field needed?
                field_offset = offset;

                offset += match CriFieldFlag::from((field_flag) & (CriFieldFlag::TypeMask as u8)) {
                    CriFieldFlag::Byte => 1,
                    CriFieldFlag::SByte => 1,
                    CriFieldFlag::Int16 => 2,
                    CriFieldFlag::UInt16 => 2,
                    CriFieldFlag::Int32 => 4,
                    CriFieldFlag::UInt32 => 4,
                    CriFieldFlag::Single => 4,
                    CriFieldFlag::String => 4,
                    CriFieldFlag::Int64 => 8,
                    CriFieldFlag::UInt64 => 8,
                    CriFieldFlag::Double => 8,
                    CriFieldFlag::Data => 8,
                    _ => panic!("At the dance floor"),
                }
            }

            let field_name = binary_reader::string32::read(source, field_name_pointer as usize, "")?.0;

            fields.push(CriTableField {
                flag: CriFieldFlag::from(field_flag),
                name: field_name,
                position: field_position,
                length: field_length,
                value: field_value,
                offset: field_offset,
            });
        }

        let header_table_name = binary_reader::string32::read(source, header_table_name_pointer as usize, "")?.0;

        Ok(CriTableHeader {
            length: header_length,
            unknown_byte: header_unknown_byte,
            encoding_type: CriTableHeaderEncodingType::from(header_encoding_type),
            rows_position: header_rows_position,
            string_pool_position: header_string_pool_position,
            data_pool_position: header_data_pool_position,
            table_name: header_table_name,
            field_count: header_field_count,
            row_length: header_row_count,
            row_count: header_row_count,
            fields,
            row_index: -1,
        })
    }

    pub fn get_length_and_position(&self, source: &[u8], field_name: &str) -> (u32, u32) {
        let field_index = self.fields.iter().position(|x| x.name == field_name);

        let field_index = match field_index {
            Some(index) => index,
            None => return (0, 0),
        };

        if field_index < 0 || field_index as usize >= self.fields.len() {
            return (0, 0);
        }

        if !has_flag(self.fields[field_index].flag.clone() as u8, CriFieldFlag::RowStorage as u8) {
            return (
                self.fields[field_index].length,
                self.fields[field_index].position,
            );
        }

        let ptr = self.go_to_value_ptr(Some(0), field_index);

        return (
            binary_reader::u32::read(source, ptr + 4 as usize, &Endianness::Big, "").unwrap(),
            0 + self.data_pool_position + binary_reader::u32::read(source, ptr as usize, &Endianness::Big, "").unwrap()
        )
    }
}