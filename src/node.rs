use crate::message::{Req, MessageType, NodeId};
use crate::generate::Generator;
use crate::broadcast::Broadcaster;
use crate::sender::Sender;
use std::collections::HashMap;

pub struct Node {
    pub id: NodeId,
    pub peers: Vec<NodeId>,
    pub handlers: HashMap<MessageType, fn(&mut Node, &Req)>,
    pub generator: Generator,
    pub broadcaster: Broadcaster,
    pub sender: Sender,
    pub neighbours: Vec<NodeId>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: String::new(),
            peers: Vec::new(),
            handlers: HashMap::new(),
            generator: Generator::new(),
            broadcaster: Broadcaster::new(),
            sender: Sender::new(),
            neighbours: Vec::new(),
        }
    }

    pub fn handle_fn(&mut self, msg_type: MessageType, handler: fn(&mut Node, &Req)) {
        self.handlers.insert(msg_type, handler);
    }
    
    pub fn handle_msg(&mut self, req: &Req) {
        match self.handlers.get(&req.body.r#type) {
            Some(handler) => {
                handler(self, req)
            }
            None => panic!("No handler for message type {:?}", req.body.r#type),
        }
    }
}


