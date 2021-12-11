use chrono::Utc;
use log::{error, info};
use rand::rngs::OsRng;
use rsa::errors::Error;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::consts::KEY_PAIR_LENGTH;

use super::{Block, Transaction};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(initial_payee_public_key: RsaPublicKey) -> Self {
        let mut rng = OsRng;
        let genesis = RsaPrivateKey::new(&mut rng, KEY_PAIR_LENGTH)
            .unwrap()
            .to_public_key();

        let mut genesis_block = Block {
            prev_hash: vec![],
            transactions: vec![Transaction {
                amount: 100,
                payer: genesis,
                payee: initial_payee_public_key,
            }],
            date: Utc::now().to_string(),
            nonce: 0,
        };
        genesis_block.mine();

        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn new_empty() -> Self {
        Self { chain: vec![] }
    }

    pub fn last_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn add_block(
        &mut self,
        transactions: Vec<Transaction>,
        sender_public_key: &RsaPublicKey,
        signatures: Vec<Vec<u8>>,
    ) -> Result<(), Error> {
        let is_verified = transactions
            .iter()
            .zip(signatures.iter())
            .all(|(transaction, signature)| transaction.verify(signature));

        if is_verified {
            let mut new_block = Block::new(self.last_block().hash(), transactions);
            new_block.mine();
            self.chain.push(new_block);
            return Ok(());
        }

        error!("wrong signature");
        Err(Error::Verification)
    }

    pub fn verify(&self) -> bool {
        info!("Verifying...");

        for i in 1..self.chain.len() {
            if self.chain[i].hash() != self.chain[i - 1].prev_hash {
                return false;
            }

            if !self.chain[i].verify() {
                return false;
            }
        }

        return true;
    }

    pub fn verify_new_block(&self, block: &Block) -> bool {
        if block.prev_hash != self.last_block().hash() {
            return false;
        }

        if !block.verify() {
            return false;
        }

        return true;
    }

    pub fn push_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}
