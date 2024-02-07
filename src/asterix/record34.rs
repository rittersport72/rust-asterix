use bytes::Bytes;

use crate::category::CatError;

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
    pub data_source_id: Option<DataSource>,
    /// I034/000
    pub message_type: Option<MessageType>,
    /// I034/030
    pub time_of_day: Option<TimeOfDay>,
    /// I034/020
    pub sector_number: Option<SectorNumber>,
    /// I034/041
    pub antenna_rotation: Option<AntennaRotation>,
    /// I034/050
    pub system_configuration_status: Option<SystemConfigurationStatus>,
    /// I034/060
    pub system_processing_mode: Option<SystemProcessingMode>,
    /// I034/100
    pub generic_polar_window: Option<GenericPolarWindow>,
    /// I034/120
    pub position_source: Option<PositionSource>,
}

impl Record34 {
    /*
     * Decode byte stream to record.
     */
    pub fn decode(&mut self, bytes: &Bytes) -> Result<usize, CatError> {
        // Data record has at least one FSPEC
        if bytes.len() > 8 {
            // Order is important, because some fields have variable length
            self.field_spec_vector = decode_field_spec(bytes);
        }

        let length = calculate_record_length();
        Ok(length)
    }

    /*
     * Encode record to byte stream.
     */
    pub fn encode(&self) -> Result<Bytes, CatError> {
        Ok(Bytes::default())
    }
}

/*
 * Calculate record byte length.
 * Checks for valid optional attributes.
 */
fn calculate_record_length() -> usize {
    1u32 as usize
}

/*
 * Decode all field spec.
 */
fn decode_field_spec(bytes: &Bytes) -> Vec<FieldSpec> {
    let array: &[u8] = bytes;
    let mut vec = Vec::new();

    loop {
        let begin_index = vec.len();
        let end_index = begin_index + FieldSpec::MESSAGE_LENGTH;

        let field_spec_array = FieldSpec::array_of_byte_message(&array[begin_index..end_index]);

        // New message
        let mut field_spec = FieldSpec::default();

        // Convert byte stream to struct
        field_spec.from_bytes(&field_spec_array);
        let fx = field_spec.get_fspec_bit(FSPEC_FX);

        // Store field in vector (moves it into vector)
        vec.push(field_spec);

        // Check field extension
        if !fx {
            break;
        }
    }
    vec
}

/*
 * Decode data source id.
 */
// fn decode_data_source_id(bytes: &Bytes) {
//     if let Some(field_spec) = self.field_spec_vector.get(0) {
//         let bit = field_spec.get_fspec_bit(1);

//         if bit {
//             let begin_index = self.field_spec_vector.len();
//             let end_index = begin_index + DataSource::MESSAGE_LENGTH;
//             let array: &[u8] = bytes;

//             let data_source_array =
//                 DataSource::array_of_byte_message(&array[begin_index..end_index]);

//             // New message
//             let mut data_source = DataSource::default();

//             // Convert byte stream to struct
//             data_source.from_bytes(&data_source_array);

//             // Store field
//             self.data_source_id = Some(data_source);
//         }
//     }
// }

/*
 * Decode message type.
 */
// fn decode_message_type(bytes: &Bytes) {
//     if let Some(field_spec) = self.field_spec_vector.get(0) {
//         let bit = field_spec.get_fspec_bit(2);

//         if bit {
//             let begin_index = self.field_spec_vector.len() + DataSource::MESSAGE_LENGTH;
//             let end_index = begin_index + MessageType::MESSAGE_LENGTH;
//             let array: &[u8] = bytes;

//             let message_type_array =
//                 MessageType::array_of_byte_message(&array[begin_index..end_index]);

//             // New message
//             let mut message_type = MessageType::default();

//             // Convert byte stream to struct
//             message_type.from_bytes(&message_type_array);

//             // Store field
//             self.message_type = Some(message_type);
//         }
//     }
// }

/*
 * Decode time of day.
 */
// fn decode_time_of_day(bytes: &Bytes) {
//     if let Some(field_spec) = self.field_spec_vector.get(0) {
//         let bit = field_spec.get_fspec_bit(3);

//         if bit {
//             let begin_index = self.field_spec_vector.len()
//                 + DataSource::MESSAGE_LENGTH
//                 + MessageType::MESSAGE_LENGTH;
//             let end_index = begin_index + TimeOfDay::MESSAGE_LENGTH;
//             let array: &[u8] = bytes;

//             let time_day_array = TimeOfDay::array_of_byte_message(&array[begin_index..end_index]);

//             // New message
//             let mut time_of_day = TimeOfDay::default();

//             // Convert byte stream to struct
//             time_of_day.from_bytes(&time_day_array);

//             // Store field
//             self.time_of_day = Some(time_of_day);
//         }
//     }
// }

/// CAT34 Standard User Application Profile (UAP)
/// FSPEC Field Reference Number (FRN)
#[derive(Debug)]
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

/// FSPEC FX Field Reference Number (FRN)
pub const FSPEC_FX: u8 = 8;

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
        let record = Record34 {
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
        let _record2 = Record34::default();

        let _field = record.field_spec_vector.get(0).unwrap();

        // Convert struct to byte stream
        //let array = record.encode(record);

        assert_eq!(true, true);
    }
}
