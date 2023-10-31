// System Configuration Status
//

/// Types of subfields with bit position from right to left
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Subfield {
    Com = 8,
    Psr = 5,
    Ssr = 4,
    Mds = 3,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct SystemConfigurationStatus {
    configuration_options: u8, // 1 byte
    com_subfield: u8,          // 1 byte
    psr_subfield: u8,          // 1 byte
    ssr_subfield: u8,          // 1 byte
    mds_subfield: u8,          // 1 byte
}

/*
* Implementation SystemConfigurationStatus
*/
impl SystemConfigurationStatus {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: u8) {
        self.configuration_options = array;
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> u8 {
        self.configuration_options
    }

    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes_subfield(&mut self, subfield: Subfield, array: u8) {
        match subfield {
            Subfield::Com => self.com_subfield = array,
            Subfield::Psr => self.psr_subfield = array,
            Subfield::Ssr => self.ssr_subfield = array,
            Subfield::Mds => self.mds_subfield = array,
        }
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes_subfield(&self, subfield: Subfield) -> u8 {
        match subfield {
            Subfield::Com => self.com_subfield,
            Subfield::Psr => self.psr_subfield,
            Subfield::Ssr => self.ssr_subfield,
            Subfield::Mds => self.mds_subfield,
        }
    }

    fn set_configuration_option(&mut self, subfield: Subfield) {
        let pattern = (0x1 as u8) << (subfield as u8 - 1);
        let field = u8::from_be(self.configuration_options);

        let result = field | pattern;
        self.configuration_options = result.to_be();
    }

    fn get_configuration_option(&self, subfield: Subfield) -> bool {
        let pattern = (0x1 as u8) << (subfield as u8 - 1);
        let field = u8::from_be(self.configuration_options);

        let result = field & pattern;
        return result == pattern;
    }

    /*
     * Convert struct to byte stream.
     */
    // pub fn to_bytes(&self) -> Box<[u8]> {
    //     // TODO: Count bits in options and assign array[1..3] to subfields
    //     if self.processing_options == 1 {
    //         let array = [0u8; 2];
    //         Box::new(array)
    //     } else {
    //         let array = [0u8; 3];
    //         Box::new(array)
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_system_configuration() {
        // Create message
        let mut system_configuration = SystemConfigurationStatus::default();
        system_configuration.set_configuration_option(Subfield::Com);

        // Convert struct to byte stream
        let array = system_configuration.to_bytes();

        // New message
        let mut object = SystemConfigurationStatus::default();

        // Convert byte stream to struct
        object.from_bytes(array);

        assert_eq!(system_configuration.get_configuration_option(Subfield::Com), object.get_configuration_option(Subfield::Com));
    }
}
