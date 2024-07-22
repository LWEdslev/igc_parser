use crate::{error::IGCError::EventInitError, StrWrapper};
use crate::records::util::Time;
use crate::Result;

#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct Event {
    pub time: Time,
    pub event_type: StrWrapper,
    pub extension: StrWrapper,
}

impl Event {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.len() < 10 { return Err(EventInitError(format!("'{line}' is too short to be parsed as an event record"))) };
        let time = Time::parse(&line[1..7])?;
        let event_type = line[7..10].to_string().into();
        let extension = line[10..].to_string().into();
        Ok(Self {time, event_type, extension})
    }
    
    pub fn is_pev(&self) -> bool {
        self.event_type.as_ref() == "PEV"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_is_parsed_correctly() {
        let event = Event::parse("E160245PEV").unwrap();
        assert_eq!(event.time, Time::from_hms(16, 2, 45).unwrap());
        assert_eq!(event.extension, "".to_string().into());
        assert_eq!(event.event_type, "PEV".to_string().into());
        assert!(event.is_pev());
    }
}

