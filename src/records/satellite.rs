use crate::records::error::IGCError;
use crate::records::error::IGCError::SatelliteInitError;
use crate::records::util::{Time};

#[derive(Debug, Clone)]
pub struct Satellite {
    pub time: Time,
    pub satellite_ids: Vec<String>,
}

impl Satellite {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 7 { return Err(SatelliteInitError(format!("'{line}' is too short to be parsed as a satellite record"))) }
        let time = Time::parse(&line[1..7])?;
        let satellite_ids = line[7..]
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        Ok(Self {time, satellite_ids})
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn satellite_record_parsed_correctly() {
        let sat_record = Satellite::parse("F160240040609123624221821").unwrap();
        assert_eq!(sat_record.time, Time::from_hms(16, 2, 40).unwrap());
        assert_eq!(sat_record.satellite_ids.len(), 9);
    }
}