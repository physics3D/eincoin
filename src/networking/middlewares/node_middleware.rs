use std::{
    process::exit,
    sync::{Arc, Mutex},
};

use bus::Bus;
use log::{error, info, warn};
use std::sync::mpsc::Sender;

use crate::{
    blockchain::Blockchain,
    networking::{InternalMessage, MessageType},
};

use super::middleware::Middleware;

pub struct NodeMiddleware {
    is_server: bool,
    on_chain_received:
        Box<dyn FnMut(&Sender<InternalMessage>, Arc<Mutex<Bus<InternalMessage>>>, &mut Blockchain)>,
    block_index: usize,
    transaction_index: usize,
    num_blocks_in_chain: usize,
    num_unmined_transactions_in_chain: usize,
}
impl NodeMiddleware {
    pub fn new(
        is_server: bool,
        on_chain_received: impl FnMut(&Sender<InternalMessage>, Arc<Mutex<Bus<InternalMessage>>>, &mut Blockchain)
            + 'static,
    ) -> Self {
        Self {
            is_server,
            block_index: 0,
            transaction_index: 0,
            num_blocks_in_chain: 0,
            num_unmined_transactions_in_chain: 0,
            on_chain_received: Box::new(on_chain_received),
        }
    }
}

impl Middleware for NodeMiddleware {
    fn on_message(
        &mut self,
        message: &InternalMessage,
        preprocessing_sender: &Sender<InternalMessage>,
        postprocessing_sender: Arc<Mutex<Bus<InternalMessage>>>,
        chain: &mut Blockchain,
    ) {
        match &message.message.message_type {
            &MessageType::MinedBlock(_) => {}
            &MessageType::SendBlockchainBlock(_) => {}
            _ => {
                if self.block_index < self.num_blocks_in_chain
                    || self.transaction_index < self.num_unmined_transactions_in_chain
                {
                    warn!("Server takes ages to send the blockchain!");
                }
            }
        }

        match &message.message.message_type {
            MessageType::Connect => {
                if !self.is_server {
                    warn!("The server tried to connect to the client");
                }
            }
            MessageType::SendBlockchain(num_blocks_in_chain, num_unmined_transactions) => {
                self.num_blocks_in_chain = *num_blocks_in_chain;
                self.num_unmined_transactions_in_chain = *num_unmined_transactions;

                info!("Receiving chain...");
            }
            MessageType::SendBlockchainBlock(block) => {
                if !chain.push_block(block.clone()) {
                    warn!("Got a wrong block from the server");
                }

                info!(
                    "Received block {}/{}",
                    self.block_index + 1,
                    self.num_blocks_in_chain
                );

                self.block_index += 1;

                if self.block_index == self.num_blocks_in_chain {
                    info!("Done receiving chain");

                    info!("Verifying chain...");
                    if chain.verify() {
                        info!("Chain is correct");
                    } else {
                        error!("Chain is wrong!");
                        info!("{:#?}", chain);
                        exit(1);
                    }

                    self.block_index = 0;
                    self.num_blocks_in_chain = 0;

                    // weird syntax to run the closure
                    (self.on_chain_received)(preprocessing_sender, postprocessing_sender, chain);
                }
            }
            MessageType::SendBlockchainTransaction(transaction) => {
                chain.unmined_transactions.push(transaction.clone());

                info!(
                    "Received transaction {}/{}",
                    self.transaction_index + 1,
                    self.num_unmined_transactions_in_chain
                );

                self.transaction_index += 1;

                if self.transaction_index == self.num_unmined_transactions_in_chain {
                    info!("Done receiving unmined transactions");
                }
            }
            MessageType::Transaction(_) => {}
            MessageType::MinedBlock(block) => {
                if !chain.push_block(block.clone()) {
                    warn!("Received a wrong mined block");
                    return;
                }

                if !self.is_server {
                    postprocessing_sender
                        .lock()
                        .unwrap()
                        .broadcast(message.clone());
                }
            }
        }
    }
}
