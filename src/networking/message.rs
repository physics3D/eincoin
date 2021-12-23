use serde::{Deserialize, Serialize};

use crate::{
    blockchain::{Block, Transaction},
    util::time_since_unix_epoch,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    Connect,
    SendBlockchain(usize),
    SendBlockchainBlock(Block),
    Transaction(Transaction),
    MinedBlock(Block),
}

impl MessageType {
    pub fn to_string(&self) -> String {
        match self {
            MessageType::Connect => "Connect",
            MessageType::SendBlockchain(_) => "SendBlockchain",
            MessageType::Transaction(_) => "Transaction",
            MessageType::MinedBlock(_) => "MinedBlock",
            MessageType::SendBlockchainBlock(_) => "SendBlockchainBlock",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_type: MessageType,
    pub timestamp: u128,
}

impl Message {
    pub fn new(message_type: MessageType) -> Self {
        Self {
            message_type,
            timestamp: time_since_unix_epoch(),
        }
    }
}

#[derive(Clone)]
pub enum MessageSource {
    Localhost,
    Foreign(String),
}

impl MessageSource {
    pub fn to_string(&self) -> String {
        match self {
            MessageSource::Localhost => "Localhost",
            MessageSource::Foreign(_) => "Foreign",
        }
        .to_string()
    }

    pub fn unwrap(&self) -> String {
        match self {
            MessageSource::Localhost => "",
            MessageSource::Foreign(addr) => addr,
        }
        .to_string()
    }
}

#[derive(Clone)]
pub enum MessageDest {
    Localhost,
    Broadcast,
    Single(String),
}

impl MessageDest {
    pub fn to_string(&self) -> String {
        match self {
            MessageDest::Localhost => "Localhost",
            MessageDest::Broadcast => "Broadcast",
            MessageDest::Single(_) => "Single",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct InternalMessage {
    pub message: Message,
    pub source: MessageSource,
    pub dest: MessageDest,
}

impl InternalMessage {
    pub fn new(message_type: MessageType, source: MessageSource, dest: MessageDest) -> Self {
        Self::from_message(Message::new(message_type), source, dest)
    }

    pub fn from_message(message: Message, source: MessageSource, dest: MessageDest) -> Self {
        Self {
            message,
            source,
            dest,
        }
    }

    pub fn should_be_send_to(&self, address: &str) -> bool {
        // don't send messages to where they came from
        if let MessageSource::Foreign(addr) = &self.source {
            if addr == address {
                return false;
            }
        }

        // check destination if it should only be send to one address
        if let MessageDest::Single(addr) = &self.dest {
            if addr != address {
                return false;
            }
        }

        true
    }
}
