use std::mem;

// Generic Polar Window
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct GenericPolarWindow {
    rho_start: u16,   // 2 bytes
    rho_end: u16,     // 2 bytes
    theta_start: u16, // 2 bytes
    theta_end: u16,   // 2 bytes
}

/*
* Implementation GenericPolarWindow
*/
impl GenericPolarWindow {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        self.rho_start = array[6] as u16 + (array[7] as u16) << 8;
        self.rho_end = array[4] as u16 + (array[5] as u16) << 8;
        self.theta_start = array[2] as u16 + (array[3] as u16) << 8;
        self.theta_end = array[0] as u16 + (array[1] as u16) << 8;
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        let rho_start_bytes = self.rho_start.to_be_bytes();
        let rho_end_bytes = self.rho_end.to_be_bytes();
        let theta_start_bytes = self.theta_start.to_be_bytes();
        let theta_end_bytes = self.theta_end.to_be_bytes();

        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[7] = rho_start_bytes[0];
        array[6] = rho_start_bytes[1];
        array[5] = rho_end_bytes[0];
        array[4] = rho_end_bytes[1];
        array[3] = theta_start_bytes[0];
        array[2] = theta_start_bytes[1];
        array[1] = theta_end_bytes[0];
        array[0] = theta_end_bytes[1];
        array
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set rho in nautical miles
     * 1/256 <= start <= 256
     * 1/256 <= end   <= 256
     */
    pub fn set_rho(&mut self, start: f32, end: f32) {
        // Start
        let trunc = (start * GenericPolarWindow::RANGE_FACTOR).trunc();
        let converted_start = trunc as u16;

        self.rho_start = converted_start.to_be();

        // End
        let trunc = (end * GenericPolarWindow::RANGE_FACTOR).trunc();
        let converted_end = trunc as u16;

        self.rho_end = converted_end.to_be();
    }

    /*
     * Get rho in nautical miles
     * 1/256 <= start <= 256
     * 1/256 <= end   <= 256
     */
    pub fn get_rho(&self) -> (f32, f32) {
        let converted_start: f32 =
            u16::from_be(self.rho_start) as f32 * GenericPolarWindow::RANGE_FACTOR;
        let converted_end: f32 =
            u16::from_be(self.rho_end) as f32 * GenericPolarWindow::RANGE_FACTOR;

        (converted_start, converted_end)
    }

    /*
     * Set theta in degrees
     * 360/(2^16) <= start <= 360
     * 360/(2^16) <= end   <= 360
     */
    pub fn set_theta(&mut self, start: f64, end: f64) {
        // Start
        let trunc = (start * GenericPolarWindow::ANGLE_FACTOR).trunc();
        let converted_start = trunc as u16;

        self.theta_start = converted_start.to_be();

        // End
        let trunc = (end * GenericPolarWindow::ANGLE_FACTOR).trunc();
        let converted_end = trunc as u16;

        self.theta_end = converted_end.to_be();
    }

    /*
     * Get theta in degrees
     * 360/(2^16) <= start <= 360
     * 360/(2^16) <= end   <= 360
     */
    pub fn get_theta(&self) -> (f64, f64) {
        let converted_start: f64 =
            u16::from_be(self.theta_start) as f64 * GenericPolarWindow::ANGLE_FACTOR;
        let converted_end: f64 =
            u16::from_be(self.theta_end) as f64 * GenericPolarWindow::ANGLE_FACTOR;

        (converted_start, converted_end)
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();

    /*
     * Conversion factor.
     */
    const RANGE_FACTOR: f32 = 1.0 / 256 as f32;
    const ANGLE_FACTOR: f64 = 360.0 / u32::pow(2, 16) as f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_polar_window() {
        // Create message
        let mut generic_polar_window = GenericPolarWindow::default();
        generic_polar_window.set_rho(0.1, 123.4);
        generic_polar_window.set_theta(0.1, 12.34);

        // Convert struct to byte stream
        let array = generic_polar_window.to_bytes();

        // New message
        let mut object = GenericPolarWindow::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(generic_polar_window.get_rho(), object.get_rho());
        assert_eq!(generic_polar_window.get_theta(), object.get_theta());
    }
}
