use crate::message::{ Req, Res, MessageType};
use crate::node::Node;

pub fn handle_echo(node: &mut Node, req: &Req) {
    assert_eq!(req.body.r#type, MessageType::Echo);
    let mut res = Res::new(node.id.clone(), req.src.clone());
    res.body.r#type = MessageType::EchoOk;
    res.body.msg_id = Some(0);
    res.body.in_reply_to = req.body.msg_id;
    res.body.echo = req.body.echo.clone();
    res.send().unwrap();
}

