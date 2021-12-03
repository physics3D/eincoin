use crate::consts::NEEDED_HASH_START;
use rand::random;

use crate::{transaction::Transaction, util::sha256};

#[derive(Debug)]
pub struct Block {
    pub prev_hash: Vec<u8>,
    pub transaction: Transaction,
    pub date: String,
    pub nonce: u64,
}

impl Block {
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
                println!("Solved: {}", self.nonce);
                return;
            }

            self.nonce += 1;
        }
    }
}
