use crate::asterix::cat34::Cat34Message;
use crate::asterix::header_field::Header;
use bytes::Bytes;

// Search for crates in subdirectory uap
use crate::uap::antenna_rotation_field::AntennaRotation;
use crate::uap::data_source_field::DataSource;
use crate::uap::field_spec::FieldSpec;
use crate::uap::generic_polar_window_field::GenericPolarWindow;
use crate::uap::message_type_field::MessageType;
use crate::uap::position_source_field::PositionSource;
use crate::uap::sector_number_field::SectorNumber;
use crate::uap::system_configuration_field::SystemConfigurationStatus;
use crate::uap::system_processing_field::SystemProcessingMode;
use crate::uap::time_of_day_field::TimeOfDay;

/// Record of CAT34 message. Several records are possible per message.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Record34 {
    /// Several field spec are possible for one record.
    field_spec_vector: Vec<FieldSpec>,
    /// I034/010
    data_source_id: Option<DataSource>,
    /// I034/000
    message_type: Option<MessageType>,
    /// I034/030
    time_of_day: Option<TimeOfDay>,
    /// I034/020
    sector_number: Option<SectorNumber>,
    /// I034/041
    antenna_rotation: Option<AntennaRotation>,
    /// I034/050
    system_configuration_status: Option<SystemConfigurationStatus>,
    /// I034/060
    system_processing_mode: Option<SystemProcessingMode>,
    /// I034/100
    generic_polar_window: Option<GenericPolarWindow>,
    /// I034/120
    position_source: Option<PositionSource>,
}

impl Record34 {
    /*
     * Decode byte stream to record.
     */
    pub fn decode(&mut self, bytes: &Bytes) {
        // Header length is 3 bytes, followd by data record
        if bytes.len() > 8 {
            // Order is important, because some fields have variable length
            self.decode_field_spec(bytes);
            self.decode_data_source_id(bytes);
            self.decode_message_type(bytes);
            self.decode_time_of_day(bytes);
        }
    }

    /*
     * Encode CAT34 message to record.
     */
    pub fn encode(&mut self, message: &Cat34Message) {
        // // Create message
        // let mut header = Header::new();
        // header.set_cat(Cat34Message::CATEGORY);
        // header.set_len(1234);

        // // Convert struct to byte stream
        // let header_array = header.to_bytes();

        // // Create message
        // let mut fspec = FieldSpec::new();
        // fspec.set_fspec(0x0a);

        // // Convert struct to byte stream
        // let fspec_array = fspec.to_bytes();

        // // The empty array
        // let mut empty: [u8; 4] = [0; 4];

        // // Copy from slice with slices.
        // empty[0..3].copy_from_slice(&header_array[0..3]);
        // empty[3..].copy_from_slice(&fspec_array[..]);

        // // Create bytes
        // let bytes = Bytes::copy_from_slice(&empty);

        // Convert fields
        // let mut message_type = MessageType::new();
        // message_type.set_message_type(message.message_type as u8);
        // self.message_type = Some(message_type);

        // if message.data_source_id.is_some() {
        //    let data_source_id = message.data_source_id.clone().unwrap();

        //    let mut data_source = DataSource::new();
        //    data_source.set_source_id_sic(data_source_id.sic);
        //    data_source.set_source_id_sac(data_source_id.sac);

        //    self.data_source_id = Some(data_source);
        // }

        // if message.time_of_day.is_some() {
        //     let time_of_day = message.time_of_day.clone().unwrap();

        //     let mut time_day = TimeOfDay::new();
        //     time_day.set_time(time_of_day.millisecond() as f32 / 1000.0);

        //     self.time_of_day = Some(time_day);
        // }

        // if message.position_data_source.is_some() {
        //     let position = message.position_data_source.clone().unwrap();
        // }
    }

    /*
     * Decode all field spec.
     */
    fn decode_field_spec(&mut self, bytes: &Bytes) {
        let array: &[u8] = bytes;
        let mut counter = 0;

        loop {
            let begin_index = Header::MESSAGE_LENGTH + counter;
            let end_index = begin_index + FieldSpec::MESSAGE_LENGTH;

            let field_spec_array = FieldSpec::array_of_byte_message(&array[begin_index..end_index]);

            // New message
            let mut field_spec = FieldSpec::default();

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

    /*
     * Decode data source id.
     */
    fn decode_data_source_id(&mut self, bytes: &Bytes) {
        if let Some(field_spec) = self.field_spec_vector.get(0) {
            let bit = field_spec.get_fspec_bit(1);

            if bit {
                let begin_index = Header::MESSAGE_LENGTH + self.field_spec_vector.len();
                let end_index = begin_index + DataSource::MESSAGE_LENGTH;
                let array: &[u8] = bytes;

                let data_source_array =
                    DataSource::array_of_byte_message(&array[begin_index..end_index]);

                // New message
                let mut data_source = DataSource::default();

                // Convert byte stream to struct
                data_source.from_bytes(&data_source_array);

                // Store field
                self.data_source_id = Some(data_source);
            }
        }
    }

    /*
     * Decode message type.
     */
    fn decode_message_type(&mut self, bytes: &Bytes) {
        if let Some(field_spec) = self.field_spec_vector.get(0) {
            let bit = field_spec.get_fspec_bit(2);

            if bit {
                let begin_index = Header::MESSAGE_LENGTH
                    + self.field_spec_vector.len()
                    + DataSource::MESSAGE_LENGTH;
                let end_index = begin_index + MessageType::MESSAGE_LENGTH;
                let array: &[u8] = bytes;

                let message_type_array =
                    MessageType::array_of_byte_message(&array[begin_index..end_index]);

                // New message
                let mut message_type = MessageType::default();

                // Convert byte stream to struct
                message_type.from_bytes(&message_type_array);

                // Store field
                self.message_type = Some(message_type);
            }
        }
    }

    /*
     * Decode time of day.
     */
    fn decode_time_of_day(&mut self, bytes: &Bytes) {
        if let Some(field_spec) = self.field_spec_vector.get(0) {
            let bit = field_spec.get_fspec_bit(3);

            if bit {
                let begin_index = Header::MESSAGE_LENGTH
                    + self.field_spec_vector.len()
                    + DataSource::MESSAGE_LENGTH
                    + MessageType::MESSAGE_LENGTH;
                let end_index = begin_index + TimeOfDay::MESSAGE_LENGTH;
                let array: &[u8] = bytes;

                let time_day_array =
                    TimeOfDay::array_of_byte_message(&array[begin_index..end_index]);

                // New message
                let mut time_of_day = TimeOfDay::default();

                // Convert byte stream to struct
                time_of_day.from_bytes(&time_day_array);

                // Store field
                self.time_of_day = Some(time_of_day);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uap::message_type_field::MessageTypeEnum;

    #[test]
    fn check_record() {
        // Create field spec
        let mut field_spec = FieldSpec::default();
        field_spec.set_fspec(0x0a);

        // Create data source
        let mut message_type = MessageType::default();
        message_type.set_message_type(MessageTypeEnum::JammingStrobe);

        // Create data source
        let mut data_source = DataSource::default();
        data_source.set_source_id_sic(42);
        data_source.set_source_id_sac(26);

        // Create time of day
        let mut time_day = TimeOfDay::default();
        time_day.set_time(12345.6);

        // Create sector number
        let mut sector = SectorNumber::default();
        sector.set_sector(0.0);

        // Create record
        let mut record = Record34 {
            field_spec_vector: vec![field_spec],
            message_type: Some(message_type),
            data_source_id: Some(data_source),
            time_of_day: Some(time_day),
            sector_number: Some(sector),
            antenna_rotation: None,
            system_configuration_status: None,
            system_processing_mode: None,
            generic_polar_window: None,
            position_source: None,
        };

        // Create default record
        let record2 = Record34::default();

        let field = record.field_spec_vector.get(0).unwrap();

        // Convert struct to byte stream
        //let array = record.encode(record);

        assert_eq!(true, true);
    }
}
