use std::mem;

// Message Type
//
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
//#[derive(Debug, PartialEq)]
pub struct MessageType {
    message: u8, // 1 byte
}

/*
* Implementation MessageType
*/
impl MessageType {
    pub fn new() -> Self {
        Self { message: 0 }
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
     * Set message type
     */
    pub fn set_message_type(&mut self, message_type: u8) {
        self.message = message_type.to_be();
    }

    /*
     * Get message type
     */
    pub fn get_message_type(&self) -> u8 {
        u8::from_be(self.message)
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
    fn check_message_type() {
        // Create message
        let mut message_type = MessageType::new();
        message_type.set_message_type(2);

        // Convert struct to byte stream
        let array = message_type.to_bytes();

        // New message
        let mut object = MessageType::new();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(message_type.get_message_type(), object.get_message_type());
    }
}
