use crate::{
    consts::{MINING_REWARD, NEEDED_HASH_START},
    util::{sha256, time_since_unix_epoch},
};
use rand::random;
use serde::{Deserialize, Serialize};

use super::Transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub date: u128,
    pub nonce: u64,
}

impl Block {
    pub fn new(prev_hash: Vec<u8>, transactions: Vec<Transaction>) -> Self {
        Self {
            prev_hash,
            transactions,
            date: time_since_unix_epoch(),
            nonce: Self::generate_nonce(),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(&bincode::serialize(self).unwrap())
    }

    pub fn generate_nonce() -> u64 {
        random()
    }

    pub fn verify_nonce(&self) -> bool {
        self.hash().starts_with(&NEEDED_HASH_START)
    }

    pub fn verify(&self, prev_hash: &[u8]) -> bool {
        self.prev_hash == prev_hash
            && self.verify_nonce()
            && self
                .transactions
                .iter()
                .take(self.transactions.len() - 1)
                .all(|transaction| transaction.verify())
            && self.transactions[self.transactions.len() - 1].amount == MINING_REWARD
    }

    // pub fn mine(&mut self) {
    //     loop {
    //         if self.verify_nonce() {
    //             info!("Solved: {}", self.nonce);
    //             return;
    //         }

    //         self.nonce += 1;
    //     }
    // }
}
