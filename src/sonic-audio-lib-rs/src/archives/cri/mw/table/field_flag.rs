#[repr(u8)]
#[derive(Clone)]
pub enum CriFieldFlag{
    Name = 16,
    DefaultValue = 32,
    RowStorage = 64,
    
    Byte = 0,
    SByte = 1,
    UInt16 = 2,
    Int16 = 3,
    UInt32 = 4,
    Int32 = 5,
    UInt64 = 6,
    Int64 = 7,
    Single = 8,
    Double = 9,
    String = 10,
    Data = 11,
    Guid = 12,

    TypeMask = 15,
}


impl From<CriFieldFlag> for u8 {
    fn from(value: CriFieldFlag) -> Self {
        match value {
            CriFieldFlag::Byte => 0,
            CriFieldFlag::SByte => 1,
            CriFieldFlag::UInt16 => 2,
            CriFieldFlag::Int16 => 3,
            CriFieldFlag::UInt32 => 4,
            CriFieldFlag::Int32 => 5,
            CriFieldFlag::UInt64 => 6,
            CriFieldFlag::Int64 => 7,
            CriFieldFlag::Single => 8,
            CriFieldFlag::Double => 9,
            CriFieldFlag::String => 10,
            CriFieldFlag::Data => 11,
            CriFieldFlag::Guid => 12,
            CriFieldFlag::TypeMask => 15,
            CriFieldFlag::Name => 16,
            CriFieldFlag::DefaultValue => 32,
            CriFieldFlag::RowStorage => 64,
        }
    }
}

impl From<u8> for CriFieldFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => CriFieldFlag::Byte,
            1 => CriFieldFlag::SByte,
            2 => CriFieldFlag::UInt16,
            3 => CriFieldFlag::Int16,
            4 => CriFieldFlag::UInt32,
            5 => CriFieldFlag::Int32,
            6 => CriFieldFlag::UInt64,
            7 => CriFieldFlag::Int64,
            8 => CriFieldFlag::Single,
            9 => CriFieldFlag::Double,
            10 => CriFieldFlag::String,
            11 => CriFieldFlag::Data,
            12 => CriFieldFlag::Guid,
            15 => CriFieldFlag::TypeMask,
            16 => CriFieldFlag::Name,
            32 => CriFieldFlag::DefaultValue,
            64 => CriFieldFlag::RowStorage,
            _ => panic!("Invalid CriFieldFlag value: {}", value)
        }
    }
}
