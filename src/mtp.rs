// TODO: Add support for other kinds of problem
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ReportInfo {
    pub status: String,
    pub time: f64,
    pub memory: u64,
}

impl ReportInfo {
    pub fn new(status: &str, time: f64, memory: u64) -> ReportInfo {
        ReportInfo {
            status: String::from(status),
            time,
            memory,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JudgeInfo {
    pub language: String,
    pub source: String,
    pub problem: Problem,
}

impl JudgeInfo {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Problem {
    pub time_limit: f64,
    pub memory_limit: f64,
    pub optimize: bool,
    pub test_cases: Vec<TestCase>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TestCase {
    pub input: String,
    pub answer: String,
}
