use std::mem;

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
     * Set bit. checkme
     */
    pub fn set_bit(&mut self, position: u8, state: bool) {
        let mask = 0xff | (state as u8) << position;
        self.fspec = self.fspec & mask;
    }

    /*
     * Get bit. checkme
     */
    pub fn get_bit(&self, position: u8) -> bool {
        let mask = 0x1 << position;
        return self.fspec & mask == 1;
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();
}
