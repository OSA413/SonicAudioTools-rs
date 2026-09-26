pub fn has_flag(value: u8, flag: u8) -> bool {
    value & flag == flag
}