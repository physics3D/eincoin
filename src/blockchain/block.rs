use crate::{consts::NEEDED_HASH_START, util::sha256};
use chrono::Utc;
use log::info;
use rand::random;
use serde::{Deserialize, Serialize};

use super::Transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub date: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(prev_hash: Vec<u8>, transactions: Vec<Transaction>) -> Self {
        Self {
            prev_hash,
            transactions,
            date: Utc::now().to_string(),
            nonce: Self::generate_nonce(),
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(self.to_string().as_bytes())
    }

    pub fn generate_nonce() -> u64 {
        random()
    }

    fn to_string(&self) -> String {
        format!("{:?}", &self)
    }

    pub fn verify(&self) -> bool {
        self.hash().starts_with(&NEEDED_HASH_START)
    }

    pub fn mine(&mut self) {
        loop {
            if self.verify() {
                info!("Solved: {}", self.nonce);
                return;
            }

            self.nonce += 1;
        }
    }
}
