use crate::error::IGCError;
use crate::error::IGCError::IGCFileInitError;
use crate::records::comment::Comment;
use crate::records::data_fix::DataFix;
use crate::records::diff_gps::DiffGPS;
use crate::records::event::Event;
use crate::records::extension::Extension;
use crate::records::file_header::FileHeader;
use crate::records::fix::Fix;
use crate::records::flight_recorder_id::FlightRecorderID;
use crate::records::Record;
use crate::records::satellite::Satellite;
use crate::records::security::Security;
use crate::records::task_info::TaskInfo;

pub struct Parsed {
    fr_ids: Option<Vec<Result<FlightRecorderID, IGCError>>>,
    fixes: Option<Vec<Result<Fix, IGCError>>>,
    task_info: Option<Vec<Result<TaskInfo, IGCError>>>,
    differential_gps_records: Option<Vec<Result<DiffGPS, IGCError>>>,
    events: Option<Vec<Result<Event, IGCError>>>,
    satellite_vec: Option<Vec<Result<Satellite, IGCError>>>,
    security_vec: Option<Vec<Result<Security, IGCError>>>,
    file_header_vec: Option<Vec<Result<FileHeader, IGCError>>>,
    i_extension_vec: Option<Vec<Result<Extension, IGCError>>>,
    j_extension_vec: Option<Vec<Result<Extension, IGCError>>>,
    data_fix_vec: Option<Vec<Result<DataFix, IGCError>>>,
    comment_vec: Option<Vec<Result<Comment, IGCError>>>,
}

impl Parsed {
    pub fn get_fr_ids(&self) -> Option<&Vec<Result<FlightRecorderID, IGCError>>> {
        self.fr_ids.as_ref()
    }

    pub fn get_fixes(&self) -> Option<&Vec<Result<Fix, IGCError>>> {
        self.fixes.as_ref()
    }

    pub fn get_task_info(&self) -> Option<&Vec<Result<TaskInfo, IGCError>>> {
        self.task_info.as_ref()
    }

    pub fn get_differential_gps_records(&self) -> Option<&Vec<Result<DiffGPS, IGCError>>> {
        self.differential_gps_records.as_ref()
    }

    pub fn get_events(&self) -> Option<&Vec<Result<Event, IGCError>>> {
        self.events.as_ref()
    }

    pub fn get_satellite_vec(&self) -> Option<&Vec<Result<Satellite, IGCError>>> {
        self.satellite_vec.as_ref()
    }

    pub fn get_security_vec(&self) -> Option<&Vec<Result<Security, IGCError>>> {
        self.security_vec.as_ref()
    }

    pub fn get_file_header_vec(&self) -> Option<&Vec<Result<FileHeader, IGCError>>> {
        self.file_header_vec.as_ref()
    }

    pub fn get_fix_extension_vec(&self) -> Option<&Vec<Result<Extension, IGCError>>> {
        self.i_extension_vec.as_ref()
    }

    pub fn get_data_fix_extension_vec(&self) -> Option<&Vec<Result<Extension, IGCError>>> {
        self.j_extension_vec.as_ref()
    }

    pub fn get_data_fix_vec(&self) -> Option<&Vec<Result<DataFix, IGCError>>> {
        self.data_fix_vec.as_ref()
    }

    pub fn get_comment_vec(&self) -> Option<&Vec<Result<Comment, IGCError>>> {
        self.comment_vec.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct ParserBuilder {
    fr_id: bool,
    fixes: bool,
    task_info: bool,
    differential_gps_records: bool, 
    events: bool,
    satellite: bool,
    security: bool,
    file_header: bool,
    i_extension: bool,
    j_extension: bool,
    data_fix: bool,
    comments: bool,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserBuilder {
    pub fn on_file(self, content: &str) -> Result<Parsed, IGCError> {
        let mut fr_ids: Option<Vec<Result<FlightRecorderID, IGCError>>> = match self.fr_id {
            true => Some(Vec::new()),
            false => None,
        };
        let mut fixes: Option<Vec<Result<Fix, IGCError>>> = match self.fixes {
            true => Some(Vec::new()),
            false => None,
        };
        let mut task_info: Option<Vec<Result<TaskInfo, IGCError>>> = match self.task_info {
            true => Some(Vec::new()),
            false => None,
        };
        let mut differential_gps_records: Option<Vec<Result<DiffGPS, IGCError>>> = match self.differential_gps_records {
            true => Some(Vec::new()),
            false => None,
        };
        let mut events: Option<Vec<Result<Event, IGCError>>> = match self.events {
            true => Some(Vec::new()),
            false => None,
        };
        let mut satellite_vec: Option<Vec<Result<Satellite, IGCError>>> = match self.satellite {
            true => Some(Vec::new()),
            false => None,
        };
        let mut security_vec: Option<Vec<Result<Security, IGCError>>> = match self.security {
            true => Some(Vec::new()),
            false => None,
        };
        let mut file_header_vec: Option<Vec<Result<FileHeader, IGCError>>> = match self.file_header {
            true => Some(Vec::new()),
            false => None,
        };
        let mut i_extension_vec: Option<Vec<Result<Extension, IGCError>>> = match self.i_extension {
            true => Some(Vec::new()),
            false => None,
        };
        let mut j_extension_vec: Option<Vec<Result<Extension, IGCError>>> = match self.j_extension {
            true => Some(Vec::new()),
            false => None,
        };
        let mut data_fix_vec: Option<Vec<Result<DataFix, IGCError>>> = match self.data_fix {
            true => Some(Vec::new()),
            false => None,
        };
        let mut comment_vec: Option<Vec<Result<Comment, IGCError>>> = match self.comments {
            true => Some(Vec::new()),
            false => None,
        };
        for line in content.lines() {
            let record = Record::parse(line);
            match line.chars().next() {
                Some(letter) => {
                    match letter {
                        'A' =>
                            if let Some(fr_ids) = &mut fr_ids {
                                fr_ids.push(match record {
                                    Ok(Record::A(frid)) => Ok(frid),
                                    Err(error) => Err(error),
                                    _ => unreachable!(),
                                })
                            },
                        'B' =>
                            if let Some(fixes) = &mut fixes {
                            fixes.push(match record {
                                Ok(Record::B(fix)) => Ok(fix),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'C' =>
                            if let Some(task_info) = &mut task_info {
                            task_info.push(match record {
                                Ok(Record::C(info)) => Ok(info),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'D' =>
                            if let Some(differential_gps_records) = &mut differential_gps_records {
                            differential_gps_records.push(match record {
                                Ok(Record::D(diff_gps)) => Ok(diff_gps),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'E' =>
                            if let Some(events) = &mut events {
                            events.push(match record {
                                Ok(Record::E(event)) => Ok(event),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'F' =>
                            if let Some(satellite_vec) = &mut satellite_vec {
                            satellite_vec.push(match record {
                                Ok(Record::F(sat)) => Ok(sat),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'G' => if let Some(security_vec) = &mut security_vec {
                            security_vec.push(match record {
                                Ok(Record::G(sec)) => Ok(sec),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'H' => if let Some(file_header_vec) = &mut file_header_vec {
                            file_header_vec.push(match record {
                                Ok(Record::H(header)) => Ok(header),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'I' => if let Some(i_extension_vec) = &mut i_extension_vec {
                            i_extension_vec.push(match record {
                                Ok(Record::I(ext)) => Ok(ext),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'J' => if let Some(j_extension_vec) = &mut j_extension_vec {
                            j_extension_vec.push(match record {
                                Ok(Record::J(ext)) => Ok(ext),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'K' => if let Some(data_fix_vec) = &mut data_fix_vec {
                            data_fix_vec.push(match record {
                                Ok(Record::K(data_fix)) => Ok(data_fix),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })
                        },
                        'L' => if let Some(comment_vec) = &mut comment_vec {
                            comment_vec.push(match record {
                                Ok(Record::L(comment)) => Ok(comment),
                                Err(error) => Err(error),
                                _ => unreachable!(),
                            })},
                        _ => return Err(IGCFileInitError(format!("{line} does not have a valid start letter"))),
                    }

                }
                None => return Err(IGCFileInitError(format!("{line} does not have a valid start letter"))),
            }
        }
        Ok(
            Parsed {
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
            }
        )
    }

    pub fn new() -> Self {
        ParserBuilder {
            fr_id: false,
            fixes: false,
            task_info: false,
            differential_gps_records: false,
            events: false,
            satellite: false,
            security: false,
            file_header: false,
            i_extension: false,
            j_extension: false,
            data_fix: false,
            comments: false,
        }
    }

    pub fn parse_flight_record_id(mut self) -> Self {
        self.fr_id = true;
        self
    }

    pub fn parse_fixes(mut self) -> Self {
        self.fixes = true;
        self
    }

    pub fn parse_task_info(mut self) -> Self {
        self.task_info = true;
        self
    }

    pub fn parse_differential_gps_records(mut self) -> Self {
        self.differential_gps_records = true;
        self
    }

    pub fn parse_events(mut self) -> Self {
        self.events = true;
        self
    }

    pub fn parse_satellite(mut self) -> Self {
        self.satellite = true;
        self
    }

    pub fn parse_security(mut self) -> Self {
        self.security = true;
        self
    }

    pub fn parse_file_header(mut self) -> Self {
        self.file_header = true;
        self
    }

    pub fn parse_fix_extension(mut self) -> Self {
        self.i_extension = true;
        self
    }

    pub fn parse_data_fix_extension(mut self) -> Self {
        self.j_extension = true;
        self
    }

    pub fn parse_data_fix(mut self) -> Self {
        self.data_fix = true;
        self
    }

    pub fn parse_comments(mut self) -> Self {
        self.comments = true;
        self
    }

    /// Alias method for parse flight record id
    pub fn parse_a_records(self) -> Self {
        self.parse_flight_record_id()
    }

    /// Alias method for parse fixes
    pub fn parse_b_records(self) -> Self {
        self.parse_fixes()
    }

    /// Alias method for parse task info
    pub fn parse_c_records(self) -> Self {
        self.parse_task_info()
    }

    /// Alias method for parse diff gps
    pub fn parse_d_records(self) -> Self {
        self.parse_differential_gps_records()
    }

    /// Alias method for parse events
    pub fn parse_e_records(self) -> Self {
        self.parse_events()
    }

    /// Alias method for parse satellite
    pub fn parse_f_records(self) -> Self {
        self.parse_satellite()
    }

    /// Alias method for parse security
    pub fn parse_g_records(self) -> Self {
        self.parse_security()
    }

    /// Alias method for parse file header
    pub fn parse_h_records(self) -> Self {
        self.parse_file_header()
    }

    /// Alias method for parse fix extension
    pub fn parse_i_records(self) -> Self {
        self.parse_fix_extension()
    }

    /// Alias method for parse data fix extension
    pub fn parse_j_records(self) -> Self {
        self.parse_data_fix_extension()
    }

    /// Alias method for parse data fix
    pub fn parse_k_records(self) -> Self {
        self.parse_data_fix()
    }

    /// Alias method for parse comments
    pub fn parse_l_records(self) -> Self {
        self.parse_comments()
    }
}
