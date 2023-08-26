
/// Errors for invalid message fields
#[derive(Debug, Clone, PartialEq)]
pub enum Cat34Error {
    I034SizeInvalid,
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

impl std::fmt::Display for Cat34Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Cat34Error::I034SizeInvalid => write!(f, "Error: I034 Data block size invalid"),
            Cat34Error::I034_000Invalid => write!(f, "Error: I034_000 Message Type invalid"),
            Cat34Error::I034_010Invalid => write!(f, "Error: I034_010 Data Source Identifier invalid"),
            Cat34Error::I034_020Invalid => write!(f, "Error: I034_020 Sector Number invalid"),
            Cat34Error::I034_030Invalid => write!(f, "Error: I034_030 Time of Day invalid"),
            Cat34Error::I034_041Invalid => write!(f, "Error: I034_041 Antenna Rotation Period invalid"),
            Cat34Error::I034_050Invalid => write!(f, "Error: I034_050 System Configuration & Status invalid"),
            Cat34Error::I034_060Invalid => write!(f, "Error: I034_060 System Processing Mode invalid"),
            Cat34Error::I034_070Invalid => write!(f, "Error: I034_070 Message Count Values invalid"),
            Cat34Error::I034_090Invalid => write!(f, "Error: I034_090 Colimation Error invalid"),
            Cat34Error::I034_100Invalid => write!(f, "Error: I034_100 Generic Polar Window invalid"),
            Cat34Error::I034_110Invalid => write!(f, "Error: I034_110 Data Filter invalid"),
            Cat34Error::I034_120Invalid => write!(f, "Error: I034_120 3D Position of Source invalid"),
        }
    }
}
