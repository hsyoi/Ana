use std::fmt;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ReportInfo {
    pub id: String,
    pub index: usize,
    pub status: String,
    pub time: f64,
    pub memory: f64,
}

impl ReportInfo {
    pub fn new(id: &str, index: usize, status: JudgeResult, time: f64, memory: f64) -> ReportInfo {
        ReportInfo {
            id: String::from(id),
            index,
            status: status.as_str().to_string(),
            time,
            memory,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[repr(i8)]
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum JudgeResult {
    CE = -1,
    AC = 0,
    WA = 1,
    TLE = 2,
    MLE = 3,
    OLE = 4,
    RE = 5,
}

impl JudgeResult {
    fn as_str(self) -> &'static str {
        match self {
            JudgeResult::AC => "AC",
            JudgeResult::CE => "CE",
            JudgeResult::MLE => "MLE",
            JudgeResult::OLE => "OLE",
            JudgeResult::RE => "RE",
            JudgeResult::TLE => "TLE",
            JudgeResult::WA => "WA",
        }
    }
}

impl fmt::Display for JudgeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}