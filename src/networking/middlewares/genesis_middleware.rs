use std::sync::{Arc, Mutex};

use bus::Bus;
use crossbeam_channel::Sender;
use log::warn;

use crate::{
    blockchain::Blockchain,
    networking::{InternalMessage, MessageType},
};

use super::{common::verify_and_append_block_to_chain, middleware::Middleware};

pub struct GenesisMiddleware;

impl Middleware for GenesisMiddleware {
    fn on_message(
        &mut self,
        message: &InternalMessage,
        _preprocessing_sender: &Sender<InternalMessage>,
        _postprocessing_sender: Arc<Mutex<Bus<InternalMessage>>>,
        chain: &mut Blockchain,
    ) {
        match &message.message.message_type {
            MessageType::Connect => {}
            MessageType::SendBlockchain(_) => {
                warn!("Someone send the root node a blockchain");
            }
            MessageType::Transaction(_) => {}
            MessageType::MinedBlock(block) => {
                verify_and_append_block_to_chain(chain, block);
            }
            MessageType::SendBlockchainBlock(_) => {
                warn!("Someone send the root node a blockchain block")
            }
        }
    }
}
