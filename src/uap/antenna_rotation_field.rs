use std::mem;

// Antenna Rotation Period
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct AntennaRotation {
    period: u16, // 2 bytes
}

/*
* Implementation AntennaRotation
*/
impl AntennaRotation {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        let lo = array[0] as u16;
        let hi = (array[1] as u16) << 8;
        self.period = hi + lo;
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[0] = (self.period & 0x00ff) as u8;
        array[1] = ((self.period & 0xff00) >> 8) as u8;
        array
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set period in seconds
     */
    pub fn set_period(&mut self, period: f32) {
        let converted_period = (period / Self::FACTOR) as u16;
        self.period = converted_period.to_be();
    }

    /*
     * Get period in seconds
     */
    pub fn get_period(&self) -> f32 {
        let converted_period: f32 = u16::from_be(self.period) as f32 * Self::FACTOR;
        converted_period
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();

    /*
     * Conversion factor.
     */
    const FACTOR: f32 = 1.0 / 128.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_period() {
        // Create message
        let mut antenna_rotation = AntennaRotation::default();
        antenna_rotation.set_period(5.2);

        // Convert struct to byte stream
        let array = antenna_rotation.to_bytes();

        // New message
        let mut object = AntennaRotation::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(antenna_rotation.get_period(), object.get_period());
    }
}
