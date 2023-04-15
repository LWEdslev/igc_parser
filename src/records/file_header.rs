use crate::error::IGCError;
use crate::error::IGCError::FileHeaderInitError;
use crate::records::util::{Date};

#[derive(Debug, Clone)]
pub enum FileHeader {
    Date(Date),
    FixAccuracy(u16),
    PilotInCharge(String),
    SecondPilot(String),
    GliderType(String),
    GliderID(String),
    GPSDatum(String),
    Firmware(String),
    Hardware(String),
    LoggerType(String),
    GPSManufacturer(String),
    PressureSensor(String),
    CompetitionID(String),
    CompetitionClass(String),
}

fn get_file_header_with_string_content<'a>(line: &'a str, header_name: &str) -> Result<&'a str, IGCError> {
    if line.len() < header_name.len() { return Err(FileHeaderInitError(format!("'{line}' does not have the correct length to be parsed as a pilot in charge record"))) };
    Ok(&line[header_name.len()..])
}

impl FileHeader {
    pub(crate) fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        match &line[0..5] {
            "HFDTE" => {
                if line.len() != 11 { return Err(FileHeaderInitError(format!("'{line}' does not have the correct length to be parsed as a file header date"))) };
                let date = Date::parse(&line[5..11])?;
                Ok(FileHeader::Date(date))
            },
            "HFFXA" => {
                if line.len() != 8 { return Err(FileHeaderInitError(format!("'{line}' does not have the correct length to be parsed as a file header fix accuracy"))) };
                let accuracy = line[6..8].parse::<u16>();
                match accuracy {
                    Ok(accuracy) => Ok(FileHeader::FixAccuracy(accuracy)),
                    Err(_) => Err(FileHeaderInitError(format!("'{line}' can not be parsed as a fix accuracy number"))),
                }
            },
            "HFPLT" => {
                let pilot_in_charge = get_file_header_with_string_content(line, "HFPLTPILOTINCHARGE:")?;
                Ok(FileHeader::PilotInCharge(pilot_in_charge.to_string()))
            },
            "HFCM2" => {
                let second_pilot = get_file_header_with_string_content(line, "HFCM2CREW2:")?;
                Ok(FileHeader::SecondPilot(second_pilot.to_string()))
            },
            "HFGTY" => {
                let content = get_file_header_with_string_content(line, "HFGTYGLIDERTYPE:")?;
                Ok(FileHeader::GliderType(content.to_string()))
            },
            "HFGID" => {
                let content = get_file_header_with_string_content(line, "HFGIDGLIDERID:")?;
                Ok(FileHeader::GliderID(content.to_string()))
            },
            "HFDTM" => {
                let content = get_file_header_with_string_content(line, "HFDTMNNNGPSDATUM:")?;
                Ok(FileHeader::GPSDatum(content.to_string()))
            },
            "HFRFW" => {
                let content = get_file_header_with_string_content(line, "HFRFWFIRMWAREVERSION:")?;
                Ok(FileHeader::Firmware(content.to_string()))
            },
            "HFRHW" => {
                let content = get_file_header_with_string_content(line, "HFRHWHARDWAREVERSION:")?;
                Ok(FileHeader::Hardware(content.to_string()))
            },
            "HFFTY" => {
                let content = get_file_header_with_string_content(line, "HFFTYFRTYPE:")?;
                Ok(FileHeader::LoggerType(content.to_string()))
            },
            "HFGPS" => {
                let content = get_file_header_with_string_content(line, "HFGPS")?;
                Ok(FileHeader::GPSManufacturer(content.to_string()))
            },
            "HFPRS" => {
                let content = get_file_header_with_string_content(line, "HFPRSPRESSALTSENSOR:")?;
                Ok(FileHeader::PressureSensor(content.to_string()))
            },
            "HFCID" => {
                let content = get_file_header_with_string_content(line, "HFCIDCOMPETITIONID:")?;
                Ok(FileHeader::CompetitionID(content.to_string()))
            },
            "HFCCL" => {
                let content = get_file_header_with_string_content(line, "HFCCLCOMPETITIONCLASS:")?;
                Ok(FileHeader::CompetitionClass(content.to_string()))
            },
            _ => Err(FileHeaderInitError(format!("'{line}' does not have a valid file header start")))
        }
    }
}