use std::error::Error;
use std::num::ParseIntError;
use regex::internal::Char;
use crate::records::diff_gps::DiffGPS;
use crate::records::error::IGCError;
use crate::records::error::IGCError::{DiffGPSInitError, FixExtensionInitError, RecordInitError, SatelliteInitError, SecurityInitError};
use crate::records::event::Event;
use crate::records::file_header::FileHeader;
use crate::records::util::{Parseable, Time};
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;
use crate::records::satellite::Satellite;
use crate::records::security::Security;
use crate::records::task_info::TaskInfo;

pub mod util;
mod error;
pub mod fix;
mod flight_recorder_id;
mod task_info;
mod diff_gps;
mod event;
mod satellite;
mod security;
mod file_header;

#[derive(Debug, Clone)]
pub enum Record {
    A(FlightRecorderID),
    B(Fix),
    C(TaskInfo),
    D(DiffGPS),
    E(Event),
    F(Satellite),
    G(Security),
    H(FileHeader),
    I(Extension),
    J(Extension),
    K(DataFix),
    L(Comment),
}

impl Parseable for Record {
    fn parse(line: &str) -> Result<Self, IGCError> {
        match line.chars().next() {
            None => Err(RecordInitError(format!("'{}' could not get first character", line))),
            Some(letter) => match letter {
                'A' => Ok(Record::A(FlightRecorderID::parse(line)?)),
                'B' => Ok(Record::B(Fix::parse(line)?)),
                'C' => Ok(Record::C(TaskInfo::parse(line)?)),
                'D' => Ok(Record::D(DiffGPS::parse(line)?)),
                'E' => Ok(Record::E(Event::parse(line)?)),
                'F' => Ok(Record::F(Satellite::parse(line)?)),
                'G' => Ok(Record::G(Security::parse(line)?)),
                'H' => Ok(Record::H(FileHeader::parse(line)?)),
                'I' => Ok(Record::I(Extension::parse(line)?)),
                'J' => Ok(Record::J(Extension::parse(line)?)),
                'K' => unimplemented!(),
                'L' => unimplemented!(),
                _ => Err(RecordInitError(format!("'{}' does not have a valid starting letter", line))),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExtensionType {I, J}

#[derive(Debug, Clone)]
pub struct Extension {
    pub extension_type: ExtensionType,
    pub number_of_extensions: u8,
    pub extensions: Vec<(u8, u8, String)>
}

impl Parseable for Extension {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        let extension_type = match &line[0..1] {
            "I" => {ExtensionType::I},
            "J" => {ExtensionType::J},
            _ => return Err(FixExtensionInitError(format!("'{line}' does not start with a valid prefix for an extension")))
        }
        if line.len() < 3 { return Err(FixExtensionInitError(format!("'{line}' is too short to be parsed as a fix extension")))}
        let number_of_extensions = match line[1..3].parse::<u8>() {
            Ok(number_of_extensions) => number_of_extensions,
            Err(_) => return Err(FixExtensionInitError(format!("'{line}' does not have a valid number of extensions field")))
        };
        if line.len() == (3 + number_of_extensions * 7) as usize { return Err(FixExtensionInitError(format!("'{line}' does not have the correct length according to number of extensions"))) }
        let extensions = line[3..]
            .chars()
            .collect::<Vec<char>>()
            .chunks(7)
            .map(|c|
                (c[0..2].iter().collect::<String>().parse::<u8>(),
                 c[2..4].iter().collect::<String>().parse::<u8>(),
                 c[4..7].iter().collect::<String>())).collect::<Vec<(Result<u8, ParseIntError>, Result<u8, ParseIntError>, String)>>();
        if !extensions.iter().all(|(start, end, _)| start.is_ok() && end.is_ok()) {
            return Err(FixExtensionInitError(format!("'{line}' has invalid start/end characters")))
        }
        let extensions = extensions.into_iter().map(|(start, end, s)| {
            match (start, end) {
                (Ok(start), Ok(end)) => (start, end, s),
                _ => unreachable!(),
            }
        }).collect::<Vec<(u8, u8, String)>>();

        Ok(Self {extension_type, number_of_extensions, extensions})
    }
}

#[derive(Debug, Clone)]
pub struct DataFixExtension {

}

#[derive(Debug, Clone)]
pub struct DataFix {}

#[derive(Debug, Clone)]
pub struct Comment {}


