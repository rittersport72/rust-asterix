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
        unsafe { *self = mem::transmute_copy::<[u8; Self::MESSAGE_LENGTH], Self>(array); }
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
    fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set subfield from bit right to left. [8..1]
     */
    pub fn set_subfield(&mut self, bit: u8, state: bool) {
        if bit < 9 && bit > 0 {
            if state {
                let mask = 2u8.pow(bit as u32 - 1);
                self.fspec = self.fspec | mask;
            } else {
                let mask = 0xff - 2u8.pow(bit as u32 - 1); // 1111 1111 subtract
                self.fspec = self.fspec & mask;
            }
        }
    }

    /*
     * Get subfield from bit right to left. [8..1]
     */
    pub fn get_subfield(&self, bit: u8) -> bool {
        if bit < 9 && bit > 0 {
            let mask = 2u8.pow(bit as u32 - 1);
            let result = self.fspec & mask;
            return result == 1;
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
        let mut field = FieldSpec::new();

        field.set_subfield(3, true);
        let result = field.get_subfield(3);

        assert_eq!(true, true);
    }
}