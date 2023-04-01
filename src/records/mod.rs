use crate::records::error::IGCError;
use crate::records::error::IGCError::RecordInitError;
use crate::records::util::Parseable;
use crate::records::fix::Fix;

mod util;
mod error;
mod fix;

enum Record {
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
            None => Err(RecordInitError(format!("\"{}\" could not get first character", line))),
            Some(letter) => match letter {
                'A' => unimplemented!(),
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
                _ => Err(RecordInitError(format!("\"{}\" does not have a valid starting letter", line))),
            }
        }
    }
}

struct FlightRecorderID {}

struct TaskInfo {}

struct DiffGPS {}

struct Event {}

struct Satellite {}

struct Security {}

struct FileHeader {}

struct FixExtension {}

struct DataFixExtension {}

struct DataFix {}

struct Comment {}