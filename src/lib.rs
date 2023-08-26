pub mod cat34;
pub mod cat_error;
pub mod header_field;
pub mod record34;
pub mod tools;
pub mod uap; // Name of subdirectory

use bytes::{Buf, Bytes};
use cat34::{Cat34Message, DataSourceIdentifier};
use time::Time;
use cat_error::Cat34Error;
use header_field::Header;
use record34::Record34;
// Search for crates in subdirectory uap
use crate::uap::field_spec::FieldSpec;

/**
 * Encode into CAT34 byte stream
 */
pub fn encode_cat34(message: &Cat34Message) -> Result<Bytes, Cat34Error> {
    // Create message
    let mut header = Header::new();
    header.set_cat(Cat34Message::CATEGORY);
    header.set_len(1234);

    // Convert struct to byte stream
    let header_array = header.to_bytes();

    // Create message
    let mut fspec = FieldSpec::new();
    fspec.set_fspec(0x0a);

    // Convert struct to byte stream
    let fspec_array = fspec.to_bytes();

    // The empty array
    let mut empty: [u8; 4] = [0; 4];

    // Copy from slice with slices.
    empty[0..3].copy_from_slice(&header_array[0..3]);
    empty[3..].copy_from_slice(&fspec_array[..]);

    // Create bytes
    let bytes = Bytes::copy_from_slice(&empty);

    Ok(bytes)
    //   Err(Cat34Error::I034AllValid)
}

/**
 * Decode from CAT34 byte stream
 */
pub fn decode_cat34(bytes: &Bytes) -> Result<Cat34Message, Cat34Error> {
    // Header length is 3 bytes, contains category and data block length
    if bytes.len() > 3 {
        let array: &[u8] = bytes;
        let header_array = Header::array_of_byte_message(&array[0..Header::MESSAGE_LENGTH]);

        // New message
        let mut header = Header::new();

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
                return Ok(Cat34Message::new(cat34::MessageType::NorthMarker));
            }
        }
    }

    Err(Cat34Error::I034SizeInvalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let mut message = Cat34Message::new(cat34::MessageType::NorthMarker);

        let data_source_identifier = DataSourceIdentifier {
            sic: Some(42),
            sac: Some(26),
        };
        message.data_source_id = Some(data_source_identifier);

        let time = Time::from_hms(11, 22, 33).unwrap();
        message.time_of_day = Some(time);

        let result = encode_cat34(&message);
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

        let result = decode_cat34(&bytes);
    }
}
