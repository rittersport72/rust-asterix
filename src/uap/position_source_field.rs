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
     * Set latitude/longitude in degrees
     *  -90 <= latitude  <=  90
     * -180 <= longitude <= 180
     */
    pub fn set_lat_lon(&mut self, lat: f64, lon: f64) {
        let trunc = (lat * PositionSource::FACTOR).trunc();
        let converted_lat = trunc as i32;
        let lat_bytes = converted_lat.to_be_bytes();

        // Latitude
        self.position[3] = lat_bytes[0];
        self.position[4] = lat_bytes[1];
        self.position[5] = lat_bytes[2];

        let trunc = (lon * PositionSource::FACTOR).trunc();
        let converted_lon = trunc as i32;
        let lon_bytes = converted_lon.to_be_bytes();

        // Longitude
        self.position[0] = lon_bytes[0];
        self.position[1] = lon_bytes[1];
        self.position[2] = lon_bytes[2];
    }

    /*
     * Get latitude/longitude in degrees
     *  -90 <= latitude  <=  90
     * -180 <= longitude <= 180
     */
    pub fn get_lat_lon(&self) -> (f64, f64) {
        let hi = self.position[0] as u32;
        let mi = (self.position[1] as u32) << 8;
        let lo = (self.position[2] as u32) << 16;

        let converted_lat: f64 = (hi + mi + lo) as f64 * Self::FACTOR;

        let hi = self.position[3] as u32;
        let mi = (self.position[4] as u32) << 8;
        let lo = (self.position[5] as u32) << 16;

        let converted_lon: f64 = (hi + mi + lo) as f64 * Self::FACTOR;

        (converted_lat, converted_lon)
    }

    /*
     * Set height in meters
     */
    pub fn set_height(&mut self, height: i16) {
        let height_bytes = height.to_be_bytes();
        self.position[6] = height_bytes[0];
        self.position[7] = height_bytes[1];
    }

    /*
     * Get height in meters
     */
    pub fn get_height(&self) -> i16 {
        let hi = self.position[6] as u16;
        let lo = (self.position[7] as u16) << 8;
        
        let converted_time: i16 = (hi + lo) as i16;
        converted_time
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();

    /*
     * Conversion factor.
     */
    const FACTOR: f64 = 180.0 / u32::pow(2, 23) as f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_position() {
        // Create message
        let mut position_source = PositionSource::default();
        position_source.set_height(4242);
        position_source.set_lat_lon(12.345, 34.567);

        // Convert struct to byte stream
        let array = position_source.to_bytes();

        // New message
        let mut object = PositionSource::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(position_source.get_height(), object.get_height());
        assert_eq!(position_source.get_lat_lon(), object.get_lat_lon());
    }
}
