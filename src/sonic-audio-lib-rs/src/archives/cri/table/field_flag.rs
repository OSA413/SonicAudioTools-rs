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
