use std::error::Error;
use std::num::ParseIntError;
use regex::internal::Char;
use crate::records::diff_gps::DiffGPS;
use crate::records::error::IGCError;
use crate::records::error::IGCError::{CommentInitError, DataFixInitError, DiffGPSInitError, ExtensionInitError, RecordInitError, SatelliteInitError, SecurityInitError};
use crate::records::event::Event;
use crate::records::file_header::FileHeader;
use crate::records::util::{Parseable, Time};
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;
use crate::records::satellite::Satellite;
use crate::records::security::Security;
use crate::records::task_info::TaskInfo;

pub mod util;
mod error;
pub mod fix;
mod flight_recorder_id;
mod task_info;
mod diff_gps;
mod event;
mod satellite;
mod security;
mod file_header;

#[derive(Debug, Clone)]
pub enum Record {
    A(FlightRecorderID),
    B(Fix),
    C(TaskInfo),
    D(DiffGPS),
    E(Event),
    F(Satellite),
    G(Security),
    H(FileHeader),
    I(Extension),
    J(Extension),
    K(DataFix),
    L(Comment),
}

impl Parseable for Record {
    fn parse(line: &str) -> Result<Self, IGCError> {
        match line.chars().next() {
            None => Err(RecordInitError(format!("'{}' could not get first character", line))),
            Some(letter) => match letter {
                'A' => Ok(Record::A(FlightRecorderID::parse(line)?)),
                'B' => Ok(Record::B(Fix::parse(line)?)),
                'C' => Ok(Record::C(TaskInfo::parse(line)?)),
                'D' => Ok(Record::D(DiffGPS::parse(line)?)),
                'E' => Ok(Record::E(Event::parse(line)?)),
                'F' => Ok(Record::F(Satellite::parse(line)?)),
                'G' => Ok(Record::G(Security::parse(line)?)),
                'H' => Ok(Record::H(FileHeader::parse(line)?)),
                'I' => Ok(Record::I(Extension::parse(line)?)),
                'J' => Ok(Record::J(Extension::parse(line)?)),
                'K' => Ok(Record::K(DataFix::parse(line)?)),
                'L' => Ok(Record::L(Comment::parse(line)?)),
                _ => Err(RecordInitError(format!("'{}' does not have a valid starting letter", line))),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExtensionType {I, J}

#[derive(Debug, Clone)]
pub struct Extension {
    pub extension_type: ExtensionType,
    pub number_of_extensions: u8,
    pub extensions: Vec<(u8, u8, String)>
}

impl Parseable for Extension {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
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

#[derive(Debug, Clone)]
pub struct DataFix {
    pub time: Time,
    pub content: String,
}

impl Parseable for DataFix {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 7 { return Err(DataFixInitError(format!("'{line}' is too short to be parsed as a data fix"))) };
        let time = Time::parse(&line[1..7])?;
        let content = line[7..].to_string();
        Ok(Self {time, content})
    }
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub content: String
}

impl Parseable for Comment {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 1 { return Err(CommentInitError(format!("'{line}' is too short to be a comment")))}
        let content = line[1..].to_string();
        Ok(Self {content})
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn example_file_can_be_parsed_without_errors() {
        let content = "AXXXABC FLIGHT:1
HFFXA035
HFDTE160701
HFPLTPILOTINCHARGE: Bloggs Bill D
HFCM2CREW2: Smith-Barry John A
HFGTYGLIDERTYPE: Schleicher ASH-25
HFGIDGLIDERID: ABCD-1234
HFDTM100GPSDATUM: WGS-1984
HFRFWFIRMWAREVERSION:6.4
HFRHWHARDWAREVERSION:3.0
HFFTYFRTYPE: Manufacturer, Model
HFGPSMarconiCanada: Superstar,12ch, max10000m
HFPRSPRESSALTSENSOR: Sensyn, XYZ1111, max11000m
HFCIDCOMPETITIONID: XYZ-78910
HFCCLCOMPETITIONCLASS:15m Motor Glider
I033638FXA3940SIU4143ENL
J010812HDT
C150701213841160701000102 500K Tri
C5111359N00101899W Lasham Clubhouse
C5110179N00102644W Lasham Start S, Start
C5209092N00255227W Sarnesfield, TP1
C5230147N00017612W Norman Cross, TP2
C5110179N00102644W Lasham Start S, Finish
C5111359N00101899W Lasham Clubhouse
F160240040609123624221821
B1602405407121N00249342WA002800042120509950
D20331
E160245PEV
B1602455107126N00149300WA002880042919509020
B1602505107134N00149283WA002900043221009015
B1602555107140N00149221WA002900043020009012
F1603000609123624221821
B1603005107150N00149202WA002910043225608009
E160305PEVCRLF
B1603055107180N00149185WA002910043521008015
B1603105107212N00149174WA002930043519608024
K16024800090
B1602485107220N00149150WA004940043619008018
B1602525107330N00149127WA004960043919508015
LXXXRURITANIAN STANDARD NATIONALS DAY 1
LXXXFLIGHT TIME: 4:14:25, TASK SPEED:58.48KTS
GREJNGJERJKNJKRE31895478537H43982FJN9248F942389T433T
GJNJK2489IERGNV3089IVJE9GO398535J3894N358954983O0934
GSKTO5427FGTNUT5621WKTC6714FT8957FGMKJ134527FGTR6751
GK2489IERGNV3089IVJE39GO398535J3894N358954983FTGY546
G12560DJUWT28719GTAOL5628FGWNIST78154INWTOLP7815FITN";
        content.lines().for_each(|line| { let rec = Record::parse(line); if rec.is_err() {println!("{line}") ; rec.unwrap();}})
    }
}