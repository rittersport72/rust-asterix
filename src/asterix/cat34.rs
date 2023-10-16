use crate::Record34;
use time::Time;

/// CAT34 message
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cat34Message {
    /// Several records are possible per message.
    record: Vec<Record34>,
}

/// Implementation CAT34 message
impl Cat34Message {
    /*
     * Category.
     */
    pub const CATEGORY: u8 = 34;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat34() {
        let mut message = Cat34Message::new(MessageType::NorthMarker);

        let data_source_identifier = DataSourceIdentifier {
            sic: 42,
            sac: 26,
        };

        // message.data_source_id = Some(data_source_identifier);
        // println!("{:?}", message);

        // assert_eq!(message.message_type, MessageType::NorthMarker);

    }
}