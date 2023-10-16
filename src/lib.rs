pub mod category;
pub mod asterix; // Name of subdirectory
pub mod uap; // Name of subdirectory

use bytes::Bytes;
use crate::asterix::cat34::Cat34Message;
use crate::asterix::header_field::Header;
use crate::asterix::record34::Record34;
use category::{Category, CatError};

/**
 * Encode many ASTERIX categories into byte stream
 */
pub fn encode_asterix(message: &Vec<Category>) -> Result<Bytes, CatError> {
    Err(CatError::SizeInvalid)
}

/**
 * Decode byte stream into many ASTERIX categories
 */
pub fn decode_asterix(bytes: &Bytes) -> Result<Vec<Category>, CatError> {
    Err(CatError::SizeInvalid)
}

/**
 * Encode into CAT34 byte stream
 */
pub fn encode_cat34(message: &Cat34Message) -> Result<Bytes, CatError> {
    // Check message
    //let result = check_mandatory_items(message);
    let result = Some(CatError::CategoryInvalid);

    if result.is_none() {
        // Create default CAT34 record
        let mut record = Record34::default();
        record.encode(message);
        // TODO: Use record attributes for returned bytes
    } else {
        let cat34_error = result.unwrap();
        return Err(cat34_error);
    }

    // TODO: Change it
    Err(CatError::SizeInvalid)
}

/**
 * Decode from CAT34 byte stream
 */
pub fn decode_cat34(bytes: &Bytes) -> Result<Cat34Message, CatError> {
    // Header length is 3 bytes, contains category and data block length
    if bytes.len() > 3 {
        let array: &[u8] = bytes;
        let header_array = Header::array_of_byte_message(&array[0..Header::MESSAGE_LENGTH]);

        // New message
        let mut header = Header::default();

        // Convert byte stream to struct
        header.from_bytes(&header_array);

        if header.get_cat() == Cat34Message::CATEGORY {
            let length = header.get_len();

            // Check for correct data block length
            if length == array.len() as u16 {
                // Create default CAT34 record
                let mut record = Record34::default();
                record.decode(bytes);

                // TODO: Use record attributes for Cat34Message
                return Ok(Cat34Message::default());
            }
        }
    }

    Err(CatError::SizeInvalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_encode() {
    //     let mut message = Cat34Message::new(cat34::MessageType::NorthMarker);

    //     let data_source_identifier = DataSourceIdentifier {
    //         sic: 42,
    //         sac: 26,
    //     };
    //     message.data_source_id = Some(data_source_identifier);

    //     let time = Time::from_hms(11, 22, 33).unwrap();
    //     message.time_of_day = Some(time);

    //     let position = PositionDataSource {
    //          height: 555.0,
    //          latitude: 47.8034663200378,
    //          longitude: 9.27816867828369,
    //     };
    //     message.position_data_source = Some(position);

    //     let result = encode_cat34(&message);
    // }

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

        let result = decode_cat34(&bytes);
    }
}
