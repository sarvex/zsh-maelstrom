use crate::message::{Req, Res, MessageType};
use crate::node::Node;

pub fn handle_init(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::Init);
    node.id = req.body.node_id.clone().unwrap();
    node.peers = req.body.node_ids.clone().unwrap();
    let index_of = match node.peers.iter().position(|x| x == &node.id) {
        Some(index) => index,
        None => panic!("Node id not found in peers"),
    };
    node.generator.set(index_of, node.peers.len());

    let mut res = Res::new(node.id.clone(), req.src.clone());
    res.src = node.id.clone();
    res.body.r#type = MessageType::InitOk;
    res.body.in_reply_to = req.body.msg_id;
    res.send().unwrap();
}

