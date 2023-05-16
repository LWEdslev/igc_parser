use crate::error::IGCError;
use crate::error::IGCError::FRIDInitError;
use crate::Result;


#[derive(Debug, Clone)]
pub struct FlightRecorderID {
    pub manufacturer: String,
    pub id: String,
    pub extension: String,
}

impl FlightRecorderID {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.chars().count() < 7 { return Err(FRIDInitError(format!("'{}' is too short for an A record", line)))};
        let manufacturer = line[1..4].to_string();
        let id = line[4..7].to_string();
        let extension = line[7..].to_string();
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
        assert_eq!(frid.manufacturer, "MMM");
        assert_eq!(frid.id, "NNN");
        assert_eq!(frid.extension, "TEXTSTRING");
    }
}