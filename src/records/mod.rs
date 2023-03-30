use std::string::ParseError;

mod util;


trait Recordable {
    fn parse() -> Result<Self, ParseError>;
}

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


struct Fix {
    pub timestamp: Time,
    pub coordinates: Coordinate,
    pub validity: bool,
    pub pressure_alt: i16,
    pub gps_alt: i16,
    extension: String,
}

impl Recordable for Fix {
    fn parse() -> Result<Self, ParseError> {
        todo!()
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
