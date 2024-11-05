use bytes::{BufMut, BytesMut};
use std::io::Read;
use bytes::Bytes;
use crate::asterix::header_field::Header;

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
        // Data record has at least two FSPEC
        if bytes.len() > 2 {
            // Order is important, because some fields have variable length
            self.field_spec_vector = decode_field_spec(bytes);
        }

        // How many bytes are in FSPEC
        let len = self.field_spec_vector.len();

        if bytes.len() > len {
            let record_bytes = bytes.slice(len..);

        }

        let length = calculate_record_length();
        Ok(length)
    }

    /*
     * Encode record to byte stream.
     */
    pub fn encode(&mut self) -> Result<Bytes, CatError> {
        // New fspec field
        let mut field_spec1 = FieldSpec::default();
        let mut field_spec2 = FieldSpec::default();

        let mut bytes_length = 0;
        let mut vector:Vec<BytesMut> = Vec::new();

        // First FSPEC
        if self.data_source_id.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_010 as u8);
            let bytes = self.data_source_id.unwrap().to_bytes();

            bytes_length += bytes.len();
            let mut bb = BytesMut::with_capacity(bytes.len());
            //let mut bb = BytesMut::new();
            //bb.put_slice(&bytes);
            //bb.extend(bytes);
            //vector.push(bb);

            bb.put(&bytes[..]);

            vector.push(bb);
        }
        if self.message_type.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_000 as u8);
            let bytes = self.message_type.unwrap().to_bytes();
        }
        if self.time_of_day.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_030 as u8);
            let bytes = self.time_of_day.unwrap().to_bytes();
        }
        if self.sector_number.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_020 as u8);
            let bytes = self.sector_number.unwrap().to_bytes();
        }
        if self.antenna_rotation.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_041 as u8);
            let bytes = self.antenna_rotation.unwrap().to_bytes();
        }
        if self.system_configuration_status.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_050 as u8);
            let bytes = self.system_configuration_status.unwrap().to_bytes();
        }
        if self.system_processing_mode.is_some() {
            field_spec1.set_fspec_bit(Cat34Fspec::I034_060 as u8);
            let bytes = self.system_processing_mode.unwrap().to_bytes();
        }
        // Second FSPEC
        if self.generic_polar_window.is_some() {
            field_spec1.set_fspec_bit(FSPEC_FX);
            field_spec2.set_fspec_bit(Cat34Fspec::I034_100 as u8 - FSPEC_FX);
            let bytes = self.generic_polar_window.unwrap().to_bytes();
        }
        if self.position_source.is_some() {
            field_spec1.set_fspec_bit(FSPEC_FX);
            field_spec2.set_fspec_bit(Cat34Fspec::I034_120 as u8 - FSPEC_FX);
            let bytes = self.position_source.unwrap().to_bytes();
        }
        // CAT34 has up to two FSPEC
        self.field_spec_vector.push(field_spec1);
        self.field_spec_vector.push(field_spec2);

        let fspec1_bytes = field_spec1.to_bytes();
        let fspec2_bytes = field_spec2.to_bytes();

        let mut sum_bytes = BytesMut::with_capacity(self.field_spec_vector.len() + bytes_length);
        sum_bytes.put(&fspec1_bytes[..]);
        sum_bytes.put(&fspec2_bytes[..]);

        for bytes in vector {
            sum_bytes.put(bytes);
        }

        Ok(sum_bytes.into())
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
