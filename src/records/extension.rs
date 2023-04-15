use std::num::ParseIntError;
use crate::error::IGCError;
use crate::error::IGCError::ExtensionInitError;

#[derive(Debug, Clone)]
pub enum ExtensionType {I, J}

#[derive(Debug, Clone)]
pub struct Extension {
    pub extension_type: ExtensionType,
    pub number_of_extensions: u8,
    pub extensions: Vec<(u8, u8, String)>
}

impl Extension {
    pub(crate) fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        let extension_type = match &line[0..1] {
            "I" => {ExtensionType::I},
            "J" => {ExtensionType::J},
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
                 c[4..7].iter().collect::<String>())).collect::<Vec<(Result<u8, ParseIntError>, Result<u8, ParseIntError>, String)>>();
        if !extensions.iter().all(|(start, end, _)| start.is_ok() && end.is_ok()) {
            return Err(ExtensionInitError(format!("'{line}' has invalid start/end characters")))
        }
        let extensions = extensions.into_iter().map(|(start, end, s)| {
            match (start, end) {
                (Ok(start), Ok(end)) => (start, end, s),
                _ => unreachable!(),
            }
        }).collect::<Vec<(u8, u8, String)>>();

        Ok(Self {extension_type, number_of_extensions, extensions})
    }
}