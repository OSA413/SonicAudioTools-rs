#[repr(u8)]
pub enum CriTableHeaderEncodingType {
    ShiftJis = 0,
    Utf8 = 1,
    Unknown(u8)
}

impl From<CriTableHeaderEncodingType> for u8 {
    fn from(value: CriTableHeaderEncodingType) -> Self {
        match value {
            CriTableHeaderEncodingType::ShiftJis => 0,
            CriTableHeaderEncodingType::Utf8 => 1,
            CriTableHeaderEncodingType::Unknown(value) => value,
        }
    }
}

impl From<u8> for CriTableHeaderEncodingType {
    fn from(value: u8) -> Self {
        match value {
            0 => CriTableHeaderEncodingType::ShiftJis,
            1 => CriTableHeaderEncodingType::Utf8,
            _ => CriTableHeaderEncodingType::Unknown(value),
        }
    }
}
