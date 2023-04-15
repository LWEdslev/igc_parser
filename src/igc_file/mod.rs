use crate::records::comment::Comment;
use crate::records::data_fix::DataFix;
use crate::records::diff_gps::DiffGPS;
use crate::error::IGCError;
use crate::error::IGCError::IGCFileInitError;
use crate::records::event::Event;
use crate::records::extension::Extension;
use crate::records::file_header::FileHeader;
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;
use crate::records::Record;
use crate::records::satellite::Satellite;
use crate::records::security::Security;
use crate::records::task_info::TaskInfo;

#[derive(Clone)]
/// For parsing the entire file at once, then it gives access to vectors holding the result of the parsing
/// Not very efficient if you only need to parse one specific kind of record but still fast enough for almost all use cases
pub struct IGCFile {
    fr_ids: Vec<Result<FlightRecorderID, IGCError>>,
    fixes: Vec<Result<Fix, IGCError>>,
    task_info: Vec<Result<TaskInfo, IGCError>>,
    differential_gps_records: Vec<Result<DiffGPS, IGCError>>,
    events: Vec<Result<Event, IGCError>>,
    satellite_vec: Vec<Result<Satellite, IGCError>>,
    security_vec: Vec<Result<Security, IGCError>>,
    file_header_vec: Vec<Result<FileHeader, IGCError>>,
    i_extension_vec: Vec<Result<Extension, IGCError>>,
    j_extension_vec: Vec<Result<Extension, IGCError>>,
    data_fix_vec: Vec<Result<DataFix, IGCError>>,
    comment_vec: Vec<Result<Comment, IGCError>>,
}

impl IGCFile {
    /// # arguments
    /// ´content´ is the UTF-8 content of the file you want to parse,
    /// it is rare for this parsing to return an Err instead of Ok(Self) but it can happen
    pub fn parse(content: &str) -> Result<Self, IGCError> {
        let mut fr_ids: Vec<Result<FlightRecorderID, IGCError>> = Vec::new();
        let mut fixes: Vec<Result<Fix, IGCError>> = Vec::new();
        let mut task_info: Vec<Result<TaskInfo, IGCError>> = Vec::new();
        let mut differential_gps_records: Vec<Result<DiffGPS, IGCError>> = Vec::new();
        let mut events: Vec<Result<Event, IGCError>> = Vec::new();
        let mut satellite_vec: Vec<Result<Satellite, IGCError>> = Vec::new();
        let mut security_vec: Vec<Result<Security, IGCError>> = Vec::new();
        let mut file_header_vec: Vec<Result<FileHeader, IGCError>> = Vec::new();
        let mut i_extension_vec: Vec<Result<Extension, IGCError>> = Vec::new();
        let mut j_extension_vec: Vec<Result<Extension, IGCError>> = Vec::new();
        let mut data_fix_vec: Vec<Result<DataFix, IGCError>> = Vec::new();
        let mut comment_vec: Vec<Result<Comment, IGCError>> = Vec::new();
        for line in content.lines() {
            let record = Record::parse(line);
            match line.chars().next() {
                Some(letter) => {
                    match letter {
                        'A' => fr_ids.push(match record {
                            Ok(Record::A(frid)) => Ok(frid),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'B' => fixes.push(match record {
                            Ok(Record::B(fix)) => Ok(fix),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'C' => task_info.push(match record {
                            Ok(Record::C(info)) => Ok(info),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'D' => differential_gps_records.push(match record {
                            Ok(Record::D(diff_gps)) => Ok(diff_gps),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'E' => events.push(match record {
                            Ok(Record::E(event)) => Ok(event),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'F' => satellite_vec.push(match record {
                            Ok(Record::F(sat)) => Ok(sat),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'G' => security_vec.push(match record {
                            Ok(Record::G(sec)) => Ok(sec),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'H' => file_header_vec.push(match record {
                            Ok(Record::H(header)) => Ok(header),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'I' => i_extension_vec.push(match record {
                            Ok(Record::I(ext)) => Ok(ext),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'J' => j_extension_vec.push(match record {
                            Ok(Record::J(ext)) => Ok(ext),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'K' => data_fix_vec.push(match record {
                            Ok(Record::K(data_fix)) => Ok(data_fix),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        'L' => comment_vec.push(match record {
                            Ok(Record::L(comment)) => Ok(comment),
                            Err(error) => Err(error),
                            _ => unreachable!(),
                        }),
                        _ => return Err(IGCFileInitError(format!("{line} does not have a valid start letter"))),
                    }

                }
                None => return Err(IGCFileInitError(format!("{line} does not have a valid start letter"))),
            }
        }

        Ok(Self {
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

    pub fn get_fr_ids(&self) -> &Vec<Result<FlightRecorderID, IGCError>> {
        &self.fr_ids
    }

    pub fn get_fixes(&self) -> &Vec<Result<Fix, IGCError>> {
        &self.fixes
    }

    pub fn get_task_info(&self) -> &Vec<Result<TaskInfo, IGCError>> {
        &self.task_info
    }

    pub fn get_differential_gps_records(&self) -> &Vec<Result<DiffGPS, IGCError>> {
        &self.differential_gps_records
    }

    pub fn get_events(&self) -> &Vec<Result<Event, IGCError>> {
        &self.events
    }

    pub fn get_satellite_vec(&self) -> &Vec<Result<Satellite, IGCError>> {
        &self.satellite_vec
    }

    pub fn get_security_vec(&self) -> &Vec<Result<Security, IGCError>> {
        &self.security_vec
    }

    pub fn get_file_header_vec(&self) -> &Vec<Result<FileHeader, IGCError>> {
        &self.file_header_vec
    }

    pub fn get_i_extension_vec(&self) -> &Vec<Result<Extension, IGCError>> {
        &self.i_extension_vec
    }

    pub fn get_j_extension_vec(&self) -> &Vec<Result<Extension, IGCError>> {
        &self.j_extension_vec
    }

    pub fn get_data_fix_vec(&self) -> &Vec<Result<DataFix, IGCError>> {
        &self.data_fix_vec
    }

    pub fn get_comment_vec(&self) -> &Vec<Result<Comment, IGCError>> {
        &self.comment_vec
    }
}
