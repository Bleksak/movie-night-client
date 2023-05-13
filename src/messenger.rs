use std::collections::HashMap;

use crate::message::{MessageType, Message};

type MessengerClosure = Box<dyn Fn(Message)>;

struct Messenger<Msg = MessageType> {
    handlers: HashMap<Msg, MessengerClosure>,
}