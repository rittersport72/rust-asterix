pub mod asterix; // Name of subdirectory
pub mod category;
pub mod uap; // Name of subdirectory

use crate::asterix::cat34::{Cat34Message, encode, decode};
use crate::asterix::header_field::Header;
use bytes::Bytes;
use category::{CatError, Category};

/**
 * Encode many ASTERIX categories into byte stream
 */
pub fn encode_asterix(messages: &Vec<Category>) -> Result<Bytes, CatError> {
    let mut sum_bytes = Bytes::default();

    for category in messages.iter() {
        let result;

        match category {
            Category::Cat034(cat34) => result = encode(cat34),
            _ => result = Err(CatError::CategoryInvalid),
        }
        // Append each byte stream to one big byte stream when OK result
        if result.is_ok() {
            let bytes = result.unwrap();
            // TODO Append to byte array
            sum_bytes = bytes;
        } else {
            return result;
        }
    }
    Ok(sum_bytes)
}

/**
 * Decode byte stream into many ASTERIX categories
 */
pub fn decode_asterix(bytes: &Bytes) -> Result<Vec<Category>, CatError> {
    // Header length is 3 bytes, contains category and data block length
    if bytes.len() > 3 {
        let array: &[u8] = bytes;
        let header_array = Header::array_of_byte_message(&array[0..Header::MESSAGE_LENGTH]);

        // New message
        let mut header = Header::default();

        // Convert byte stream to struct
        header.from_bytes(&header_array);
        let length = header.get_len() as usize;

        // Collect decoded messages
        let mut vector:Vec<Category> = Vec::new();

        // TODO Process remaining bytes of next message
        // Check for correct data block length
        if length <= bytes.len() {
            if header.get_cat() == Cat34Message::CATEGORY {
                let result = decode(bytes);

                if result.is_ok() {
                    let message = result.unwrap();
                    let cat = Category::Cat034(message);
                    vector.push(cat);
                    return Ok(vector);
                } else {
                    let error = result.err().unwrap();
                    return Err(error);
                }
            } else {
                return Err(CatError::CategoryInvalid);
            }
        }
    }
    Err(CatError::SizeInvalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asterix::header_field::Header;
    use crate::asterix::record34::Record34;

    #[test]
    fn test_encode() {
        let mut header = Header::default();
        header.set_cat(Cat34Message::CATEGORY);

        let record34 = Record34::default();

        let mut message34 = Cat34Message::default();
        message34.set_header(header);
        message34.insert_record34(record34);

        let cat = Category::Cat034(message34);

        let messages = vec![cat];

        let result = encode_asterix(&messages);

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_decode() {
        let _array: &'static [u8] = &[
            0x22, 0x00, 0x10, 0xf6, 0x19, 0x0e, 0x02, 0x3a, 0x69, 0x2b, 0x40, 0x88, 0x40, 0x40,
            0x80, 0x00,
        ];
        // North Marker message, length 23
        // WGS84 Height 555 m
        // WGS84 Latitude 47.8034663200378 deg
        // WGS84 Longitude 9.27816867828369 deg
        let array: &'static [u8] = &[
            0x22, 0x00, 0x17, 0xed, 0x10, 0x7b, 0x2a, 0x01, 0x4e, 0x51, 0x7b, 0x01, 0x00, 0x80,
            0x00, 0x02, 0x2b, 0x21, 0xfe, 0x5b, 0x06, 0x99, 0x0a,
        ];
        let bytes = Bytes::from(array);

        let result = decode_asterix(&bytes);

        assert_eq!(result.is_err(), false);
    }
}
