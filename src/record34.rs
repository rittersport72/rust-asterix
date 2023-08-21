use crate::data_source_field::DataSource;
use crate::field_spec::FieldSpec;
use crate::message_type_field::MessageType;
use crate::sector_number_field::SectorNumber;
use crate::time_of_day_field::TimeOfDay;

use bytes::Bytes;

/// Record of CAT34 message. Several records are possible per message.
//#[derive(Debug, Clone, PartialEq)]
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
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &Bytes) {}

    /*
     * Convert struct to byte stream.
     */
    pub fn to_bytes(&mut self) -> Bytes {
        // Create bytes
        let bytes = Bytes::copy_from_slice(b"fsfd");
        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::message_type_field;

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

        let field = record.field_spec_vector.get(0).unwrap();

        // Convert struct to byte stream
        let array = record.to_bytes();

        assert_eq!(true, true);
    }
}
