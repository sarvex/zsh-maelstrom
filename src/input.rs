use crate::message::Message;
use std::io;

pub struct InputReader {
}

impl InputReader {
    pub fn new() -> InputReader {
        InputReader {}
    }

    pub fn read_line(&self) -> Message {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        serde_json::from_str(&input).unwrap()
    }
}

