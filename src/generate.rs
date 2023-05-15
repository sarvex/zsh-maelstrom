use crate::message::{Res, Req, MessageId, MessageType};
use crate::node::Node;

pub fn handle_generate(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::Generate);
    let mut res = Res::new(node.id.clone(), req.src.clone());
    res.body.r#type = MessageType::GenerateOk;
    res.body.msg_id = Some(0);
    res.body.id = Some(node.generator.next());
    res.body.in_reply_to = req.body.msg_id;
    res.send().unwrap();
}

pub struct Generator {
    counter: u64,
}

impl Generator {
    pub fn new() -> Generator {
        Generator { 
            counter: 0,
        }
    }

    pub fn set(&mut self, index: usize, nb_nodes: usize) {
        let max = std::u64::MAX/nb_nodes as u64;
        self.counter = max * index as u64;
    }

    pub fn next(&mut self) -> MessageId {
        self.counter += 1;
        self.counter
    }
}


