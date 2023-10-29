use std::mem;

// A one-octet field Data Category (CAT) indicating to which Category the data transmitted belongs
// A two-octet field Length Indicator (LEN) indicating the total length (in octets) of the Data Block, including the CAT and LEN fields
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
pub struct Header {
    cat: u8,  // 1 byte
    len: u16, // 2 bytes
}

/*
* Implementation Header
*/
impl Header {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        self.cat = array[0];
        let lo = array[1] as u16;
        let hi = (array[2] as u16) << 8;
        self.len = hi + lo;
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        let len_bytes = self.len.to_be_bytes();

        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[0] = self.cat;
        array[1] = len_bytes[1];
        array[2] = len_bytes[0];
        array
    }

    /*
     * Create fixed length array from slice.
     */
    pub fn array_of_byte_message(array: &[u8]) -> [u8; Self::MESSAGE_LENGTH] {
        array.try_into().expect("slice with incorrect length")
    }

    /*
     * Set category
     */
    pub fn set_cat(&mut self, cat: u8) {
        self.cat = cat.to_be();
    }

    /*
     * Get category
     */
    pub fn get_cat(&self) -> u8 {
        return u8::from_be(self.cat);
    }

    /*
     * Set length
     */
    pub fn set_len(&mut self, len: u16) {
        self.len = len.to_be();
    }

    /*
     * Get length
     */
    pub fn get_len(&self) -> u16 {
        return u16::from_be(self.len);
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
    fn check_header() {
        // Create message
        let mut header = Header::default();
        header.set_cat(34);
        header.set_len(1234);

        // Convert struct to byte stream
        let array = header.to_bytes();

        // New message
        let mut object = Header::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(header.get_cat(), object.get_cat());
        assert_eq!(header.get_len(), object.get_len());
    }
}
