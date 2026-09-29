use bitflags::bitflags;

bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct CriFieldFlag: u8 {
        const Name = 16;
        const DefaultValue = 32;
        const RowStorage = 64;
        
        const Byte = 0;
        const SByte = 1;
        const UInt16 = 2;
        const Int16 = 3;
        const UInt32 = 4;
        const Int32 = 5;
        const UInt64 = 6;
        const Int64 = 7;
        const Single = 8;
        const Double = 9;
        const String = 10;
        const Data = 11;
        const Guid = 12;

        const TypeMask = 15;
        // The source may set any bits
        const _ = !0;
    }
}
