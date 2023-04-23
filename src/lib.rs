pub mod cat34;
pub mod cat_error;
pub mod field_spec;
pub mod header_field;
pub mod tools;

use bytes::{Bytes, Buf};
use cat34::Cat34Message;
use cat_error::Cat34Error;

/**
 * Encode into CAT34 byte stream
 */
fn encode(message: &Cat34Message) -> Result<Bytes, Cat34Error> {

    let array: &'static [u8] = &[1, 2, 3, 4, 5];
    let bytes = Bytes::from(array);


    let b = Bytes::from_static(array);

    Ok(bytes)
 //   Err(Cat34Error::I034AllValid)
}

/**
 * Decode from CAT34 byte stream
 */
fn decode(bytes: Bytes) -> Result<Cat34Message, Cat34Error> {
    // Header length is 3 bytes, contains category and data block length
    if bytes.len() > 3 {
        
        return Ok(Cat34Message::new(cat34::MessageType::NorthMarker));
    }

    Err(Cat34Error::I034_000Invalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat34::Cat34Message;

    #[test]
    fn cat34() {
        let message = Cat34Message::new(cat34::MessageType::NorthMarker);

        println!("{:?}", message);
    }
}
