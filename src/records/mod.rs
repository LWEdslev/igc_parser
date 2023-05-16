use crate::Result;
use crate::records::diff_gps::DiffGPS;
use crate::error::IGCError;
use crate::error::IGCError::{RecordInitError};
use crate::records::{event::Event, file_header::FileHeader, fix::Fix, flight_recorder_id::FlightRecorderID, satellite::Satellite, security::Security, task_info::TaskInfo};
use crate::records::comment::Comment;
use crate::records::data_fix::DataFix;
use crate::records::extension::Extension;

pub mod util;
pub mod fix;
pub mod flight_recorder_id;
pub mod task_info;
pub mod diff_gps;
pub mod event;
pub mod satellite;
pub mod security;
pub mod file_header;
pub mod comment;
pub mod extension;
pub mod data_fix;

/// Record enum for getting different record types
/// Each element contains a struct/enum that is the result of parsing a specific line
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

impl Record {
    /// Returns an Ok(Record) corresponding to the line that was parsed, returns Err(IGCError) if something went wrong
    /// Should never panic!
    /// # Arguments
    ///
    ///  * `line` - A string slice that holds the line to be parsed
    /// # Examples
    /// ```
    /// use igc_parser::records::Record;
    /// let fix = match Record::parse("B1602405407121N00249342WA002800042120509950") {
    ///     Ok(Record::B(fix)) => fix,
    ///     _ => panic!("invalid valid string slice"),
    /// };
    /// println!("Succesfully parsed: {:?}", fix)
    /// ```
    pub fn parse(line: &str) -> Result<Self> {
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