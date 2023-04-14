use std::error::Error;
use std::num::ParseIntError;
use regex::internal::Char;
use crate::records::diff_gps::DiffGPS;
use crate::records::error::IGCError;
use crate::records::error::IGCError::{DiffGPSInitError, RecordInitError, SatelliteInitError, SecurityInitError};
use crate::records::event::Event;
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
    I(FixExtension),
    J(DataFixExtension),
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
                'H' => unimplemented!(),
                'I' => unimplemented!(),
                'J' => unimplemented!(),
                'K' => unimplemented!(),
                'L' => unimplemented!(),
                _ => Err(RecordInitError(format!("'{}' does not have a valid starting letter", line))),
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct FileHeader {}

#[derive(Debug, Clone)]
pub struct FixExtension {}

#[derive(Debug, Clone)]
pub struct DataFixExtension {}

#[derive(Debug, Clone)]
pub struct DataFix {}

#[derive(Debug, Clone)]
pub struct Comment {}


