use std::collections::HashMap;
use crate::message::{Res, MessageId};

pub struct Sender {
    outbox: HashMap<MessageId, Res>,
}

impl Sender {
    pub fn new() -> Sender {
        Sender {
            outbox: HashMap::new(),
        }
    }

    pub fn send(&mut self, res: Res) {
        self.outbox.insert(res.body.msg_id.unwrap(), res);
    }

    pub fn ack(&mut self, msg_id: &MessageId) {
        self.outbox.remove(msg_id);
    }

    pub fn send_all(&mut self) {
        for (_, res) in self.outbox.iter_mut() {
            res.send().unwrap();
        }
    }
}
