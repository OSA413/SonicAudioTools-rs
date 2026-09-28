#[derive(PartialEq, Debug)]
pub enum CriCpkMode {
    None = -1,
    FileNameIdAndGroup = 5,
    IdAndGroup = 4,
    FileNameAndGroup = 3,
    FileNameAndId = 2,
    FileName = 1,
    Id = 0,
}

impl From<u32> for CriCpkMode {
    fn from(value: u32) -> Self {
        match value {
            0 => CriCpkMode::Id,
            1 => CriCpkMode::FileName,
            2 => CriCpkMode::FileNameAndId,
            3 => CriCpkMode::FileNameAndGroup,
            4 => CriCpkMode::IdAndGroup,
            5 => CriCpkMode::FileNameIdAndGroup,
            _ => CriCpkMode::None,
        }
    }
}

impl From<CriCpkMode> for i32 {
    fn from(value: CriCpkMode) -> Self {
        match value {
            CriCpkMode::Id => 0,
            CriCpkMode::FileName => 1,
            CriCpkMode::FileNameAndId => 2,
            CriCpkMode::FileNameAndGroup => 3,
            CriCpkMode::IdAndGroup => 4,
            CriCpkMode::FileNameIdAndGroup => 5,
            _ => -1,
        }
    }
}