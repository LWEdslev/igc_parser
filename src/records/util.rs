use std::num::ParseIntError;
use crate::records::error::IGCError;
use crate::records::error::IGCError::*;

pub trait Parseable {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized;
}

type Seconds = u32;

#[derive(PartialEq, Clone, Debug)]
pub struct Time {
    h: u8,
    m: u8,
    s: u8,
}

impl Parseable for Time {
    fn parse(line: &str) -> Result<Self, IGCError> {
        if line.len() != 6 {return Err(TimeInitError(format!("\"{}\" is not 6 characters long", line)))}
        match (line[0..2].parse::<u8>(), line[2..4].parse::<u8>(), line[4..6].parse::<u8>()) {
            (Ok(h), Ok(m), Ok(s)) => Time::from_hms(h, m, s),
            _ => Err(TimeInitError(format!("unable to parse \"{}\" as numbers", line))),
        }
    }
}

impl Time {
    pub fn from_hms(h: u8, m: u8, s: u8) -> Result<Self, IGCError> {
        if h > 23 { return Err(TimeInitError(format!("{} hours are too many, there must be less than 24 hours", h)))}
        if m > 59 { return Err(TimeInitError(format!("{} minutes are too many, there must be less 60 minutes", m)))}
        if s > 59 { return Err(TimeInitError(format!("{} seconds are too many, there must be less 60 seconds", s)))}
        Ok(
            Self { h, m, s }
        )
    }

    pub fn from_seconds_since_midnight(s: u32) -> Result<Self, IGCError> {
        if s >= 86400 { return Err(TimeInitError(format!("{} seconds is too large to fit in 24 hours", s)))}
        Time::from_hms(
            (s / 3600) as u8,
            ((s % 3600) / 60) as u8,
            (s % 60) as u8)
    }

    pub fn seconds_since_midnight(&self) -> Seconds {
        3600 * (self.h as u32) + 60 * (self.m as u32) + (self.s as u32)
    }

    pub fn add_hours(&mut self, h: u8) -> Result<(), IGCError> {
        if self.h + h > 23 { return Err(TimeInitError(format!("tried to add {} hours with {}", self.h, h)))}
        Ok(self.h += h)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Coordinate {
    pub latitude: Latitude,
    pub longitude: Longitude,
}
//                 012345678
//B094139 5152202N 00032723W A001140015000854106968064092190039002770100
impl Parseable for Coordinate {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() != 17 {
            return Err(CoordinateInitError(format!("\"{}\" is not the correct length for a coordinate", line)))
        }
        let latitude = Latitude::parse(&line[0..8])?;
        let longitude = Longitude::parse(&line[8..17])?;
        Ok(Coordinate { latitude, longitude })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Latitude {
    pub degrees: u8,
    pub minutes: f32,
    pub is_north: bool,
}

impl Parseable for Latitude {
    fn parse(line: &str) -> Result<Self, IGCError> {
        let (degrees, minutes, is_north) = (&line[0..2], &line[2..7], &line[7..8]);
        let (degrees, minutes) = match (degrees.parse::<u8>(), minutes.parse::<f32>()) {
            (Ok(degrees), Ok(minutes)) => (degrees, minutes / 1000.),
            _ => return Err(CoordinateInitError(format!("unable to parse \"{}\"", line))),
        };
        let is_north = match is_north {
            "N" => true,
            "S" => false,
            _ => return Err(CoordinateInitError(format!("{} is not a valid latitude compass direction", is_north)))
        };
        Ok(Latitude {
            degrees,
            minutes,
            is_north,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Longitude {
    pub degrees: u8,
    pub minutes: f32,
    pub is_east: bool,
}
//  012 34567 8
//  000 32723 W
impl Parseable for Longitude {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        let (degrees, minutes) = match (line[0..3].parse::<u8>(), line[3..8].parse::<f32>()) {
            (Ok(degrees), Ok(minutes)) => (degrees, minutes / 1000.),
            _ => return Err(CoordinateInitError(format!("unable to parse \"{}\"", line))),
        };

        let is_east = match &line[8..9] {
            "E" => true,
            "W" => false,
            _ => return Err(CoordinateInitError(format!("{} is not a valid longitude compass direction", &line[8..9])))
        };

        Ok(Longitude {
            degrees,
            minutes,
            is_east,
        })
    }
}