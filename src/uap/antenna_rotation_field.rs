use std::mem;
use deku::prelude::*;

// Antenna Rotation Period
//
#[derive(Default, Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
pub struct AntennaRotation {
    period: u16, // 2 bytes
}

/*
* Implementation AntennaRotation
*/
impl AntennaRotation {
    /*
     * Set period in seconds
     */
    pub fn set_period(&mut self, period: f32) {
        let converted_period = (period / Self::FACTOR) as u16;
        self.period = converted_period;
    }
    /*
     * Get period in seconds
     */
    pub fn get_period(&self) -> f32 {
        let converted_period: f32 = self.period as f32 * Self::FACTOR;
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
        let vector = antenna_rotation.to_bytes().unwrap();
        let array: &[u8] = &vector;

        // New message
        let (_rest, object) = AntennaRotation::from_bytes((array, 0)).unwrap();
        let vvv = array.to_vec();

        assert_eq!(antenna_rotation.get_period(), object.get_period());
    }
}
