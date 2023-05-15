mod node;
mod message;
mod input;
mod echo;
mod init;
mod generate;
mod broadcast;
mod sender;

use node::Node;
use message::MessageType;
use input::InputReader;
use echo::handle_echo;
use init::handle_init;
use generate::handle_generate;
use broadcast::{handle_broadcast, handle_broadcast_ok, handle_read, handle_topology};

fn main() {
    let mut node = Node::new();
    node.handle_fn(MessageType::Init, handle_init);
    node.handle_fn(MessageType::Echo, handle_echo);
    node.handle_fn(MessageType::Generate, handle_generate);
    node.handle_fn(MessageType::Broadcast, handle_broadcast);
    node.handle_fn(MessageType::Read, handle_read);
    node.handle_fn(MessageType::Topology, handle_topology);
    node.handle_fn(MessageType::BroadcastOk, handle_broadcast_ok);


    let input_reader = InputReader::new();
    loop {
        let msg = input_reader.read_line();
        node.handle_msg(&msg);
    }
}


