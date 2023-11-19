use thiserror::Error;

#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, Error)]
pub enum IGCError {
    #[error("Error initializing time: {0}")]
    TimeInitError(String),
    #[error("Error initializing date: {0}")]
    DateInitError(String),
    #[error("Error initializing fix: {0}")]
    FixInitError(String),
    #[error("Error initializing record: {0}")]
    RecordInitError(String),
    #[error("Error initializing coordinate: {0}")]
    CoordinateInitError(String),
    #[error("Error initializing flight recorder ID: {0}")]
    FRIDInitError(String),
    #[error("Error initializing task info: {0}")]
    TaskInfoInitError(String),
    #[error("Error initializing GPS: {0}")]
    DiffGPSInitError(String),
    #[error("Error initializing event: {0}")]
    EventInitError(String),
    #[error("Error initializing satellite: {0}")]
    SatelliteInitError(String),
    #[error("Error initializing security: {0}")]
    SecurityInitError(String),
    #[error("Error initializing file header: {0}")]
    FileHeaderInitError(String),
    #[error("Error initializing extension: {0}")]
    ExtensionInitError(String),
    #[error("Error initializing comment: {0}")]
    CommentInitError(String),
    #[error("Error initializing data fix: {0}")]
    DataFixInitError(String),
    #[error("Error initializing IGC file: {0}")]
    IGCFileInitError(String)
}