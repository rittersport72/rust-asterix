//extern crate chrono;
use chrono::offset::local::Local;
use chrono::datetime::DateTime;

/// Types of messages
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    NorthMarker = 1,
    SectorCrossing,
    GeographicalFiltering,
    JammingStrobe,
    SolarStorm,
    SSRJammingStrobe,
    ModeSJammingStrobe,
}

/// CAT34 message
#[derive(Debug, Clone, PartialEq)]
pub struct Cat34Message {
    /// I034/010
    pub data_source_id: Option<DataSourceIdentifier>,
    /// I034/000
    pub message_type: MessageType,
    /// I034/030
    pub time_of_day:  Option<DateTime<Local>>,
    /// I034/020
    pub sector_number: Option<f32>,
    /// I034/041
    pub antenna_rotation_speed: Option<f32>,
    /// I034/050
    pub system_configuration_status: Option<SystemConfigurationStatus>,
    /// I034/060
    pub system_processing_mode: Option<SystemProcessingMode>,
    /// I034/070
    pub message_count_values: Option<MessageCountValues>,
    /// I034/100
    pub generic_polar_window: Option<GenericPolarWindow>,
    /// I034/110
    pub data_filter: Option<u8>,
    /// I034/120
    pub position_data_source: Option<PositionDataSource>,
    /// I034/090
    pub colimation_error: Option<ColimationError>,
}

/// Implementation CAT34 message
impl Cat34Message {
    /// Creates a new message of the provided type, with all other fields set to None
    pub fn new(message_type: MessageType) -> Self {
        Self {
            data_source_id: None,
            message_type: message_type,
            time_of_day: None,
            sector_number: None,
            antenna_rotation_speed: None,
            system_configuration_status: None,
            system_processing_mode: None,
            message_count_values: None,
            generic_polar_window: None,
            data_filter: None,
            position_data_source: None,
            colimation_error: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataSourceIdentifier {
    pub sic: Option<u8>,
    pub sac: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemConfigurationStatus {
    pub com_subfield: Option<COMSubfieldC>,
    pub psr_subfield: Option<SRSubfieldC>,
    pub ssr_subfield: Option<SRSubfieldC>,
    pub mds_subfield: Option<MDSSubfieldC>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct COMSubfieldC {
    pub nogo: Option<bool>,
    pub rdpc: Option<bool>,
    pub tdpr: Option<bool>,
    pub ovlrdp: Option<bool>,
    pub ovlxmt: Option<bool>,
    pub msc: Option<bool>,
    pub tsv: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRSubfieldC {
    pub ant: Option<bool>,
    pub chab: Option<u8>,
    pub ovl: Option<bool>,
    pub msc: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MDSSubfieldC {
    pub ant: Option<bool>,
    pub chab: Option<u8>,
    pub ovlsur: Option<bool>,
    pub msc: Option<bool>,
    pub scf: Option<bool>,
    pub dlf: Option<bool>,
    pub ovlscf: Option<bool>,
    pub ovldlf: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemProcessingMode {
    pub com_subfield: Option<COMSubfieldP>,
    pub psr_subfield: Option<PSRSubfieldP>,
    pub ssr_subfield: Option<SSRSubfieldP>,
    pub mds_subfield: Option<MDSSubfieldP>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct COMSubfieldP {
    pub redrdp: Option<u8>,
    pub redxmt: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PSRSubfieldP {
    pub pol: Option<bool>,
    pub redrad: Option<u8>,
    pub stc: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SSRSubfieldP {
    pub redrad: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MDSSubfieldP {
    pub redrad: Option<u8>,
    pub clu: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageCountValues {
    pub rep: Option<u8>,
    pub typ: Option<u8>,
    pub counter: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColimationError {
    pub range_error: Option<f32>,
    pub azimuth_error: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericPolarWindow {
    pub rho_start: Option<f32>,
    pub rho_end: Option<f32>,
    pub theta_start: Option<f32>,
    pub theta_end: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionDataSource {
    pub height: Option<f32>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}