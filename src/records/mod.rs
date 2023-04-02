use crate::records::error::IGCError;
use crate::records::error::IGCError::RecordInitError;
use crate::records::util::Parseable;
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;

pub mod util;
mod error;
pub mod fix;
mod flight_recorder_id;

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
                'C' => unimplemented!(),
                'D' => unimplemented!(),
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
pub struct TaskInfo {}

#[derive(Debug, Clone)]
pub struct DiffGPS {}

#[derive(Debug, Clone)]
pub struct Event {}

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