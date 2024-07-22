use crate::{error::IGCError::FileHeaderInitError, StrWrapper};
use crate::records::util::Date;
use crate::Result;
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub enum FileHeader {
    Date(Date),
    FixAccuracy(u16),
    PilotInCharge(StrWrapper),
    SecondPilot(StrWrapper),
    GliderType(StrWrapper),
    GliderID(StrWrapper),
    GPSDatum(StrWrapper),
    Firmware(StrWrapper),
    Hardware(StrWrapper),
    LoggerType(StrWrapper),
    GPSManufacturer(StrWrapper),
    PressureSensor(StrWrapper),
    CompetitionID(StrWrapper),
    CompetitionClass(StrWrapper),
}

fn get_file_header_with_string_content<'a>(line: &'a str, header_name: &str) -> Result<&'a str> {
    if line.len() < header_name.len() { return Err(FileHeaderInitError(format!("'{line}' does not have the correct length to be parsed as a pilot in charge record"))) };
    Ok(&line[header_name.len()..])
}

impl FileHeader {
    pub(crate) fn parse(line: &str) -> Result<Self> {
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
                Ok(FileHeader::PilotInCharge(pilot_in_charge.to_string().into()))
            },
            "HFCM2" => {
                let second_pilot = get_file_header_with_string_content(line, "HFCM2CREW2:")?;
                Ok(FileHeader::SecondPilot(second_pilot.to_string().into()))
            },
            "HFGTY" => {
                let content = get_file_header_with_string_content(line, "HFGTYGLIDERTYPE:")?;
                Ok(FileHeader::GliderType(content.to_string().into()))
            },
            "HFGID" => {
                let content = get_file_header_with_string_content(line, "HFGIDGLIDERID:")?;
                Ok(FileHeader::GliderID(content.to_string().into()))
            },
            "HFDTM" => {
                let content = get_file_header_with_string_content(line, "HFDTMNNNGPSDATUM:")?;
                Ok(FileHeader::GPSDatum(content.to_string().into()))
            },
            "HFRFW" => {
                let content = get_file_header_with_string_content(line, "HFRFWFIRMWAREVERSION:")?;
                Ok(FileHeader::Firmware(content.to_string().into()))
            },
            "HFRHW" => {
                let content = get_file_header_with_string_content(line, "HFRHWHARDWAREVERSION:")?;
                Ok(FileHeader::Hardware(content.to_string().into()))
            },
            "HFFTY" => {
                let content = get_file_header_with_string_content(line, "HFFTYFRTYPE:")?;
                Ok(FileHeader::LoggerType(content.to_string().into()))
            },
            "HFGPS" => {
                let content = get_file_header_with_string_content(line, "HFGPS")?;
                Ok(FileHeader::GPSManufacturer(content.to_string().into()))
            },
            "HFPRS" => {
                let content = get_file_header_with_string_content(line, "HFPRSPRESSALTSENSOR:")?;
                Ok(FileHeader::PressureSensor(content.to_string().into()))
            },
            "HFCID" => {
                let content = get_file_header_with_string_content(line, "HFCIDCOMPETITIONID:")?;
                Ok(FileHeader::CompetitionID(content.to_string().into()))
            },
            "HFCCL" => {
                let content = get_file_header_with_string_content(line, "HFCCLCOMPETITIONCLASS:")?;
                Ok(FileHeader::CompetitionClass(content.to_string().into()))
            },
            _ => Err(FileHeaderInitError(format!("'{line}' does not have a valid file header start")))
        }
    }
}