use core::panic;

use common_binary::{binary_reader, endianness::Endianness, error::CommonBinaryError};

use crate::{archives::cri::mw::table::{field::CriTableField, field_flag::CriFieldFlag, field_value::CriFieldValue, header_encoding_type::CriTableHeaderEncodingType}, has_flag::has_flag};

pub struct CriTableHeader {
    pub length: u32,
    pub unknown_byte: u8,
    pub encoding_type: CriTableHeaderEncodingType,
    pub rows_position: u16,
    pub string_pool_position: u32,
    pub data_pool_position: u32,
    pub table_name: String,
    pub field_count: u16,
    pub row_length: u16,
    pub row_count: u32,
    pub fields: Vec<CriTableField>,
    pub row_index: usize,
}

// Isn't this function beautiful?
pub fn read_value_by_flag(type_: &CriFieldFlag, source: &[u8], starting_pointer: usize, header_string_pool_position: usize, header_data_pool_position: usize, value_pointer: usize) -> Result<CriFieldValue, CommonBinaryError> {
    return if type_.contains(CriFieldFlag::Guid) { todo!("guid"); } 
    else if type_.contains(CriFieldFlag::Data) {
        let position = binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")? as usize;
        let length = binary_reader::u32::read(source, value_pointer + 0x04, &Endianness::Big, "")? as usize;
        let start = starting_pointer + position + header_data_pool_position as usize;
        Ok(CriFieldValue::Data(source[start..start + length].to_vec()))
    }
    else if type_.contains(CriFieldFlag::String) {
        let string_pointer = binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")? as usize;
        let string_pointer = starting_pointer + header_string_pool_position as usize + string_pointer;
        let string = binary_reader::string::read(source, string_pointer, "")?.0;
        Ok(CriFieldValue::String(string))
    }
    else if type_.contains(CriFieldFlag::Double) { Ok(CriFieldValue::Double(binary_reader::f64::read(source, value_pointer, &Endianness::Big, "")?)) }
    else if type_.contains(CriFieldFlag::UInt64) { Ok(CriFieldValue::UInt64(binary_reader::u64::read(source, value_pointer, &Endianness::Big, "")?)) }
    else if type_.contains(CriFieldFlag::Int64) { Ok(CriFieldValue::Int64(binary_reader::u64::read(source, value_pointer, &Endianness::Big, "")? as i64)) }
    else if type_.contains(CriFieldFlag::Single) { Ok(CriFieldValue::Single(binary_reader::f32::read(source, value_pointer, &Endianness::Big, "")?)) }
    else if type_.contains(CriFieldFlag::UInt32) { Ok(CriFieldValue::UInt32(binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")?)) }
    else if type_.contains(CriFieldFlag::Int32) { Ok(CriFieldValue::Int32(binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")? as i32)) }
    else if type_.contains(CriFieldFlag::UInt16) { Ok(CriFieldValue::UInt16(binary_reader::u16::read(source, value_pointer, &Endianness::Big, "")?)) }
    else if type_.contains(CriFieldFlag::Int16) { Ok(CriFieldValue::Int16(binary_reader::u16::read(source, value_pointer, &Endianness::Big, "")? as i16)) }
    else if type_.contains(CriFieldFlag::SByte) { Ok(CriFieldValue::SByte(binary_reader::u8::read(source, value_pointer, "")? as i8)) }
    else if type_.contains(CriFieldFlag::Byte) { Ok(CriFieldValue::Byte(binary_reader::u8::read(source, value_pointer, "")?)) }
    else { panic!("At the dance floor, {type_:?}") };
}

pub fn get_null_value_by_flag(type_: &CriFieldFlag, ) -> CriFieldValue {
    return if type_.contains(CriFieldFlag::Guid) { todo!("guid"); } 
    else if type_.contains(CriFieldFlag::Data) { CriFieldValue::Data(vec![])}
    else if type_.contains(CriFieldFlag::String) { CriFieldValue::String(String::new()) }
    else if type_.contains(CriFieldFlag::Double) { CriFieldValue::Double(0.0) }
    else if type_.contains(CriFieldFlag::UInt64) { CriFieldValue::UInt64(0) }
    else if type_.contains(CriFieldFlag::Int64) { CriFieldValue::Int64(0) }
    else if type_.contains(CriFieldFlag::Single) { CriFieldValue::Single(0.0) }
    else if type_.contains(CriFieldFlag::UInt32) { CriFieldValue::UInt32(0) }
    else if type_.contains(CriFieldFlag::Int32) { CriFieldValue::Int32(0) }
    else if type_.contains(CriFieldFlag::UInt16) { CriFieldValue::UInt16(0) }
    else if type_.contains(CriFieldFlag::Int16) { CriFieldValue::Int16(0) }
    else if type_.contains(CriFieldFlag::SByte) { CriFieldValue::SByte(0) }
    else if type_.contains(CriFieldFlag::Byte) { CriFieldValue::Byte(0) }
    else { panic!("At the dance floor, {type_:?}") };
}

impl CriTableHeader {
    pub fn get_field(&self, field_name: &str) -> Option<&CriTableField> {
        self.fields.iter().find(|field| field.name == field_name)
    }

    pub fn go_to_value_ptr(starting_pointer: Option<usize>, rows_position: usize, row_length: usize, row_index: usize, offset: usize) -> usize {
        starting_pointer.unwrap_or(0) + rows_position + (row_length * row_index) + offset
    }

    pub fn read(&mut self) -> bool {
        if self.row_index as u32 >= self.row_count {
            return false;
        }

        self.row_index += 1;
        return true;
    }

    pub fn read_table(source: &[u8], pointer: Option<usize>, expected_segnature: [u8; 4]) -> Result<Self, CommonBinaryError> {
        let starting_pointer = pointer.unwrap_or(0);

        if !source[starting_pointer..starting_pointer + 4].eq(&expected_segnature) {
            eprintln!("Invalid signature");
            return Err(CommonBinaryError::SeeConsole());
        }

        let header_length = binary_reader::u32::read(source, starting_pointer + 0x04, &Endianness::Big, "")? + 0x8;
        let header_unknown_byte = binary_reader::u8::read(source, starting_pointer + 0x08, "")?;
        let header_encoding_type = binary_reader::u8::read(source, starting_pointer + 0x09, "")?;

        if header_unknown_byte != 0 {
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

        let header_rows_position = binary_reader::u16::read(source, starting_pointer + 0x0A, &Endianness::Big, "")? + 0x8;
        let header_string_pool_position = binary_reader::u32::read(source, starting_pointer + 0x0C, &Endianness::Big, "")? + 0x8;
        let header_data_pool_position = binary_reader::u32::read(source, starting_pointer + 0x10, &Endianness::Big, "")? + 0x8;
        let header_relative_table_name_pointer = binary_reader::u32::read(source, starting_pointer + 0x14, &Endianness::Big, "")?;
        let header_field_count = binary_reader::u16::read(source, starting_pointer + 0x18, &Endianness::Big, "")?;
        let header_row_length = binary_reader::u16::read(source, starting_pointer + 0x1A, &Endianness::Big, "")?;
        let header_row_count = binary_reader::u32::read(source, starting_pointer + 0x1C, &Endianness::Big, "")?;

        let pointer = starting_pointer + 0x20;

        let mut fields: Vec<CriTableField> = Vec::with_capacity(header_field_count as usize);

        let mut field_index_pointer = 0;

        let mut row_storage_offset = 0;

        for field_index in 0..header_field_count {
            let field_flag = CriFieldFlag::from_bits(binary_reader::u8::read(source, pointer + field_index_pointer, "")?).unwrap();
            field_index_pointer += 0x01;
            // Maybe redo as a builder pattern?
            let mut field_name_pointer = 0;
            let mut field_position = 0;
            let mut field_length = 0;
            let mut field_offset = 0;
            // /Maybe redo as a builder pattern?

            if has_flag(field_flag.bits(), CriFieldFlag::Name.bits())
            {
                field_name_pointer = binary_reader::u32::read(source, pointer + field_index_pointer, &Endianness::Big, "")?;
                field_index_pointer += 0x04;
            }

            let field_name = binary_reader::string32::read(source, starting_pointer + header_string_pool_position as usize + field_name_pointer as usize, "").unwrap().0;

            let type_ = field_flag.clone() & CriFieldFlag::TypeMask;

            let type_offset =
                if type_.contains(CriFieldFlag::Data) { 8 }
                else if type_.contains(CriFieldFlag::Double) { 8 }
                else if type_.contains(CriFieldFlag::UInt64) { 8 }
                else if type_.contains(CriFieldFlag::Int64) { 8 }
                else if type_.contains(CriFieldFlag::String) { 4 }
                else if type_.contains(CriFieldFlag::Single) { 4 }
                else if type_.contains(CriFieldFlag::UInt32) { 4 }
                else if type_.contains(CriFieldFlag::Int32) { 4 }
                else if type_.contains(CriFieldFlag::UInt16) { 2 }
                else if type_.contains(CriFieldFlag::Int16) { 2 }
                else if type_.contains(CriFieldFlag::SByte) { 1 }
                else if type_.contains(CriFieldFlag::Byte) { 1 }
                else { panic!("At the dance floor, {type_:?}") };

            // This value is shared between all default rows...
            let mut field_default_value = CriFieldValue::Byte(0);

            if has_flag(field_flag.bits(), CriFieldFlag::DefaultValue.bits())
            {
                if has_flag(field_flag.bits(), CriFieldFlag::Data.bits())
                {
                    field_position = binary_reader::u32::read(source, pointer + field_index_pointer, &Endianness::Big, "")?;
                    field_length = binary_reader::u32::read(source, pointer + field_index_pointer + 0x4, &Endianness::Big, "")?;
                    field_index_pointer += 0x08;
                }
                else
                {
                    field_default_value = read_value_by_flag(&type_, source, starting_pointer, header_string_pool_position as usize, header_data_pool_position as usize, pointer + field_index_pointer)?;
                    field_index_pointer += type_offset;
                }
            }

            if has_flag(field_flag.bits(), CriFieldFlag::RowStorage.bits())
            {
                field_offset = row_storage_offset;
                row_storage_offset += type_offset as u32;
            }

            for row_index in 0..header_row_count {
                let mut row_value = CriFieldValue::Byte(0);
                if has_flag(field_flag.bits(), CriFieldFlag::RowStorage.bits())
                {
                    let value_pointer = CriTableHeader::go_to_value_ptr(Some(starting_pointer), header_rows_position as usize, header_row_length as usize, row_index as usize, field_offset as usize);
                    row_value = read_value_by_flag(&type_, source, starting_pointer, header_string_pool_position as usize, header_data_pool_position as usize, value_pointer)?;
                }
                else if has_flag(field_flag.bits(), CriFieldFlag::DefaultValue.bits())
                    && !has_flag(field_flag.bits(), CriFieldFlag::Data.bits())
                {
                    row_value = field_default_value.clone();
                }
                // Not even per row, and not even constant value? Then there's no storage.
                else if !has_flag(field_flag.bits(), CriFieldFlag::RowStorage.bits())
                    && !has_flag(field_flag.bits(), CriFieldFlag::DefaultValue.bits())
                {
                    row_value = get_null_value_by_flag(&type_);
                }

                if fields.len() <= field_index as usize {
                    fields.push(CriTableField {
                        flag: field_flag.clone(),
                        name: field_name.clone(),
                        position: field_position,
                        length: field_length,
                        values: vec![],
                        offset: field_offset as u32,
                    });
                }

                fields[field_index as usize].values.push(row_value);
            }
        }

        let header_table_name = binary_reader::string32::read(source, starting_pointer + header_string_pool_position as usize + header_relative_table_name_pointer as usize, "").unwrap().0;

        Ok(CriTableHeader {
            length: header_length,
            unknown_byte: header_unknown_byte,
            encoding_type: CriTableHeaderEncodingType::from(header_encoding_type),
            rows_position: header_rows_position,
            string_pool_position: header_string_pool_position,
            data_pool_position: header_data_pool_position,
            table_name: header_table_name,
            field_count: header_field_count,
            row_length: header_row_length,
            row_count: header_row_count,
            fields,
            row_index: 0,
        })
    }

    pub fn get_length_and_position(&self, source: &[u8], field_name: &str) -> (u32, u32) {
        let field_index = self.fields.iter().position(|x| x.name == field_name);

        let field_index = match field_index {
            Some(index) => index,
            None => return (0, 0),
        };

        if field_index as usize >= self.fields.len() {
            return (0, 0);
        }

        if !has_flag(self.fields[field_index].flag.bits(), CriFieldFlag::RowStorage.bits()) {
            return (
                self.fields[field_index].length,
                self.fields[field_index].position,
            );
        }

        let ptr = CriTableHeader::go_to_value_ptr(Some(0), self.rows_position as usize, self.row_length as usize, self.row_index - 1, self.fields[field_index].offset as usize);

        return (
            binary_reader::u32::read(source, ptr + 4, &Endianness::Big, "").unwrap(),
            0 + self.data_pool_position + binary_reader::u32::read(source, ptr, &Endianness::Big, "").unwrap()
        )
    }
}