use std::sync;

use ana_common::mtp;
use tokio::prelude::*;
use zmq;

pub struct Receiver(zmq::Socket);

impl From<zmq::Socket> for Receiver {
    fn from(socket: zmq::Socket) -> Receiver {
        Receiver(socket)
    }
}

impl Stream for Receiver {
    type Item = mtp::JudgeInfo;
    type Error = ();
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.0.recv_bytes(zmq::DONTWAIT) {
            Ok(res) => match serde_json::from_slice(&res) {
                Ok(res) => Ok(Async::Ready(Some(res))),
                Err(_) => Ok(Async::Ready(None)),
            },
            Err(_) => Ok(Async::NotReady),
        }
    }
}

#[derive(Clone)]
pub struct Sender(sync::Arc<sync::Mutex<zmq::Socket>>);

impl From<zmq::Socket> for Sender {
    fn from(socket: zmq::Socket) -> Sender {
        Sender(sync::Arc::new(sync::Mutex::new(socket)))
    }
}

impl Sender {
    pub fn send_report(&self, report: mtp::ReportInfo) {
        self.0
            .lock()
            .unwrap()
            .send(&report.to_json(), 0)
            .expect("Failed to send the report information");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::io;
    use std::path;

    use uuid::prelude::*;

    pub fn generate_judge_info<T: AsRef<path::Path>>(
        source_file: T,
        problem_file: T,
        spj_source_file: Option<T>,
    ) -> io::Result<mtp::JudgeInfo> {
        let source = mtp::Source {
            language: String::from("cpp.gxx"),
            code: String::from_utf8(fs::read(&source_file)?).unwrap(),
        };
        let mut problem: mtp::Problem = serde_json::from_reader(fs::File::open(&problem_file)?)?;
        if let Some(spj_source_file) = spj_source_file {
            problem.checker = mtp::Source {
                language: String::from("cpp.gxx"),
                code: String::from_utf8(fs::read(&spj_source_file)?).unwrap(),
            };
        }
        Ok(mtp::JudgeInfo {
            id: Uuid::new_v4().to_string(),
            source,
            problem,
        })
    }

    #[test]
    fn test_judge_receiver() -> io::Result<()> {
        let context = zmq::Context::new();
        let sender = {
            let sender = context.socket(zmq::PUSH)?;
            sender.bind("inproc://judge-receiver")?;
            sender
        };
        let mut receiver = {
            let receiver = context.socket(zmq::PULL)?;
            receiver.connect("inproc://judge-receiver")?;
            Receiver::from(receiver)
        };

        assert_eq!(receiver.poll(), Ok(Async::NotReady));

        let judge_info = generate_judge_info("example/source.cpp", "example/problem.json", None)?;
        sender.send(&serde_json::to_string(&judge_info).unwrap(), 0)?;
        assert_eq!(receiver.poll(), Ok(Async::Ready(Some(judge_info))));

        sender.send("hello", 0)?;
        assert_eq!(receiver.poll(), Ok(Async::Ready(None)));

        Ok(())
    }

    #[test]
    fn test_report_sender() -> io::Result<()> {
        let context = zmq::Context::new();
        let sender = {
            let sender = context.socket(zmq::PUSH)?;
            sender.bind("inproc://report-sender")?;
            Sender::from(sender)
        };
        let receiver = {
            let receiver = context.socket(zmq::PULL)?;
            receiver.connect("inproc://report-sender")?;
            receiver
        };

        let report_info =
            mtp::ReportInfo::new("test_report_sender", 0, mtp::JudgeResult::AC, 0.8, 13.6);
        sender.send_report(report_info.clone());

        assert_eq!(receiver.recv_string(0)?.unwrap(), report_info.to_json());

        Ok(())
    }
}