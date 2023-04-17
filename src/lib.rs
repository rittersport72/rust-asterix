pub mod cat34;
pub mod cat_error;
pub mod field_spec;
pub mod tools;

use bytes::Bytes;
use cat34::Cat34Message;
use cat_error::Cat34Error;

/**
 * Encode into CAT34 byte stream
 */
fn encode(message: &Cat34Message) -> Result<Bytes, Cat34Error> {

    let bytes = Bytes::from(vec![1, 2, 3]);

    Ok(bytes)
 //   Err(Cat34Error::I034AllValid)
}

/**
 * Decode from CAT34 byte stream
 */
fn decode(bytes: Bytes) -> Result<Cat34Message, Cat34Error> {
    if bytes.len() > 0 {
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
