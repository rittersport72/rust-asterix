use std::mem;

// Time of Day
//
// The attributes in structs have Network Byte Order in Big Endian
#[repr(packed(1))]
#[derive(Debug, PartialEq, Clone)]
pub struct TimeOfDay {
    time1: u8, // 1 byte
    time2: u8, // 1 byte
    time3: u8, // 1 byte
}

/*
* Implementation TimeOfDay
*/
impl TimeOfDay {
    pub fn new() -> Self {
        Self {
            time1: 0,
            time2: 0,
            time3: 0,
        }
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
     * Set time in seconds
     */
    pub fn set_time(&mut self, time: f32) {
        let converted_time: u32 = (time / Self::FACTOR) as u32;

        self.time1 = (converted_time & 0xff) as u8;
        self.time2 = ((converted_time >> 8) & 0xff) as u8;
        self.time3 = ((converted_time >> 16) & 0xff) as u8;
    }

    /*
     * Get time in seconds
     */
    pub fn get_time(&self) -> f32 {
        let converted_time: f32 =
            (self.time1 as u32 + (self.time2 as u32) << 8 + (self.time3 as u32) << 16) as f32
                * Self::FACTOR;
        converted_time
    }

    /*
     * Message length in memory.
     */
    pub const MESSAGE_LENGTH: usize = mem::size_of::<Self>();

    /*
     * Conversion factor.
     */
    const FACTOR: f32 = 1.0 / 128.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_time() {
        // Create message
        let mut time_day = TimeOfDay::new();
        time_day.set_time(3.0);

        // Convert struct to byte stream
        let array = time_day.to_bytes();

        // New message
        let mut object = TimeOfDay::new();

        // Convert byte stream to struct
        object.from_bytes(&array);

        assert_eq!(time_day.get_time(), object.get_time());
    }
}
