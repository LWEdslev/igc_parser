use std::rc::Rc;

use crate::records::util::{Coordinate, Time};
use crate::error::IGCError::FixInitError;
use crate::{Result, StrWrapper};
#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
/// Fix
pub struct Fix {
    pub timestamp: Time,
    pub coordinates: Coordinate,
    pub pressure_alt: i16,
    pub gps_alt: Option<i16>, //option because of validity flag
    pub extension: StrWrapper,
}

impl Fix {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.chars().count() < 35 {
            return Err(FixInitError(format!("\"{}\" is too short to be parsed as a fix", line)))
        }
        if !line.starts_with('B') {
            return Err(FixInitError(format!("\"{}\" does not start with B and can therefore not be parsed as a B record (Fix)", line)))
        }
        let timestamp = Time::parse(&line[1..7])?;
        let coordinates = Coordinate::parse(&line[7..24])?;
        let gps_alt = match &line[24..25] {
            "A" => match line[30..35].parse::<i16>() {
                Ok(alt) => Some(alt),
                Err(_) => return Err(FixInitError(format!("\"{}\" could not parse GPS altitude", line)))
            },
            "V" => None,
            _ => return Err(FixInitError(format!("\"{}\" does not have A or V in GPS validity field", line)))
        };
        let pressure_alt = match line[25..30].parse::<i16>() {
            Ok(alt) => alt,
            Err(_) => return Err(FixInitError(format!("\"{}\" could not parse pressure altitude", line)))
        };

        let extension = line[35..].to_string().into();

        Ok(
            Fix {
                timestamp,
                coordinates,
                pressure_alt,
                gps_alt,
                extension,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::records::Record;
    use crate::records::util::{Latitude, Longitude};
    use super::*;

    #[test]
    fn fix_parsed_correctly() {
        //          0         1         2         3         4         5         6
        //          0123456789012345678901234567890123456789012345678901234567890123456
        let line = "B0941395152202N00032723WA001140015000854106968064092190039002770100";
        let record = Record::parse(line).unwrap();
        if let Record::B(fix) = record {
            let Fix {
                timestamp,
                coordinates,
                pressure_alt,
                gps_alt,
                extension
            } = fix;

            let true_coordinate = Coordinate {latitude: Latitude {
                degrees: 51, minutes: 52.202, is_north: true
            }, longitude: Longitude {
                degrees: 0,
                minutes: 32.723,
                is_east: false,
            }};

            assert_eq!(Time::from_hms(9, 41, 39).unwrap(), timestamp);
            assert_eq!(true_coordinate, coordinates);
            assert_eq!(pressure_alt, 114);
            assert_eq!(gps_alt, Some(150));
            assert_eq!(extension, String::from("00854106968064092190039002770100").into());
        } else {
            assert!(false)
        }
    }
}