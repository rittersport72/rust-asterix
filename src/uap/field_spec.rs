use std::mem;

// Primary Subfield
// | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | bit
// |SF1|SF2|SF3|SF4|SF5|SF6|SF7| FX| subfield
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct FieldSpec {
    fspec: u8, // 1 byte
}

/*
* Implementation FieldSpec
*/
impl FieldSpec {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        self.fspec = array[0];
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[0] = self.fspec;
        array
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set fspec as bit field
     */
    pub fn set_fspec(&mut self, fspec: u8) {
        self.fspec = fspec.to_be();
    }

    /*
     * Get fspec as bit field
     */
    pub fn get_fspec(&self) -> u8 {
        u8::from_be(self.fspec)
    }

    /*
     * Set fspec bit value to 1 at index
     */
    pub fn set_fspec_bit(&mut self, index: u8) {
        // Range SF1..SF7..FX
        if index >= 1 && index <= 8 {
            // Shift by 1..8
            let pattern = (0x1 as u8) << (8 - index);
            let field = u8::from_be(self.fspec);

            let result = field | pattern;
            self.fspec = result.to_be();
        }
    }

    /*
     * Get fspec bit value at index
     */
    pub fn get_fspec_bit(&self, index: u8) -> bool {
        // Range SF1..SF7..FX
        if index >= 1 && index <= 8 {
            // Shift by 1..8
            let pattern = (0x1 as u8) << (8 - index);
            let field = u8::from_be(self.fspec);

            let result = field & pattern == pattern;
            return result;
        }

        return false;
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
    fn check_subfield() {
        // Create message
        let mut field_spec = FieldSpec::default();
        field_spec.set_fspec(0x0a);

        // Convert struct to byte stream
        let array = field_spec.to_bytes();

        // New message
        let mut object = FieldSpec::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(field_spec.get_fspec(), object.get_fspec());
    }

    #[test]
    fn check_set_fspec_bit() {
        // Create message
        let mut field_spec = FieldSpec::default();
        field_spec.set_fspec(0b00001111);

        field_spec.set_fspec_bit(1);

        assert_eq!(field_spec.get_fspec(), 0b10001111);
    }

    #[test]
    fn check_get_fspec_bit() {
        // Create message
        let mut field_spec = FieldSpec::default();
        field_spec.set_fspec(0b10001111);

        let bit = field_spec.get_fspec_bit(1);

        assert_eq!(bit, true);
    }
}
