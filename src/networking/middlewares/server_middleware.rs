use std::sync::{Arc, Mutex};

use bus::Bus;
use log::info;
use std::sync::mpsc::Sender;

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
            let all_blocks = chain.all_blocks();

            let address = message.source.unwrap();
            let mut sender = postprocessing_sender.lock().unwrap();
            sender.broadcast(InternalMessage::new(
                MessageType::SendBlockchain(all_blocks.len(), chain.unmined_transactions.len()),
                MessageSource::Localhost,
                MessageDest::Single(address.clone()),
            ));

            for mut block in all_blocks {
                block.children = vec![];
                sender.broadcast(InternalMessage::new(
                    MessageType::SendBlockchainBlock(block),
                    MessageSource::Localhost,
                    MessageDest::Single(address.clone()),
                ));
            }

            for transaction in &chain.unmined_transactions {
                sender.broadcast(InternalMessage::new(
                    MessageType::SendBlockchainTransaction(transaction.clone()),
                    MessageSource::Localhost,
                    MessageDest::Single(address.clone()),
                ));
            }

            return;
        }

        if let MessageType::SendBlockchain(_, _) = message.message.message_type {
            return;
        }
        if let MessageType::SendBlockchainBlock(_) = message.message.message_type {
            return;
        }

        info!(
            "Forwarding a {} message",
            message.message.message_type.to_string()
        );

        let mut new_message = message.clone();
        new_message.dest = MessageDest::Broadcast;

        postprocessing_sender.lock().unwrap().broadcast(new_message);
    }
}
