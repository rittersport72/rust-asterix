use deku::prelude::*;
use std::mem;

// System Identification Code (SIC) and System Area Code (SAC)
//
#[derive(Default, Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
pub struct DataSource {
    sic: u8, // 1 byte
    sac: u8, // 1 byte
}

/*
* Implementation DataSource
*/
impl DataSource {
    /*
     * Set source id SIC
     */
    pub fn set_source_id_sic(&mut self, sic: u8) {
        self.sic = sic;
    }

    /*
     * Get source id SIC
     */
    pub fn get_source_id_sic(&self) -> u8 {
        return self.sic;
    }

    /*
     * Set source id SAC
     */
    pub fn set_source_id_sac(&mut self, sac: u8) {
        self.sac = sac;
    }

    /*
     * Get source id SAC
     */
    pub fn get_source_id_sac(&self) -> u8 {
        return self.sac;
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();
}

#[cfg(test)]
mod tests {
    use crate::uap::antenna_rotation_field::AntennaRotation;
    use super::*;

    #[test]
    fn check_data_source() {
        // Create message
        let mut data_source = DataSource::default();
        data_source.set_source_id_sic(42);
        data_source.set_source_id_sac(26);

        // Convert struct to byte stream
        let vector = data_source.to_bytes().unwrap();
        let array: &[u8] = &vector;

        // New message
        let (_rest, object) = DataSource::from_bytes((array, 0)).unwrap();
        let vvv = array.to_vec();

        assert_eq!(data_source.get_source_id_sic(), object.get_source_id_sic());
        assert_eq!(data_source.get_source_id_sac(), object.get_source_id_sac());
    }
}
