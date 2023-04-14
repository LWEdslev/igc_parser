use crate::records::error::IGCError;
use crate::records::error::IGCError::FileHeaderInitError;
use crate::records::util::{Date, Parseable};

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

impl Parseable for FileHeader {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        match &line[0..5] {
            "HFDTE" => {
                if line.len() != 11 { return Err(FileHeaderInitError(format!("'{line}' does not have the correct length to be parsed as a file header date"))) };
                let date = Date::parse(&line[5..11])?;
                Ok(FileHeader::Date(date))
            },
            "HFFXA" => { todo!() },
            "HFPLT" => { todo!() },
            "HFCM2" => { todo!() },
            "HFGTY" => { todo!() },
            "HFGID" => { todo!() },
            "HFDTM" => { todo!() },
            "HFRFW" => { todo!() },
            "HFRHW" => { todo!() },
            "HFFTY" => { todo!() },
            "HFGPS" => { todo!() },
            "HFPRS" => { todo!() },
            "HFCID" => { todo!() },
            "HFCCL" => { todo!() },
            _ => Err(FileHeaderInitError(format!("'{line}' does not have a valid file header start")))
        }
    }
}