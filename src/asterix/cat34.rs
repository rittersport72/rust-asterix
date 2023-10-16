use crate::Header;
use crate::Record34;

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
     * Category.
     */
    pub const CATEGORY: u8 = 34;
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
}