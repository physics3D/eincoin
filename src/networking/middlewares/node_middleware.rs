use std::sync::{Arc, Mutex};

use bus::Bus;
use crossbeam_channel::Sender;
use log::{info, warn};

use crate::{
    blockchain::Blockchain,
    networking::{InternalMessage, MessageType},
};

use super::{common::verify_and_append_block_to_chain, middleware::Middleware};

pub struct NodeMiddleware {
    is_server: bool,
    on_chain_received:
        Box<dyn FnMut(&Sender<InternalMessage>, Arc<Mutex<Bus<InternalMessage>>>, &mut Blockchain)>,
    index: u32,
    num_blocks_in_chain: u32,
}
impl NodeMiddleware {
    pub fn new(
        is_server: bool,
        on_chain_received: impl FnMut(&Sender<InternalMessage>, Arc<Mutex<Bus<InternalMessage>>>, &mut Blockchain)
            + 'static,
    ) -> Self {
        Self {
            is_server,
            index: 0,
            num_blocks_in_chain: 0,
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
            MessageType::Connect => {
                if !self.is_server {
                    warn!("The server tried to connect to the client");
                }
            }
            MessageType::SendBlockchain(num_blocks_in_chain) => {
                self.num_blocks_in_chain = *num_blocks_in_chain;

                info!("Receiving chain...");
            }
            MessageType::SendBlockchainBlock(block) => {
                chain.chain.push(block.clone());

                info!(
                    "Received block {}/{}",
                    self.index + 1,
                    self.num_blocks_in_chain
                );

                self.index += 1;

                if self.index == self.num_blocks_in_chain {
                    info!("Done receiving chain");

                    self.index = 0;
                    self.num_blocks_in_chain = 0;

                    // weird syntax to run the closure
                    (self.on_chain_received)(preprocessing_sender, postprocessing_sender, chain);
                }
            }
            MessageType::Transaction(_) => {}
            MessageType::MinedBlock(block) => {
                verify_and_append_block_to_chain(chain, block);
            }
        }
    }
}
