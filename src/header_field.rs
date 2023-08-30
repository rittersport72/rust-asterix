use std::mem;

// A one-octet field Data Category (CAT) indicating to which Category the data transmitted belongs
// A two-octet field Length Indicator (LEN) indicating the total length (in octets) of the Data Block, including the CAT and LEN fields
//
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    cat: u8,  // 1 byte
    len: u16, // 2 bytes
}

/*
* Implementation Header
*/
impl Header {
    pub fn new() -> Self {
        Self { cat: 0, len: 0 }
    }

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
        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[0] = self.cat;
        array[1] = (self.len & 0x00ff) as u8;
        array[2] = ((self.len & 0xff00) >> 8) as u8;
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
        let mut header = Header::new();
        header.set_cat(34);
        header.set_len(1234);

        // Convert struct to byte stream
        let array = header.to_bytes();

        // New message
        let mut object = Header::new();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(header.get_cat(), object.get_cat());
        assert_eq!(header.get_len(), object.get_len());
    }
}
