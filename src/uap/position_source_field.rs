use std::mem;

// 3D Position of Data Source
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct PositionSource {
    position: [u8; 8], // 8 bytes
}

/*
* Implementation PositionSource
*/
impl PositionSource {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        self.position = *array;
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        self.position
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set height in meters
     */
    pub fn set_height(&mut self, height: i16) {
        self.position[6] = (height & 0x00ff) as u8;
        self.position[7] = ((height & 0xff00u16 as i16) >> 8) as u8;
    }

    /*
     * Get height in meters
     */
    pub fn get_height(&self) -> i16 {
        let height = (self.position[6] as u16) + ((self.position[7] as u16) << 8);
        height as i16
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();

    /*
     * Conversion factor.
     */
    const FACTOR: f32 = 180.0 / u32::pow(2, 23) as f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_period() {
        // Create message
        let mut position_source = PositionSource::default();
        position_source.set_height(4242);

        // Convert struct to byte stream
        let array = position_source.to_bytes();

        // New message
        let mut object = PositionSource::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(position_source.get_height(), object.get_height());
    }
}
