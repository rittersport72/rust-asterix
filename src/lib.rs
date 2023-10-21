pub mod asterix; // Name of subdirectory
pub mod category;
pub mod uap; // Name of subdirectory

use crate::asterix::cat34::Cat34Message;
use bytes::Bytes;
use category::{CatError, Category};

/**
 * Encode many ASTERIX categories into byte stream
 */
pub fn encode_asterix(messages: &Vec<Category>) -> Result<Bytes, CatError> {
    let mut result = Err(CatError::CategoryInvalid);

    for category in messages.iter() {
        match category {
            Category::Cat034(cat34) => result = Cat34Message::encode(cat34),
            _ => result = Err(CatError::CategoryInvalid),
        }
        // TODO: Append each byte stream to one big byte stream when OK result
    }
    result
}

/**
 * Decode byte stream into many ASTERIX categories
 */
pub fn decode_asterix(bytes: &Bytes) -> Result<Vec<Category>, CatError> {
    // TODO: Check byte stream header and match acccording to category
    let result = Cat34Message::decode(bytes);

    // TODO: Append each message to vector when OK result
    Err(CatError::SizeInvalid)
}
