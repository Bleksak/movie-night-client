use crate::route::RouteKey;

#[derive(Debug, Clone, Copy)]
pub struct Message {
    consumed: bool,
    message_type: MessageType
}

impl Message {
    pub fn new(message_type: MessageType) -> Self {
        Self {
            consumed: false,
            message_type
        }
    }
    
    pub fn consume(&mut self) {
        self.consumed = true;
    }
    
    pub fn consumed(&self) -> bool {
        self.consumed
    }
    
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Increment,
    Decrement,
    RouteChange(RouteKey)
}