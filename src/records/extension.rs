use std::num::ParseIntError;
use crate::{error::IGCError::ExtensionInitError, StrWrapper};
use crate::Result;

#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionType {I, J}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct Extension {
    pub extension_type: ExtensionType,
    pub number_of_extensions: u8,
    pub extensions: Vec<(u8, u8, StrWrapper)>
}

impl Extension {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        let extension_type = match &line[0..1] {
            "I" => ExtensionType::I,
            "J" => ExtensionType::J,
            _ => return Err(ExtensionInitError(format!("'{line}' does not start with a valid prefix for an extension")))
        };
        if line.len() < 3 { return Err(ExtensionInitError(format!("'{line}' is too short to be parsed as a fix extension")))}
        let number_of_extensions = match line[1..3].parse::<u8>() {
            Ok(number_of_extensions) => number_of_extensions,
            Err(_) => return Err(ExtensionInitError(format!("'{line}' does not have a valid number of extensions field")))
        };
        if line.len() != (3 + number_of_extensions * 7) as usize { return Err(ExtensionInitError(format!("'{line}' does not have the correct length according to number of extensions"))) }
        let extensions = line[3..]
            .chars()
            .collect::<Vec<char>>()
            .chunks(7)
            .map(|c|
                (c[0..2].iter().collect::<String>().parse::<u8>(),
                 c[2..4].iter().collect::<String>().parse::<u8>(),
                 c[4..7].iter().collect::<String>())).collect::<Vec<(core::result::Result<u8, ParseIntError>, core::result::Result<u8, ParseIntError>, String)>>();
        if !extensions.iter().all(|(start, end, _)| start.is_ok() && end.is_ok()) {
            return Err(ExtensionInitError(format!("'{line}' has invalid start/end characters")))
        }
        let extensions = extensions.into_iter().map(|(start, end, s)| {
            (start.unwrap(), end.unwrap(), s.into())
        }).collect::<Vec<_>>();

        Ok(Self {extension_type, number_of_extensions, extensions})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extension_test() {
        let s = "I023638FXA3940SIU";
        let extension = Extension::parse(s).unwrap();
        assert_eq!(extension.extension_type, ExtensionType::I);
        assert_eq!(extension.number_of_extensions, 2);
        assert_eq!(extension.extensions, vec![(36, 38, "FXA".into()), (39, 40, "SIU".into())]);

        // negative, too short
        let s = "I0";
        assert!(Extension::parse(s).is_err());

        // negative, valid start, invalid end
        let s = "I0236A8FXA3940SIU";
        assert!(Extension::parse(s).is_err());

        // negative, invalid start, valid end
        let s = "I0A3638FXA3940SIU";
        assert!(Extension::parse(s).is_err());

        // negative
        let s = "A0A3638FXA3940SIU";
        assert!(Extension::parse(s).is_err());
    }
}
        