#[derive(Debug)]
pub enum IGCError {
    TimeInitError(String),
    FixInitError(String),
    CoordInitError(String),
    RecordInitError(String),
    CoordinateInitError(String),
}
