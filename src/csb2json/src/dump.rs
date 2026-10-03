use common_binary::{error::CommonBinaryError};
use serde::Serialize;
use sonic_audio_lib_rs::archives::cri::{
    cpk::archive::CriCpkArchive, mw::table::{
        field::CriTableField, field_value::CriFieldValue, header::CriTableHeader, header_encoding_type::CriTableHeaderEncodingType,
    },
};

const UTF_SIGNATURE: [u8; 4] = *b"@UTF";

#[derive(Serialize)]
pub struct JsonResult {
    pub csb: TableJson,
    pub cpk: Option<CpkJson>,
}

#[derive(Serialize)]
pub struct TableJson {
    pub name: String,
    pub encoding: String,
    pub fields: Vec<FieldJson>,
    pub rows: Vec<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Serialize)]
pub struct FieldJson {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: &'static str,
}

#[derive(Serialize)]
pub struct CpkJson {
    pub mode: String,
    pub comment: String,
    pub entries: Vec<CpkEntryJson>,
}

#[derive(Serialize)]
pub struct CpkEntryJson {
    pub id: u32,
    pub directory_name: String,
    pub name: String,
    pub table: Option<TableJson>,
}

pub fn dump(
    csb_source: &[u8],
    cpk_source: Option<&[u8]>,
) -> Result<JsonResult, CommonBinaryError> {
    let csb = dump_table(csb_source, Some(0))?;
    let cpk = match cpk_source {
        Some(source) => Some(dump_cpk(source)?),
        None => None,
    };
    Ok(JsonResult { csb, cpk })
}

fn dump_cpk(source: &[u8]) -> Result<CpkJson, CommonBinaryError> {
    let archive = CriCpkArchive::read(source, 0)?;

    let mut entries = Vec::with_capacity(archive.entries.len());
    for entry in &archive.entries {
        let mut table: Option<TableJson> = None;
        if is_table_data(source, entry.position as usize) {
            table = match dump_table(source, Some(entry.position as usize)) {
                Ok(table) => Some(table),
                Err(_) => None,
            }
        }

        entries.push(CpkEntryJson {
            id: entry.id,
            directory_name: entry.directory_name.clone(),
            name: entry.name.clone(),
            table,
        });
    }

    Ok(CpkJson {
        mode: format!("{:?}", archive.mode),
        comment: archive.comment,
        entries,
    })
}

fn dump_table(
    source: &[u8],
    pointer: Option<usize>,
) -> Result<TableJson, CommonBinaryError> {
    let header = CriTableHeader::read_table(source, pointer, UTF_SIGNATURE)?;

    let encoding = match header.encoding_type {
        CriTableHeaderEncodingType::ShiftJis => "shift-jis".to_string(),
        CriTableHeaderEncodingType::Utf8 => "utf-8".to_string(),
        CriTableHeaderEncodingType::Unknown(value) => format!("unknown-{value}"),
    };

    let fields = header
        .fields
        .iter()
        .map(|field| FieldJson {
            name: field.name.clone(),
            type_: field_type_name(field),
        })
        .collect();

    let mut rows = Vec::with_capacity(header.row_count as usize);
    for row_index in 0..header.row_count as usize {
        let mut values = serde_json::Map::new();
        for field in &header.fields {
            let json_value = serde_json::to_value(&field.values[row_index]).unwrap();
            values.insert(field.name.clone(), json_value);
        }
        rows.push(values);
    }

    Ok(TableJson {
        name: header.table_name,
        encoding,
        fields,
        rows,
    })
}

fn is_table_data(source: &[u8], pointer: usize) -> bool {
    source.len() - pointer >= UTF_SIGNATURE.len() && source[pointer..pointer+UTF_SIGNATURE.len()] == UTF_SIGNATURE
}

// Maybe this can be simplified?
fn field_type_name(field: &CriTableField) -> &'static str {
    match field.values[0] {
        CriFieldValue::Byte(_) => "byte",
        CriFieldValue::SByte(_) => "sbyte",
        CriFieldValue::UInt16(_) => "uint16",
        CriFieldValue::Int16(_) => "int16",
        CriFieldValue::UInt32(_) => "uint32",
        CriFieldValue::Int32(_) => "int32",
        CriFieldValue::UInt64(_) => "uint64",
        CriFieldValue::Int64(_) => "int64",
        CriFieldValue::Single(_) => "single",
        CriFieldValue::Double(_) => "double",
        CriFieldValue::String(_) => "string",
        CriFieldValue::Data(_) => "data",
        CriFieldValue::Guid(_) => "guid",
    }
}

