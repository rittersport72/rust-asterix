use bytes::Bytes;

use crate::asterix::header_field::Header;
use crate::asterix::record34::Record34;
use crate::category::CatError;

/// CAT34 message
#[derive(Default, Debug, PartialEq, Clone)]
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
     * Encode into CAT34 byte stream.
     */
    pub fn encode(message: &Cat34Message) -> Result<Bytes, CatError> {
        let mut message_clone = message.clone();

        let header = message_clone.get_header();
        if header.get_cat() == Cat34Message::CATEGORY {
            // Iterate over all Record34
            loop {
                let record = message_clone.remove_record34();
                if record.is_some() {
                    let record34 = record.unwrap();
                    let result = record34.encode();
                    // TODO: Append result bytes to return bytes
                } else {
                    // No more record34 returned
                    break;
                }
            }
            Ok(Bytes::default())
        } else {
            Err(CatError::CategoryInvalid)
        }
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

            if header.get_cat() == Cat34Message::CATEGORY {
                let length = header.get_len();

                // Check for correct data block length
                if length == array.len() as u16 {
                    let mut message = Cat34Message::default();
                    // TODO: Insert loop because several record34 can be in the array
                    // Create default CAT34 record
                    let mut record = Record34::default();
                    let result = record.decode(bytes);

                    match result {
                        Ok(length) => {
                            // TODO: Append record to CAT34 message
                            message.insert_record34(record);
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

    /*
     * Category.
     */
    pub const CATEGORY: u8 = 34;
}

/// CAT34 FSPEC
pub enum Cat34Fspec {
    I034_010 = 1,
    I034_000,
    I034_030,
    I034_020,
    I034_041,
    I034_050,
    I034_060,
    I034_070,
    I034_100,
    I034_110,
    I034_120,
    I034_090,
    I034RE,
    I034SP,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cat34_header() {
        let mut message = Cat34Message::default();

        let mut header = Header::default();
        header.set_cat(Cat34Message::CATEGORY);
        header.set_len(42);

        message.set_header(header);

        assert_eq!(message.get_header(), header);
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

        let mut header = Header::default();
        header.set_cat(34);

        let record34 = Record34::default();

        message.set_header(header);
        message.insert_record34(record34);

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

        let result = Cat34Message::encode(&message);
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

        let result = Cat34Message::decode(&bytes);
    }
}
