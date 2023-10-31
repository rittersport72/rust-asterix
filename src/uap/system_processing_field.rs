// System Processing Mode
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
pub struct SystemProcessingMode {
    processing_options: u8, // 1 byte
    com_subfield: u8,       // 1 byte
    psr_subfield: u8,       // 1 byte
    ssr_subfield: u8,       // 1 byte
    mds_subfield: u8,       // 1 byte
}

/*
* Implementation SystemProcessingMode
*/
impl SystemProcessingMode {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: u8) {
        self.processing_options = array;
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> u8 {
        self.processing_options
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

    fn set_processing_option(&mut self, subfield: Subfield) {
        let pattern = (0x1 as u8) << (subfield as u8 - 1);
        let field = u8::from_be(self.processing_options);

        let result = field | pattern;
        self.processing_options = result.to_be();
    }

    fn get_processing_option(&self, subfield: Subfield) -> bool {
        let pattern = (0x1 as u8) << (subfield as u8 - 1);
        let field = u8::from_be(self.processing_options);

        let result = field & pattern;
        return result == pattern;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_position() {
        // Create message
        let mut system_processing = SystemProcessingMode::default();
        system_processing.set_processing_option(Subfield::Com);

        // Convert struct to byte stream
        let array = system_processing.to_bytes();

        // New message
        let mut object = SystemProcessingMode::default();

        // Convert byte stream to struct
        object.from_bytes(array);

        assert_eq!(system_processing.get_processing_option(Subfield::Com), object.get_processing_option(Subfield::Com));
    }
}
