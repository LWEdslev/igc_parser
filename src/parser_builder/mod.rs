use std::marker::PhantomData;

use crate::error::IGCError::IGCFileInitError;
use crate::records::comment::Comment;
use crate::records::data_fix::DataFix;
use crate::records::diff_gps::DiffGPS;
use crate::records::event::Event;
use crate::records::extension::Extension;
use crate::records::file_header::FileHeader;
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;
use crate::records::satellite::Satellite;
use crate::records::security::Security;
use crate::records::task_info::TaskInfo;
use crate::records::Record;
use crate::Result;

/*
To the reader of this file
The const bool generics give a nice effect without ever having to return options
It unfortunately does not look very nice and requires a lot of generic copying code
*/


pub struct Parsed<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> {
    fr_ids: Option<Vec<Result<FlightRecorderID>>>,
    fixes: Option<Vec<Result<Fix>>>,
    task_info: Option<Vec<Result<TaskInfo>>>,
    differential_gps_records: Option<Vec<Result<DiffGPS>>>,
    events: Option<Vec<Result<Event>>>,
    satellite_vec: Option<Vec<Result<Satellite>>>,
    security_vec: Option<Vec<Result<Security>>>,
    file_header_vec: Option<Vec<Result<FileHeader>>>,
    i_extension_vec: Option<Vec<Result<Extension>>>,
    j_extension_vec: Option<Vec<Result<Extension>>>,
    data_fix_vec: Option<Vec<Result<DataFix>>>,
    comment_vec: Option<Vec<Result<Comment>>>,
}

impl<
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<true,B,C,D,E,F,G,H,I,J,K,L> {
    /// Get flight records ID's from the parsed file
    /// Identical to `get_a_records`
    pub fn get_fr_ids(&self) -> &Vec<Result<FlightRecorderID>> {
        self.fr_ids.as_ref().expect("uncreachable typestate error")
    }

    /// Get flight records ID's from the parsed file
    /// Identical to `get_fr_ids`
    pub fn get_a_records(&self) -> &Vec<Result<FlightRecorderID>> {
        self.get_fr_ids()
    }
}

impl<
const A: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,true,C,D,E,F,G,H,I,J,K,L> {
    /// Get fixes from the parsed file
    /// Identical to `get_b_records`
    pub fn get_fixes(&self) -> &Vec<Result<Fix>> {
        self.fixes.as_ref().expect("uncreachable typestate error")
    }

    /// Get flight records ID's from the parsed file
    /// Identical to `get_fixes`
    pub fn get_b_records(&self) -> &Vec<Result<Fix>> {
        self.get_fixes()
    }
}

impl<
const A: bool,
const B: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,true,D,E,F,G,H,I,J,K,L> {
    /// Get task info from the parsed file
    /// Identical to `get_c_records`
    pub fn get_task_info(&self) -> &Vec<Result<TaskInfo>> {
        self.task_info.as_ref().expect("uncreachable typestate error")
    }

    /// Get task info from the parsed file
    /// Identical to `get_task_info`
    pub fn get_c_records(&self) -> &Vec<Result<TaskInfo>> {
        self.get_task_info()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,true,E,F,G,H,I,J,K,L> {
    /// Get differential GPS records from the parsed file
    /// Identical to `get_d_records`
    pub fn get_differential_gps_records(&self) -> &Vec<Result<DiffGPS>> {
        self.differential_gps_records.as_ref().expect("uncreachable typestate error")
    }

    /// Get differential GPS records from the parsed file
    /// Identical to `get_differential_gps_records`
    pub fn get_d_records(&self) -> &Vec<Result<DiffGPS>> {
        self.get_differential_gps_records()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,D,true,F,G,H,I,J,K,L> {
    /// Get events from the parsed file
    /// Identical to `get_e_records`
    pub fn get_events(&self) -> &Vec<Result<Event>> {
        self.events.as_ref().expect("uncreachable typestate error")
    }

    /// Get events from the parsed file
    /// Identical to `get_events`
    pub fn get_e_records(&self) -> &Vec<Result<Event>> {
        self.get_events()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,D,E,true,G,H,I,J,K,L> {
    /// Get satellite data from the parsed file
    /// Identical to `get_f_records`
    pub fn get_satellite_vec(&self) -> &Vec<Result<Satellite>> {
        self.satellite_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get satellite data from the parsed file
    /// Identical to `get_satellite_vec`
    pub fn get_f_records(&self) -> &Vec<Result<Satellite>> {
        self.get_satellite_vec()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,D,E,F,true,H,I,J,K,L> {
    /// Get security data from the parsed file
    /// Identical to `get_g_records`
    pub fn get_security_vec(&self) -> &Vec<Result<Security>> {
        self.security_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get security data from the parsed file
    /// Identical to `get_security_vec`
    pub fn get_g_records(&self) -> &Vec<Result<Security>> {
        self.get_security_vec()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const I: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,D,E,F,G,true,I,J,K,L> {
    /// Get file headers from the parsed file
    /// Identical to `get_h_records`
    pub fn get_file_header_vec(&self) -> &Vec<Result<FileHeader>> {
        self.file_header_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get file headers from the parsed file
    /// Identical to `get_file_header_vec`
    pub fn get_h_records(&self) -> &Vec<Result<FileHeader>> {
        self.get_file_header_vec()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const J: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,D,E,F,G,H,true,J,K,L> {
    /// Get fix extensions from the parsed file
    /// Identical to `get_i_records`
    pub fn get_fix_extension_vec(&self) -> &Vec<Result<Extension>> {
        self.i_extension_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get fix extensions from the parsed file
    /// Identical to `get_fix_extension_vec`
    pub fn get_i_records(&self) -> &Vec<Result<Extension>> {
        self.get_fix_extension_vec()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const K: bool,
const L: bool,
> Parsed<A,B,C,D,E,F,G,H,I,true,K,L> {
    /// Get data fix extensions from the parsed file
    /// Identical to `get_j_records`
    pub fn get_data_fix_extension_vec(&self) -> &Vec<Result<Extension>> {
        self.j_extension_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get data fix extensions from the parsed file
    /// Identical to `get_data_fix_extension_vec`
    pub fn get_j_records(&self) -> &Vec<Result<Extension>> {
        self.get_data_fix_extension_vec()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const L: bool,
> Parsed<A,B,C,D,E,F,G,H,I,J,true,L> {
    /// Get data fixes from the parsed file
    /// Identical to `get_k_records`
    pub fn get_data_fix_vec(&self) -> &Vec<Result<DataFix>> {
        self.data_fix_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get data fixes from the parsed file
    /// Identical to `get_data_fix_vec`
    pub fn get_k_records(&self) -> &Vec<Result<DataFix>> {
        self.get_data_fix_vec()
    }
}

impl<
const A: bool,
const B: bool,
const C: bool,
const D: bool,
const E: bool,
const F: bool,
const G: bool,
const H: bool,
const I: bool,
const J: bool,
const K: bool,
> Parsed<A,B,C,D,E,F,G,H,I,J,K,true> {
    /// Get comments from the parsed file
    /// Identical to `get_l_records`
    pub fn get_comment_vec(&self) -> &Vec<Result<Comment>> {
        self.comment_vec.as_ref().expect("uncreachable typestate error")
    }

    /// Get comments from the parsed file
    /// Identical to `get_comment_vec`
    pub fn get_l_records(&self) -> &Vec<Result<Comment>> {
        self.get_comment_vec()
    }
}

#[derive(Clone, Debug)]
pub struct ParserBuilder<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> {}

pub fn new_builder(
) -> ParserBuilder<false, false, false, false, false, false, false, false, false, false, false, false>
{
    ParserBuilder {}
}

impl<
        const A: bool,
        const B: bool,
        const C: bool,
        const D: bool,
        const E: bool,
        const F: bool,
        const G: bool,
        const H: bool,
        const I: bool,
        const J: bool,
        const K: bool,
        const L: bool,
    > ParserBuilder<A,B,C,D,E,F,G,H,I,J,K,L>
{
    pub fn on_file(self, content: &str) -> Result<Parsed<A,B,C,D,E,F,G,H,I,J,K,L>> {
        let mut fr_ids: Option<Vec<Result<FlightRecorderID>>> = match A {
            true => Some(Vec::new()),
            false => None,
        };
        let mut fixes: Option<Vec<Result<Fix>>> = match B {
            true => Some(Vec::new()),
            false => None,
        };
        let mut task_info: Option<Vec<Result<TaskInfo>>> = match C {
            true => Some(Vec::new()),
            false => None,
        };
        let mut differential_gps_records: Option<Vec<Result<DiffGPS>>> = match D {
                true => Some(Vec::new()),
                false => None,
            };
        let mut events: Option<Vec<Result<Event>>> = match E {
            true => Some(Vec::new()),
            false => None,
        };
        let mut satellite_vec: Option<Vec<Result<Satellite>>> = match F {
            true => Some(Vec::new()),
            false => None,
        };
        let mut security_vec: Option<Vec<Result<Security>>> = match G {
            true => Some(Vec::new()),
            false => None,
        };
        let mut file_header_vec: Option<Vec<Result<FileHeader>>> = match H {
            true => Some(Vec::new()),
            false => None,
        };
        let mut i_extension_vec: Option<Vec<Result<Extension>>> = match I {
            true => Some(Vec::new()),
            false => None,
        };
        let mut j_extension_vec: Option<Vec<Result<Extension>>> = match J {
            true => Some(Vec::new()),
            false => None,
        };
        let mut data_fix_vec: Option<Vec<Result<DataFix>>> = match K {
            true => Some(Vec::new()),
            false => None,
        };
        let mut comment_vec: Option<Vec<Result<Comment>>> = match L {
            true => Some(Vec::new()),
            false => None,
        };
        for line in content.lines() {
            let record = Record::parse(line);
            match line.chars().next() {
                Some(letter) => match letter {
                    'A' => {
                        if let Some(fr_ids) = &mut fr_ids {
                            fr_ids.push(match record {
                                Ok(Record::A(frid)) => Ok(frid),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'B' => {
                        if let Some(fixes) = &mut fixes {
                            fixes.push(match record {
                                Ok(Record::B(fix)) => Ok(fix),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'C' => {
                        if let Some(task_info) = &mut task_info {
                            task_info.push(match record {
                                Ok(Record::C(info)) => Ok(info),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'D' => {
                        if let Some(differential_gps_records) = &mut differential_gps_records {
                            differential_gps_records.push(match record {
                                Ok(Record::D(diff_gps)) => Ok(diff_gps),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'E' => {
                        if let Some(events) = &mut events {
                            events.push(match record {
                                Ok(Record::E(event)) => Ok(event),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'F' => {
                        if let Some(satellite_vec) = &mut satellite_vec {
                            satellite_vec.push(match record {
                                Ok(Record::F(sat)) => Ok(sat),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'G' => {
                        if let Some(security_vec) = &mut security_vec {
                            security_vec.push(match record {
                                Ok(Record::G(sec)) => Ok(sec),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'H' => {
                        if let Some(file_header_vec) = &mut file_header_vec {
                            file_header_vec.push(match record {
                                Ok(Record::H(header)) => Ok(header),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'I' => {
                        if let Some(i_extension_vec) = &mut i_extension_vec {
                            i_extension_vec.push(match record {
                                Ok(Record::I(ext)) => Ok(ext),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'J' => {
                        if let Some(j_extension_vec) = &mut j_extension_vec {
                            j_extension_vec.push(match record {
                                Ok(Record::J(ext)) => Ok(ext),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'K' => {
                        if let Some(data_fix_vec) = &mut data_fix_vec {
                            data_fix_vec.push(match record {
                                Ok(Record::K(data_fix)) => Ok(data_fix),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    'L' => {
                        if let Some(comment_vec) = &mut comment_vec {
                            comment_vec.push(match record {
                                Ok(Record::L(comment)) => Ok(comment),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        }
                    }
                    _ => {
                        return Err(IGCFileInitError(format!(
                            "{line} does not have a valid start letter"
                        )))
                    }
                },
                None => {
                    return Err(IGCFileInitError(format!(
                        "{line} does not have a valid start letter"
                    )))
                }
            }
        }
        Ok(Parsed {
            fr_ids,
            fixes,
            task_info,
            differential_gps_records,
            events,
            satellite_vec,
            security_vec,
            file_header_vec,
            i_extension_vec,
            j_extension_vec,
            data_fix_vec,
            comment_vec,
        })
    }
}
// Implementation for enabling parsing of flight record IDs when A is false.
impl<
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<false, B, C, D, E, F, G, H, I, J, K, L>
{
    /// Enable builder to parse flight recorder ID's
    /// Identical to `parse_a_records`
    pub fn parse_flight_record_id(self) -> ParserBuilder<true, B, C, D, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse flight recorder ID's
    /// Identical to `parse_flight_record_id`
    pub fn parse_a_records(self) -> ParserBuilder<true, B, C, D, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of fixes when B is false.
impl<
    const A: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, false, C, D, E, F, G, H, I, J, K, L>
{
    /// Enable builder to parse fixes
    /// Identical to `parse_b_records`
    pub fn parse_fixes(self) -> ParserBuilder<A, true, C, D, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse fixes
    /// Identical to `parse_fixes`
    pub fn parse_b_records(self) -> ParserBuilder<A, true, C, D, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of task info when C is false.
impl<
    const A: bool,
    const B: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, false, D, E, F, G, H, I, J, K, L>
{
    /// Enable builder to parse task info
    /// Identical to `parse_c_records`
    pub fn parse_task_info(self) -> ParserBuilder<A, B, true, D, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse task info
    /// Identical to `parse_task_info`
    pub fn parse_c_records(self) -> ParserBuilder<A, B, true, D, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of differential GPS records when D is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, false, E, F, G, H, I, J, K, L>
{
    /// Enable builder to parse differential GPS records
    /// Identical to `parse_d_records`
    pub fn parse_differential_gps_records(self) -> ParserBuilder<A, B, C, true, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse differential GPS records
    /// Identical to `parse_differential_gps_records`
    pub fn parse_d_records(self) -> ParserBuilder<A, B, C, true, E, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of events when E is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, false, F, G, H, I, J, K, L>
{
    /// Enable builder to parse events
    /// Identical to `parse_e_records`
    pub fn parse_events(self) -> ParserBuilder<A, B, C, D, true, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse events
    /// Identical to `parse_events`
    pub fn parse_e_records(self) -> ParserBuilder<A, B, C, D, true, F, G, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of satellite data when F is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, E, false, G, H, I, J, K, L>
{
    /// Enable builder to parse satellite data
    /// Identical to `parse_f_records`
    pub fn parse_satellite(self) -> ParserBuilder<A, B, C, D, E, true, G, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse satellite data
    /// Identical to `parse_satellite`
    pub fn parse_f_records(self) -> ParserBuilder<A, B, C, D, E, true, G, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of security data when G is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, E, F, false, H, I, J, K, L>
{
    /// Enable builder to parse security data
    /// Identical to `parse_g_records`
    pub fn parse_security(self) -> ParserBuilder<A, B, C, D, E, F, true, H, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse security data
    /// Identical to `parse_security`
    pub fn parse_g_records(self) -> ParserBuilder<A, B, C, D, E, F, true, H, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of file headers when H is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const I: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, E, F, G, false, I, J, K, L>
{
    /// Enable builder to parse file headers
    /// Identical to `parse_h_records`
    pub fn parse_file_header(self) -> ParserBuilder<A, B, C, D, E, F, G, true, I, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse file headers
    /// Identical to `parse_file_header`
    pub fn parse_h_records(self) -> ParserBuilder<A, B, C, D, E, F, G, true, I, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of fix extensions when I is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const J: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, E, F, G, H, false, J, K, L>
{
    /// Enable builder to parse fix extensions
    /// Identical to `parse_i_records`
    pub fn parse_fix_extension(self) -> ParserBuilder<A, B, C, D, E, F, G, H, true, J, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse fix extensions
    /// Identical to `parse_fix_extension`
    pub fn parse_i_records(self) -> ParserBuilder<A, B, C, D, E, F, G, H, true, J, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of data fix extensions when J is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const K: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, E, F, G, H, I, false, K, L>
{
    /// Enable builder to parse data fix extensions
    /// Identical to `parse_j_records`
    pub fn parse_data_fix_extension(self) -> ParserBuilder<A, B, C, D, E, F, G, H, I, true, K, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse data fix extensions
    /// Identical to `parse_data_fix_extension`
    pub fn parse_j_records(self) -> ParserBuilder<A, B, C, D, E, F, G, H, I, true, K, L> {
        ParserBuilder {}
    }
}

// Implementation for enabling parsing of data fixes when K is false.
impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const L: bool,
> ParserBuilder<A, B, C, D, E, F, G, H, I, J, false, L>
{
    /// Enable builder to parse data fixes
    /// Identical to `parse_k_records`
    pub fn parse_data_fix(self) -> ParserBuilder<A, B, C, D, E, F, G, H, I, J, true, L> {
        ParserBuilder {}
    }

    /// Enable builder to parse data fixes
    /// Identical to `parse_data_fix`
    pub fn parse_k_records(self) -> ParserBuilder<A, B, C, D, E, F, G, H, I, J, true, L> {
        ParserBuilder {}
    }
}

impl<
    const A: bool,
    const B: bool,
    const C: bool,
    const D: bool,
    const E: bool,
    const F: bool,
    const G: bool,
    const H: bool,
    const I: bool,
    const J: bool,
    const K: bool,
> ParserBuilder<A, B, C, D, E, F, G, H, I, J, K, false>
{
    /// Enable builder to parse comments
    /// Identical to `parse_l_records`
    pub fn parse_comments(self) -> ParserBuilder<A, B, C, D, E, F, G, H, I, J, K, true> {
        ParserBuilder {}
    }

    /// Enable builder to parse comments
    /// Identical to `parse_comments`
    pub fn parse_l_records(self) -> ParserBuilder<A, B, C, D, E, F, G, H, I, J, K, true> {
        ParserBuilder {}
    }
}
