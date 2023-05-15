use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: Body,
}

pub type Res = Message;
pub type Req = Message;

impl Res {
    pub fn send(&self) -> Result<(), String> {
        println!("{}", serde_json::to_string(&self).unwrap());
        Ok(())
    }
}

impl Message {
    pub fn new(src: NodeId, dest: NodeId) -> Message {
        Message {
            src,
            dest,
            body: Body {
                r#type: MessageType::Unknown,
                msg_id: None,
                in_reply_to: None,
                echo: None,
                node_id: None,
                node_ids: None,
                id: None,
                message: None,
                messages: None,
                topology: None,
            },
        }
    }
}

pub type MessageId = u64;
pub type NodeId = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Unknown,
    Init,
    InitOk,
    Echo,
    EchoOk,
    Generate,
    GenerateOk,
    Broadcast,
    BroadcastOk,
    Read,
    ReadOk,
    Topology,
    TopologyOk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub r#type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<NodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<NodeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<HashMap<NodeId, Vec<NodeId>>>,
}


