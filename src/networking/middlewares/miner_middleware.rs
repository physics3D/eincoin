use std::sync::{Arc, Mutex};

use bus::Bus;
use std::sync::mpsc::Sender;

use crate::{
    blockchain::{Block, Blockchain, Transaction, Wallet},
    networking::{InternalMessage, MessageType},
};

use super::{middleware::Middleware, Miner};

pub struct MinerMiddleware {
    transactions: Vec<Transaction>,
    miner: Miner,
}

impl MinerMiddleware {
    pub fn new(wallet: Wallet) -> Self {
        Self {
            transactions: vec![],
            miner: Miner::new(wallet),
        }
    }
}

impl Middleware for MinerMiddleware {
    fn on_message(
        &mut self,
        message: &InternalMessage,
        preprocessing_sender: &Sender<InternalMessage>,
        _postprocessing_sender: Arc<Mutex<Bus<InternalMessage>>>,
        chain: &mut Blockchain,
    ) {
        if let MessageType::Transaction(transaction) = &message.message.message_type {
            self.transactions.push(transaction.clone());
            self.miner.abort();

            let new_block = Block::new(chain.last_block().hash(), self.transactions.clone());
            self.miner.mine(new_block, preprocessing_sender.clone());
        }

        if let MessageType::MinedBlock(_) = &message.message.message_type {
            self.transactions.clear();
            self.miner.abort();
        }
    }
}
