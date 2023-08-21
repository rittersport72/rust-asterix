use std::mem;

// Sector Number
//
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
//#[derive(Debug, PartialEq)]
pub struct SectorNumber {
    sector: u8, // 1 byte
}

/*
* Implementation SectorNumber
*/
impl SectorNumber {
    pub fn new() -> Self {
        Self { sector: 0 }
    }

    /*
     * Convert byte stream to struct. This uses unsafe.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        unsafe {
            *self = mem::transmute_copy::<[u8; Self::MESSAGE_LENGTH], Self>(array);
        }
    }

    /*
     * Convert struct to byte stream. This uses unsafe.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        unsafe { mem::transmute_copy::<Self, [u8; Self::MESSAGE_LENGTH]>(self) }
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set sector in degree
     */
    pub fn set_sector(&mut self, sector: f32) {
        let converted_sector: u8 = (sector / Self::FACTOR) as u8;
        self.sector = converted_sector.to_be();
    }

    /*
     * Get sector in degree
     */
    pub fn get_sector(&self) -> f32 {
        let converted_sector: f32 = u8::from_be(self.sector) as f32 * Self::FACTOR;
        converted_sector
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();

    /*
     * Conversion factor.
     */
    const FACTOR: f32 = 360.0 / 256.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sector() {
        // Create message
        let mut sector_number = SectorNumber::new();
        sector_number.set_sector(3.0);

        // Convert struct to byte stream
        let array = sector_number.to_bytes();

        // New message
        let mut object = SectorNumber::new();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(sector_number.get_sector(), object.get_sector());
    }
}
