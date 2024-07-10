use bytes::{BufMut, Bytes, BytesMut};

use crate::asterix::header_field::Header;
use crate::asterix::record34::Record34;
use crate::category::{Category, CatError};

/// CAT34 message
#[derive(Debug, PartialEq, Clone)]
pub struct Cat34Message {
    /// Header contains category and message lenghs
    header: Header,
    /// Several records are possible per message.
    record: Vec<Record34>,
}

/// Implementation CAT34 message
impl Cat34Message {
    /*
     * Insert Record34 into vector.
     */
    pub fn insert_record34(&mut self, record34: Record34) {
        self.record.push(record34);
    }

    /*
     * Remove Record34 from vector.
     */
    pub fn remove_record34(&mut self) -> Option<Record34> {
        self.record.pop()
    }

    pub fn get_header(&self) -> Header {
        self.header
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    /*
     * Category.
     */
    pub const CATEGORY: u8 = 34;
}

impl Default for Cat34Message {
    fn default() -> Self {
        let mut message = Self {
            header: Header::default(),
            record: Vec::new(),
        };
        message.header.set_cat(Cat34Message::CATEGORY);

        message
    }
}

impl TryFrom<Category> for Cat34Message {
    type Error = ();

    fn try_from(value: Category) -> Result<Self, Self::Error> {
        match value {
            Category::Cat007 => Err(()),
            Category::Cat034(cat) => Ok(cat),
            Category::Cat048 => Err(()),
            Category::Cat062 => Err(()),
        }
    }
}

/*
 * Encode into CAT34 byte stream.
 */
pub fn encode(message: &Cat34Message) -> Result<Bytes, CatError> {
    let mut message_clone = message.clone();

    let mut bytes_length = 0;
    let mut vector:Vec<Bytes> = Vec::new();

    // Iterate over all Record34
    loop {
        let record = message_clone.remove_record34();
        if record.is_some() {
            let mut record34 = record.unwrap();
            let result = record34.encode();

            if result.is_ok() {
                let bytes = result.unwrap();
                bytes_length += bytes.len();
                // Append result bytes to return bytes
                vector.push(bytes);
            } else {
                return result;
            }
        } else {
            // No more record34 returned
            break;
        }
    }

    // Calculate length
    message_clone.header.set_len((Header::MESSAGE_LENGTH + bytes_length) as u16);
    let header = message_clone.header.to_bytes();

    let mut sum_bytes = BytesMut::with_capacity(Header::MESSAGE_LENGTH + bytes_length);
    sum_bytes.put(&header[..]);

    for bytes in vector {
        sum_bytes.put(bytes);
    }

    Ok(sum_bytes.into())
}

/*
 * Decode from CAT34 byte stream
 */
pub fn decode(bytes: &Bytes) -> Result<Cat34Message, CatError> {
    // Header length is 3 bytes, contains category and data block length
    if bytes.len() > 3 {
        let array: &[u8] = bytes;
        let header_array = Header::array_of_byte_message(&array[0..Header::MESSAGE_LENGTH]);

        // New message
        let mut header = Header::default();

        // Convert byte stream to struct
        header.from_bytes(&header_array);
        let length = header.get_len() as usize;

        // Check for correct data block length
        if length == bytes.len() {
            if header.get_cat() == Cat34Message::CATEGORY {
                let mut offset = Header::MESSAGE_LENGTH;
                let mut message = Cat34Message::default();
                // TODO: Insert loop because several record34 can be in the array
                // Create default CAT34 record
                let record_bytes = bytes.slice(offset..);
                let mut record = Record34::default();
                let result = record.decode(&record_bytes);

                match result {
                    Ok(record_length) => {
                        offset += record_length;
                        // Append record to CAT34 message
                        message.insert_record34(record);

                        // Check offset with byte length
                        if offset > length {
                            return Err(CatError::SizeInvalid);
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }

                return Ok(message);
            }
        }
    }

    Err(CatError::SizeInvalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cat34_header() {
        let mut message = Cat34Message::default();

        let mut header = Header::default();
        header.set_cat(26);
        header.set_len(42);

        message.set_header(header);

        assert_eq!(message.get_header().get_cat(), 26);
        assert_eq!(message.get_header().get_len(), 42);
    }

    #[test]
    fn check_cat34_record() {
        let mut message = Cat34Message::default();

        let first_record = Record34::default();
        let second_record = Record34::default();

        message.insert_record34(first_record);
        message.insert_record34(second_record);

        assert_eq!(message.remove_record34(), Some(Record34::default()));
        assert_eq!(message.remove_record34(), Some(Record34::default()));
        assert_eq!(message.remove_record34(), None);
    }

    #[test]
    fn test_encode() {
        let mut message = Cat34Message::default();

        let record34 = Record34::default();
        message.insert_record34(record34);

        let _result = encode(&message);
    }

    #[test]
    fn test_decode() {
        // North Marker message, length 23
        // WGS84 Height 555 m
        // WGS84 Latitude 47.8034663200378 deg
        // WGS84 Longitude 9.27816867828369 deg
        let array: &'static [u8] = &[
            0x22, 0x00, 0x17, 0xed, 0x10, 0x7b, 0x2a, 0x01, 0x4e, 0x51, 0x7b, 0x01, 0x00, 0x80,
            0x00, 0x02, 0x2b, 0x21, 0xfe, 0x5b, 0x06, 0x99, 0x0a,
        ];
        let bytes = Bytes::from(array);

        let _result = decode(&bytes);
    }

    #[test]
    fn test_try_from() {
        let cat_enum = Category::Cat034(Cat34Message::default());
        let cat: Cat34Message = cat_enum.try_into().unwrap();

        assert_eq!(cat, Cat34Message::default());
    }
}
