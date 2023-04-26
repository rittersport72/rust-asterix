use std::mem;

// Primary Subfield
// | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | bit
// |SF1|SF2|SF3|SF4|SF5|SF6|SF7| FX| subfield
//
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
//#[derive(Debug, PartialEq)]
pub struct FieldSpec {
    fspec: u8, // 1 byte
}

/*
* Implementation FieldSpec
*/
impl FieldSpec {
    pub fn new() -> Self {
        Self { fspec: 0 }
    }

    /*
     * Convert byte stream to struct. This uses unsafe.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        unsafe {
            *self = mem::transmute_copy::<[u8; Self::MESSAGE_LENGTH], Self>(array);
        }
    }

    /*
     * Convert struct to byte stream. This uses unsafe.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        unsafe { mem::transmute_copy::<Self, [u8; Self::MESSAGE_LENGTH]>(self) }
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
        self.fspec = fspec;
    }

    /*
     * Get fspec as bit field
     */
    pub fn get_fspec(&self) -> u8 {
        self.fspec
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
        let mut field_spec = FieldSpec::new();
        field_spec.set_fspec(0x0a);

        // Convert struct to byte stream
        let array = field_spec.to_bytes();

        // New message
        let mut object = FieldSpec::new();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(field_spec.get_fspec(), object.get_fspec());
    }
}
