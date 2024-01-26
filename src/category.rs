use crate::asterix::cat34::Cat34Message;

/// ASTERIX category
pub enum Category {
    Cat007,               // Directed Interrogation Messages
    Cat034(Cat34Message), // Transmission of Monoradar Service Messages
    Cat048,               // Monoradar Target Reports
    Cat062,               // System Track Data
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Category::Cat007 => write!(f, "Cat007"),
            Category::Cat034(_message) => write!(f, "Cat034"),
            Category::Cat048 => write!(f, "Cat048"),
            Category::Cat062 => write!(f, "Cat062"),
        }
    }
}

/// Errors for invalid message fields
#[derive(Debug, Clone, PartialEq)]
pub enum CatError {
    CategoryInvalid,
    SizeInvalid,
    I034_000Invalid,
    I034_010Invalid,
    I034_020Invalid,
    I034_030Invalid,
    I034_041Invalid,
    I034_050Invalid,
    I034_060Invalid,
    I034_070Invalid,
    I034_090Invalid,
    I034_100Invalid,
    I034_110Invalid,
    I034_120Invalid,
}

impl std::fmt::Display for CatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CatError::CategoryInvalid => write!(f, "Error: Category invalid"),
            CatError::SizeInvalid => write!(f, "Error: Data block size invalid"),
            CatError::I034_000Invalid => write!(f, "Error: I034_000 Message Type invalid"),
            CatError::I034_010Invalid => write!(f, "Error: I034_010 Data Source Identifier invalid"),
            CatError::I034_020Invalid => write!(f, "Error: I034_020 Sector Number invalid"),
            CatError::I034_030Invalid => write!(f, "Error: I034_030 Time of Day invalid"),
            CatError::I034_041Invalid => write!(f, "Error: I034_041 Antenna Rotation Period invalid"),
            CatError::I034_050Invalid => write!(f, "Error: I034_050 System Configuration & Status invalid"),
            CatError::I034_060Invalid => write!(f, "Error: I034_060 System Processing Mode invalid"),
            CatError::I034_070Invalid => write!(f, "Error: I034_070 Message Count Values invalid"),
            CatError::I034_090Invalid => write!(f, "Error: I034_090 Colimation Error invalid"),
            CatError::I034_100Invalid => write!(f, "Error: I034_100 Generic Polar Window invalid"),
            CatError::I034_110Invalid => write!(f, "Error: I034_110 Data Filter invalid"),
            CatError::I034_120Invalid => write!(f, "Error: I034_120 3D Position of Source invalid"),
        }
    }
}
