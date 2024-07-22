use crate::{error::IGCError::FRIDInitError, StrWrapper};
use crate::Result;
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct FlightRecorderID {
    pub manufacturer: StrWrapper,
    pub id: StrWrapper,
    pub extension: StrWrapper,
}

impl FlightRecorderID {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.chars().count() < 7 { return Err(FRIDInitError(format!("'{}' is too short for an A record", line)))};
        let manufacturer = line[1..4].to_string().into();
        let id = line[4..7].to_string().into();
        let extension = line[7..].to_string().into();
        Ok(FlightRecorderID {
            manufacturer,
            id,
            extension,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fr_id_parsed_correctly() {
        let frid = FlightRecorderID::parse("AMMMNNNTEXTSTRING").unwrap();
        assert_eq!(frid.manufacturer, "MMM".into());
        assert_eq!(frid.id, "NNN".into());
        assert_eq!(frid.extension, "TEXTSTRING".into());
    }
}