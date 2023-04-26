pub mod cat34;
pub mod cat_error;
pub mod field_spec;
pub mod header_field;
pub mod tools;

use bytes::{Bytes, Buf};
use cat34::Cat34Message;
use cat_error::Cat34Error;
use header_field::Header;
use field_spec::FieldSpec;

/**
 * Encode into CAT34 byte stream
 */
fn encode_cat34(message: &Cat34Message) -> Result<Bytes, Cat34Error> {
    // Create message
    let mut header = Header::new();
    header.set_cat(34);
    header.set_len(1234);
    
    // Convert struct to byte stream
    let header_array = header.to_bytes();

    // Create message
    let mut fspec = FieldSpec::new();
    fspec.set_fspec(0x0a);

    // Convert struct to byte stream
    let fspec_array = fspec.to_bytes();




    let array: &'static [u8] = &[1, 2, 3, 4, 5];
    let bytes = Bytes::from(array);

    Ok(bytes)
 //   Err(Cat34Error::I034AllValid)
}

/**
 * Decode from CAT34 byte stream
 */
fn decode_cat34(bytes: &Bytes) -> Result<Cat34Message, Cat34Error> {
    let array: &'static [u8] = &[1, 2, 3, 4, 5];
    //let array: &[u8] = bytes;

    // Header length is 3 bytes, contains category and data block length
    if array.len() > 3 {
        let a = Header::array_of_byte_message(&array[0..Header::MESSAGE_LENGTH]);

        // New message
        let mut object = Header::new();

        // Convert byte stream to struct
        object.from_bytes(&a);

        return Ok(Cat34Message::new(cat34::MessageType::NorthMarker));
    }

    Err(Cat34Error::I034_000Invalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat34::Cat34Message;

    #[test]
    fn test_encode() {
        let mut message = Cat34Message::new(cat34::MessageType::NorthMarker);

        let result = encode_cat34(&message);
    }

    #[test]
    fn test_decode() {
        let array: &'static [u8] = &[0x22, 0x00, 0x10, 0xf6, 0x19, 0x0e, 0x02, 0x3a, 0x69, 0x2b, 0x40, 0x88, 0x40, 0x40, 0x80, 0x00];
        let bytes = Bytes::from(array);

        let result = decode_cat34(&bytes);
    }
}
