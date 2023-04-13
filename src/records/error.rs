#[derive(Debug)]
pub enum IGCError {
    TimeInitError(String),
    DateInitError(String),
    FixInitError(String),
    CoordInitError(String),
    RecordInitError(String),
    CoordinateInitError(String),
    FRIDInitError(String),
    TaskInfoInitError(String),
    DiffGPSInitError(String),
    EventInitError(String),
}
