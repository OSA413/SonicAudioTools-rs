#[derive(PartialEq)]
pub enum CriAaxEntryFlag {
    Intro = 0,
    Loop = 1,
}

impl From<u8> for CriAaxEntryFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => CriAaxEntryFlag::Intro,
            1 => CriAaxEntryFlag::Loop,
            value => panic!("Invalid value for CriAaxEntryFlag: {value}"),
        }
    }
}

impl From<CriAaxEntryFlag> for u8 {
    fn from(value: CriAaxEntryFlag) -> Self {
        match value {
            CriAaxEntryFlag::Intro => 0,
            CriAaxEntryFlag::Loop => 1,
        }
    }
}