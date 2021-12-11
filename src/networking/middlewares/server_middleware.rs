use std::sync::{Arc, Mutex};

use bus::Bus;
use crossbeam_channel::Sender;
use log::info;

use crate::{
    blockchain::Blockchain,
    networking::{
        message::{MessageDest, MessageSource},
        InternalMessage, MessageType,
    },
};

use super::Middleware;

pub struct ServerMiddleware;

impl Middleware for ServerMiddleware {
    fn on_message(
        &mut self,
        message: &InternalMessage,
        _preprocessing_sender: &Sender<InternalMessage>,
        postprocessing_sender: Arc<Mutex<Bus<InternalMessage>>>,
        chain: &mut Blockchain,
    ) {
        // dont forward connect, sendblockchain and sendblockchainblock messages
        if let MessageType::Connect = message.message.message_type {
            let address = message.source.unwrap();
            let mut sender = postprocessing_sender.lock().unwrap();
            sender.broadcast(InternalMessage::new(
                MessageType::SendBlockchain(chain.chain.len() as u32),
                MessageSource::Localhost,
                MessageDest::Single(address.clone()),
            ));

            for block in &chain.chain {
                sender.broadcast(InternalMessage::new(
                    MessageType::SendBlockchainBlock(block.clone()),
                    MessageSource::Localhost,
                    MessageDest::Single(address.clone()),
                ));
            }
            return;
        }

        if let MessageType::SendBlockchain(_) = message.message.message_type {
            return;
        }
        if let MessageType::SendBlockchainBlock(_) = message.message.message_type {
            return;
        }

        info!(
            "Forwarding a {} message from {} to {}",
            message.message.message_type.to_string(),
            message.source.to_string(),
            message.dest.to_string()
        );

        let mut new_message = message.clone();
        new_message.dest = MessageDest::Broadcast;

        postprocessing_sender.lock().unwrap().broadcast(new_message);
    }
}
