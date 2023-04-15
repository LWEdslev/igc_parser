use crate::records::error::IGCError;
use crate::records::error::IGCError::EventInitError;
use crate::records::util::Time;


#[derive(Debug, Clone)]
pub struct Event {
    pub time: Time,
    pub event_type: String,
    pub extension: String,
}

impl Event {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 10 { return Err(EventInitError(format!("'{line}' is too short to be parsed as an event record"))) };
        let time = Time::parse(&line[1..7])?;
        let event_type = line[7..10].to_string();
        let extension = line[10..].to_string();
        Ok(Self {time, event_type, extension})
    }
}

impl Event {
    pub fn is_pev(&self) -> bool {
        self.event_type == "PEV".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn event_is_parsed_correctly() {
        let event = Event::parse("E160245PEV").unwrap();
        assert_eq!(event.time, Time::from_hms(16, 2, 45).unwrap());
        assert_eq!(event.extension, "".to_string());
        assert_eq!(event.event_type, "PEV".to_string());
        assert!(event.is_pev());
    }
}

