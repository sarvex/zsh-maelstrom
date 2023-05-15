use crate::message::{Res, Req, MessageId, MessageType};
use crate::node::Node;

pub fn handle_broadcast(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::Broadcast);
    let mut res = Res::new(node.id.clone(), req.src.clone());
    res.body.r#type = MessageType::BroadcastOk;
    res.body.in_reply_to = req.body.msg_id;
    let new_item = req.body.message.unwrap();
    if !node.broadcaster.has(new_item) {
        node.broadcaster.broadcast(new_item);
        for neighbour in node.neighbours.iter() {
            let mut broadcast_message = Res::new(node.id.clone(), neighbour.clone());
            let msg_id = node.generator.next();
            broadcast_message.body.msg_id = Some(msg_id);
            broadcast_message.body.r#type = MessageType::Broadcast;
            broadcast_message.body.message = Some(new_item);
            node.sender.send(broadcast_message);
        }
    }
    res.send().unwrap();
}

pub fn handle_broadcast_ok(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::BroadcastOk);
    let in_reply_to = req.body.in_reply_to.unwrap();
    node.sender.ack(&in_reply_to);
}

pub fn handle_read(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::Read);
    let mut res = Res::new(node.id.clone(), req.src.clone());
    res.body.r#type = MessageType::ReadOk;
    res.body.in_reply_to = req.body.msg_id;
    res.body.messages = Some(node.broadcaster.read());
    res.send().unwrap();
}

pub fn handle_topology(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::Topology);
    let mut res = Res::new(node.id.clone(), req.src.clone());
    res.body.in_reply_to = req.body.msg_id;
    res.body.r#type = MessageType::TopologyOk;
    //let topology = req.body.topology.clone().unwrap();
    //node.neighbours = topology.get(&node.id).unwrap().clone();
    node.neighbours = node.peers.clone();
    res.send().unwrap();
}


pub struct Broadcaster {
    messages: Vec<MessageId>,
}

impl Broadcaster {
    pub fn new() -> Broadcaster {
        Broadcaster { 
            messages: Vec::new(),
        }
    }

    pub fn has(&self, msg: MessageId) -> bool {
        self.messages.contains(&msg)
    }

    pub fn broadcast(&mut self, msg: MessageId) {
        self.messages.push(msg);
    }

    pub fn read(&mut self) -> Vec<MessageId> {
        self.messages.clone()
    }
}
