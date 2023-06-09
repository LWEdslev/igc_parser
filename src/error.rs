#[derive(Debug, Clone)]
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
    SatelliteInitError(String),
    SecurityInitError(String),
    FileHeaderInitError(String),
    ExtensionInitError(String),
    CommentInitError(String),
    DataFixInitError(String),
    IGCFileInitError(String)
}
