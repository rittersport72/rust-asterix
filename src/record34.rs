use crate::field_spec::FieldSpec;
use crate::data_source_field::DataSource;

use bytes::Bytes;

/// Record of CAT34 message. Several records are possible per message.
//#[derive(Debug, Clone, PartialEq)]
pub struct Record34 {
    /// Several field spec are possible for one record.
    pub field_spec_vector: Vec<FieldSpec>,
    /// I034/010
    pub data_source_id: Option<DataSource>,
}

impl Record34 {
    /*
     * Convert byte stream to struct.
     */
    pub fn from_bytes(&mut self, array: &Bytes) {

    }

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
    use super::*;

    #[test]
    fn check_record() {
        // Create field spec
        let mut field_spec = FieldSpec::new();
        field_spec.set_fspec(0x0a);

        // Create data source
        let mut data_source = DataSource::new();
        data_source.set_source_id_sic(42);
        data_source.set_source_id_sac(26);

        // Create record
        let mut record = Record34 {
            field_spec_vector: vec![field_spec],
            data_source_id: Some(data_source)
        };

        // Convert struct to byte stream
        let array = record.to_bytes();


        assert_eq!(true, true);
    }
}
