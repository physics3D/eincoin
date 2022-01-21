use std::sync::{Arc, Mutex};

use bus::Bus;
use log::warn;
use std::sync::mpsc::Sender;

use crate::{
    blockchain::{Block, Blockchain, Transaction, TransactionOutput, Wallet},
    consts::MINING_REWARD,
    networking::{InternalMessage, MessageType},
};

use super::{middleware::Middleware, Miner};

pub struct MinerMiddleware {
    transactions: Vec<Transaction>,
    miner: Miner,
    wallet: Wallet,
}

impl MinerMiddleware {
    pub fn new(wallet: Wallet) -> Self {
        Self {
            transactions: vec![],
            miner: Miner::new(),
            wallet,
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
            if !transaction.verify(chain) {
                warn!("Received a wrong transaction");
                return;
            }

            self.transactions.push(transaction.clone());
            self.miner.abort();

            let mut new_block = Block::new(
                chain.main_chain().last().unwrap().hash(),
                self.transactions.clone(),
            );

            // add the transaction where the miner gets money
            new_block.transactions.push(
                Transaction::new(
                    MINING_REWARD,
                    0,
                    None,
                    self.wallet.public_key.clone(),
                    chain,
                )
                .unwrap(),
            );

            for transaction in &mut self.transactions {
                let tx_ins_sum = transaction.tx_ins_sum(chain).unwrap();
                let tx_outs_sum = transaction.tx_outs_sum();

                // if there is one, get transaction fee
                if tx_ins_sum > tx_outs_sum {
                    transaction.transaction_outputs.push(TransactionOutput::new(
                        tx_ins_sum - tx_outs_sum,
                        self.wallet.public_key.clone(),
                    ));
                }
            }

            self.miner.mine(new_block, preprocessing_sender.clone());
        }

        if let MessageType::MinedBlock(block) = &message.message.message_type {
            if !chain.push_block(block.clone()) {
                warn!("Received a wrong block");
                return;
            }

            self.transactions.clear();
            self.miner.abort();
        }
    }
}
