use std::mem;

/// Types of messages
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MessageTypeEnum {
    Unknown,
    NorthMarker,
    SectorCrossing,
    GeographicalFiltering,
    JammingStrobe,
    SolarStorm,
    SSRJammingStrobe,
    ModeSJammingStrobe,
}

// Message Type
//
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct MessageType {
    message: u8, // 1 byte
}

/*
* Implementation MessageType
*/
impl MessageType {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &[u8; Self::MESSAGE_LENGTH]) {
        self.message = array[0];
    }

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&self) -> [u8; Self::MESSAGE_LENGTH] {
        let mut array = [0u8; Self::MESSAGE_LENGTH];
        array[0] = self.message;
        array
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
    pub fn set_message_type(&mut self, message_type: MessageTypeEnum) {
        let value = message_type as u8;
        self.message = value.to_be();
    }

    /*
     * Get message type
     */
    pub fn get_message_type(&self) -> MessageTypeEnum {
        let value = u8::from_be(self.message);

        match value {
            1 => MessageTypeEnum::NorthMarker,
            2 => MessageTypeEnum::SectorCrossing,
            3 => MessageTypeEnum::GeographicalFiltering,
            4 => MessageTypeEnum::JammingStrobe,
            5 => MessageTypeEnum::SolarStorm,
            6 => MessageTypeEnum::SSRJammingStrobe,
            7 => MessageTypeEnum::ModeSJammingStrobe,
            _ => MessageTypeEnum::Unknown,
        }
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
        let mut message_type = MessageType::default();
        message_type.set_message_type(MessageTypeEnum::JammingStrobe);

        // Convert struct to byte stream
        let array = message_type.to_bytes();

        // New message
        let mut object = MessageType::default();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(message_type.get_message_type(), object.get_message_type());
    }
}
