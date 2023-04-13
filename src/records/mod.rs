use std::num::ParseIntError;
use crate::records::diff_gps::DiffGPS;
use crate::records::error::IGCError;
use crate::records::error::IGCError::{DiffGPSInitError, RecordInitError};
use crate::records::event::Event;
use crate::records::util::Parseable;
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;
use crate::records::task_info::TaskInfo;

pub mod util;
mod error;
pub mod fix;
mod flight_recorder_id;
mod task_info;
mod diff_gps;
mod event;

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
                'E' => unimplemented!(),
                'F' => unimplemented!(),
                'G' => unimplemented!(),
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
pub struct Satellite {}

#[derive(Debug, Clone)]
pub struct Security {}

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

