use crate::error::IGCError;
use crate::error::IGCError::DiffGPSInitError;
use crate::Result;
#[derive(Debug, Clone)]
pub struct DiffGPS {
    pub qualifier: DiffGPSQualifier,
    pub dgps_station_id: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffGPSQualifier {
    GPS,
    DGPS,
}

impl DiffGPS {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.len() != 6 {return Err(DiffGPSInitError(format!("'{line} is not long enough to be parsed as a differential GPS record'")))}
        let qualifier = match line[1..2].parse::<u16>() {
            Ok(qualifier) => qualifier,
            Err(_) => return Err(DiffGPSInitError(format!("'{line}''s qualifier can not be parsed as a valid number"))),
        };
        let qualifier = match qualifier {
            1 => DiffGPSQualifier::GPS,
            2 => DiffGPSQualifier::DGPS,
            _ => return Err(DiffGPSInitError(format!("'{line}''s qualifier must be either 1 or 2"))),
        };

        let dgps_station_id = match line[2..6].parse::<u16>() {
            Ok(dgps_station_id) => dgps_station_id,
            Err(_) => return Err(DiffGPSInitError(format!("'{line}''s DGPS station ID can not be parsed as a valid number"))),
        };

        Ok(Self {qualifier, dgps_station_id})
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn diff_gps_should_be_correct() {
        let diff_gps = DiffGPS::parse("D21234").unwrap();
        assert_eq!(diff_gps.dgps_station_id, 1234);
        assert_eq!(diff_gps.qualifier, DiffGPSQualifier::DGPS);
    }
}