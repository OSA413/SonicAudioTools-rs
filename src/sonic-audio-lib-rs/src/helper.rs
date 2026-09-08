// TODO improve
pub fn align(value: u64, alignment: u64) -> u64 {
    let mut value = value;
    while (value % alignment) != 0
    {
        value += 1;
    }

    value
}