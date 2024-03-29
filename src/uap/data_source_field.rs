use std::mem;

// System Identification Code (SIC) and System Area Code (SAC)
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct DataSource {
    sic: u8, // 1 byte
    sac: u8, // 1 byte
}

/*
* Implementation DataSource
*/
impl DataSource {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        self.sic = array[0];
        self.sac = array[1];
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[0] = self.sic;
        array[1] = self.sac;
        array
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set source id SIC
     */
    pub fn set_source_id_sic(&mut self, sic: u8) {
        self.sic = sic.to_be();
    }

    /*
     * Get source id SIC
     */
    pub fn get_source_id_sic(&self) -> u8 {
        return u8::from_be(self.sic);
    }

    /*
     * Set source id SAC
     */
    pub fn set_source_id_sac(&mut self, sac: u8) {
        self.sac = sac.to_be();
    }

    /*
     * Get source id SAC
     */
    pub fn get_source_id_sac(&self) -> u8 {
        return u8::from_be(self.sac);
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_data_source() {
        // Create message
        let mut data_source = DataSource::default();
        data_source.set_source_id_sic(42);
        data_source.set_source_id_sac(26);

        // Convert struct to byte stream
        let array = data_source.to_bytes();

        // New message
        let mut object = DataSource::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(data_source.get_source_id_sic(), object.get_source_id_sic());
        assert_eq!(data_source.get_source_id_sac(), object.get_source_id_sac());
    }
}
