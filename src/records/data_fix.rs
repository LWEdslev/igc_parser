use crate::error::IGCError;
use crate::error::IGCError::DataFixInitError;
use crate::records::util::Time;
use crate::Result;
#[derive(Debug, Clone)]
pub struct DataFix {
    pub time: Time,
    pub content: String,
}

impl DataFix {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.len() < 7 { return Err(DataFixInitError(format!("'{line}' is too short to be parsed as a data fix"))) };
        let time = Time::parse(&line[1..7])?;
        let content = line[7..].to_string();
        Ok(Self {time, content})
    }
}