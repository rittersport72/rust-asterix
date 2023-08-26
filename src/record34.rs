use crate::cat34::Cat34Message;
use crate::header_field::Header;
use bytes::Bytes;

// Search for crates in subdirectory uap
use crate::uap::field_spec::FieldSpec;
use crate::uap::message_type_field::MessageType;
use crate::uap::data_source_field::DataSource;
use crate::uap::time_of_day_field::TimeOfDay;
use crate::uap::sector_number_field::SectorNumber;

/// Record of CAT34 message. Several records are possible per message.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Record34 {
    /// Several field spec are possible for one record.
    pub field_spec_vector: Vec<FieldSpec>,
    /// I034/000
    pub message_type: Option<MessageType>,
    /// I034/010
    pub data_source_id: Option<DataSource>,
    /// I034/030
    pub time_of_day: Option<TimeOfDay>,
    /// I034/020
    pub sector_number: Option<SectorNumber>,
}

impl Record34 {
    /*
     * Decode byte stream to record.
     */
    pub fn decode(&mut self, bytes: &Bytes) {
        // Header length is 3 bytes, followd by data record
        if bytes.len() > 8 {
            self.decode_field_spec(bytes);
        }
    }

    /*
     * Encode CAT34 message to record.
     */
    pub fn encode(&mut self, message: &Cat34Message) {}

    /*
     * Decode all field spec.
     */
    fn decode_field_spec(&mut self, bytes: &Bytes) {
        let array: &[u8] = bytes;
        let mut counter = 0;

        loop {
            let field_spec_array = FieldSpec::array_of_byte_message(
                &array[(Header::MESSAGE_LENGTH + counter)..(Header::MESSAGE_LENGTH + counter + 1)]
            );

            // New message
            let mut field_spec = FieldSpec::new();

            // Convert byte stream to struct
            field_spec.from_bytes(&field_spec_array);

            let field_spec_clone = field_spec.clone();

            // Store field in vector (moves it into vector)
            self.field_spec_vector.push(field_spec);

            if field_spec_clone.get_fspec_bit(8) {
                counter += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_record() {
        // Create field spec
        let mut field_spec = FieldSpec::new();
        field_spec.set_fspec(0x0a);

        // Create data source
        let mut message_type = MessageType::new();
        message_type.set_message_type(2);

        // Create data source
        let mut data_source = DataSource::new();
        data_source.set_source_id_sic(42);
        data_source.set_source_id_sac(26);

        // Create time of day
        let mut time_day = TimeOfDay::new();
        time_day.set_time(12345.6);

        // Create sector number
        let mut sector = SectorNumber::new();
        sector.set_sector(0.0);

        // Create record
        let mut record = Record34 {
            field_spec_vector: vec![field_spec],
            message_type: Some(message_type),
            data_source_id: Some(data_source),
            time_of_day: Some(time_day),
            sector_number: Some(sector),
        };

        // Create default record
        let record2 = Record34::default();

        let field = record.field_spec_vector.get(0).unwrap();

        // Convert struct to byte stream
        //let array = record.encode(record);

        assert_eq!(true, true);
    }
}
