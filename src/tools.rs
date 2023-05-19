use crate::cat34::Cat34Message;
use crate::cat34::MessageType;
use crate::cat_error::Cat34Error;

pub fn check_mandatory_items(message: &Cat34Message) -> Option<Cat34Error> {
    match message.message_type {
        MessageType::NorthMarker => return check_north_marker_items(message),
        MessageType::SectorCrossing => return check_sector_crossing_items(message),
        MessageType::GeographicalFiltering => return check_geographical_filtering_items(message),
        MessageType::JammingStrobe => return check_jamming_strobe_items(message),
        MessageType::SolarStorm => return check_solar_storm_items(message),
        MessageType::SSRJammingStrobe => return check_ssr_jamming_strobe_items(message),
        MessageType::ModeSJammingStrobe => return check_modes_jamming_strobe_items(message),
    };
}

pub fn check_north_marker_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::NorthMarker {
        return Some(Cat34Error::I034_000Invalid);
    }
    if message.data_source_id == None {
        return Some(Cat34Error::I034_010Invalid);
    }
    if message.sector_number != None {
        return Some(Cat34Error::I034_020Invalid);
    }
    if message.time_of_day == None {
        return Some(Cat34Error::I034_030Invalid);
    }
    if message.generic_polar_window != None {
        return Some(Cat34Error::I034_100Invalid);
    }
    if message.data_filter != None {
        return Some(Cat34Error::I034_110Invalid);
    }
    return None;
}

pub fn check_sector_crossing_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::SectorCrossing {
        return Some(Cat34Error::I034_000Invalid);
    }
    if message.data_source_id == None {
        return Some(Cat34Error::I034_010Invalid);
    }
    if message.sector_number == None {
        return Some(Cat34Error::I034_020Invalid);
    }
    if message.time_of_day == None {
        return Some(Cat34Error::I034_030Invalid);
    }
    if message.antenna_rotation_speed != None {
        return Some(Cat34Error::I034_041Invalid);
    }
    if message.generic_polar_window != None {
        return Some(Cat34Error::I034_100Invalid);
    }
    if message.data_filter != None {
        return Some(Cat34Error::I034_110Invalid);
    }
    if message.position_data_source != None {
        return Some(Cat34Error::I034_120Invalid);
    }
    return None;
}

pub fn check_geographical_filtering_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::GeographicalFiltering {
        return Some(Cat34Error::I034_000Invalid);
    }
    if message.data_source_id == None {
        return Some(Cat34Error::I034_010Invalid);
    }
    if message.sector_number != None {
        return Some(Cat34Error::I034_020Invalid);
    }
    if message.antenna_rotation_speed != None {
        return Some(Cat34Error::I034_041Invalid);
    }
    if message.system_configuration_status != None {
        return Some(Cat34Error::I034_050Invalid);
    }
    if message.system_processing_mode != None {
        return Some(Cat34Error::I034_060Invalid);
    }
    if message.message_count_values != None {
        return Some(Cat34Error::I034_070Invalid);
    }
    if message.colimation_error != None {
        return Some(Cat34Error::I034_090Invalid);
    }
    if message.data_filter == None {
        return Some(Cat34Error::I034_110Invalid);
    }
    if message.position_data_source != None {
        return Some(Cat34Error::I034_120Invalid);
    }
    return None;
}

pub fn check_jamming_strobe_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::JammingStrobe {
        return Some(Cat34Error::I034_000Invalid);
    }

    let result = check_common_items(message);

    return result;
}

pub fn check_solar_storm_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::SolarStorm {
        return Some(Cat34Error::I034_000Invalid);
    }

    let result = check_common_items(message);

    return result;
}

pub fn check_ssr_jamming_strobe_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::SSRJammingStrobe {
        return Some(Cat34Error::I034_000Invalid);
    }

    let result = check_common_items(message);

    return result;
}

pub fn check_modes_jamming_strobe_items(message: &Cat34Message) -> Option<Cat34Error> {
    if message.message_type != MessageType::ModeSJammingStrobe {
        return Some(Cat34Error::I034_000Invalid);
    }

    let result = check_common_items(message);

    return result;
}

fn check_common_items(message: &Cat34Message) -> Option<Cat34Error> {
    // For Jamming Strobe, Solar Storm, SSR Jamming Strobe, ModeS Jamming Strobe
    if message.data_source_id == None {
        return Some(Cat34Error::I034_010Invalid);
    }
    if message.sector_number != None {
        return Some(Cat34Error::I034_020Invalid);
    }
    if message.antenna_rotation_speed != None {
        return Some(Cat34Error::I034_041Invalid);
    }
    if message.system_configuration_status != None {
        return Some(Cat34Error::I034_050Invalid);
    }
    if message.system_processing_mode != None {
        return Some(Cat34Error::I034_060Invalid);
    }
    if message.message_count_values != None {
        return Some(Cat34Error::I034_070Invalid);
    }
    if message.colimation_error != None {
        return Some(Cat34Error::I034_090Invalid);
    }
    if message.generic_polar_window == None {
        return Some(Cat34Error::I034_110Invalid);
    }
    if message.data_filter != None {
        return Some(Cat34Error::I034_110Invalid);
    }
    if message.position_data_source != None {
        return Some(Cat34Error::I034_120Invalid);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cat34::*;
    use crate::cat_error::Cat34Error;

    #[test]
    fn check_items() {
        let mut message = Cat34Message::new(MessageType::NorthMarker);

        let data_source_identifier = DataSourceIdentifier {
            sic: Some(42),
            sac: Some(26),
        };

        message.data_source_id = Some(data_source_identifier);

        let result = check_mandatory_items(&message);
        assert_eq!(result.unwrap(), Cat34Error::I034_030Invalid);
    }
}
