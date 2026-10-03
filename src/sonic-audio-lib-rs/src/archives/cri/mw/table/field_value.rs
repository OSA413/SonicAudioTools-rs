use serde::Serialize;
#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum CriFieldValue {
    Byte(u8),
    SByte(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    Single(f32),
    Double(f64),
    String(String),
    Data(
        #[serde(skip)]
        Vec<u8>
    ),
    Guid(u128), //I don't remember actually the underlying type of GUID/UUID but afair it's u128
}