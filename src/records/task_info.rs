use crate::records::error::IGCError;
use crate::records::error::IGCError::TaskInfoInitError;
use crate::records::util::{Coordinate, Date, Parseable, Time};

#[derive(Debug, Clone)]
pub enum TaskInfo {
    TaskPoint(TaskPoint),
    DeclarationTime(DeclarationTime),
}

impl Parseable for TaskInfo {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 18 { return Err(TaskInfoInitError(format!("'{}' is too short to be parsed as kind of task info record", line))) }
        if line[1..17].chars().all(|c| c.is_numeric()) {
            Ok(TaskInfo::DeclarationTime(DeclarationTime::parse(line)?))
        } else {
            Ok(TaskInfo::TaskPoint(TaskPoint::parse(line)?))
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskPoint {
    pub coordinate: Coordinate,
    pub name: Option<String>,
}

impl Parseable for TaskPoint {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        let coordinate = Coordinate::parse(&line[1..18])?;
        let name = if line.len() == 18 { None } else {
            Some(line[18..].to_string())
        };
        Ok(Self { coordinate, name })
    }
}

#[derive(Debug, Clone)]
pub struct DeclarationTime {
    pub date: Date,
    pub time: Time,
    extra: String,
}

impl Parseable for DeclarationTime {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 23 { return Err(TaskInfoInitError(format!("'{}' is too short to be a declaration time record", line))) }
        let date = Date::parse(&line[1..7])?;
        let time = Time::parse(&line[7..13])?;
        let extra = line[13..].to_string();
        Ok(Self { date, time, extra })
    }
}

impl DeclarationTime {
    pub fn get_extra(&self) -> String {
        self.extra.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::records::util::{Latitude, Longitude};
    use super::*;

    #[test]
    fn declaration_time_parsed_correctly() {
        if let Ok(TaskInfo::DeclarationTime(decl_time)) = TaskInfo::parse("C070323213339000000000103") {
            assert_eq!(decl_time.time, Time::from_hms(21, 33, 39).unwrap());
            assert_eq!(decl_time.date, Date { d: 7, m: 3, y: 23 })
        } else {
            assert!(false)
        }
    }

    #[test]
    fn task_point_parsed_correctly() {
        if let Ok(TaskInfo::TaskPoint(task_point)) = TaskInfo::parse("C3835269S17609420ETASA Taupo Start A") {
            assert_eq!(task_point.coordinate.latitude, Latitude {
                degrees: 38,
                minutes: 35.269,
                is_north: false,
            });

            assert_eq!(task_point.coordinate.longitude, Longitude {
                degrees: 176,
                minutes: 9.42,
                is_east: true,
            });

            assert_eq!(task_point.name.unwrap(), String::from("TASA Taupo Start A"))
        } else {
            assert!(false)
        }
    }
}