use crate::{
    consts::{MINING_REWARD, NEEDED_HASH_START},
    util::{sha256, time_since_unix_epoch},
};
use rand::random;
use serde::{Deserialize, Serialize};

use super::{Blockchain, Transaction};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub date: u128,
    pub nonce: u64,
    pub children: Vec<Block>,
}

// we can't check with the children in the hash
#[derive(Serialize)]
struct BlockForCheck {
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
            nonce: random(),
            children: vec![],
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(
            &bincode::serialize(&BlockForCheck {
                prev_hash: self.prev_hash.clone(),
                transactions: self.transactions.clone(),
                date: self.date,
                nonce: self.nonce,
            })
            .unwrap(),
        )
    }

    pub fn verify_nonce(&self) -> bool {
        self.hash().starts_with(&NEEDED_HASH_START)
    }

    pub fn verify(&self, prev_hash: &[u8], chain: &Blockchain) -> bool {
        let self_hash = self.hash();

        self.prev_hash == prev_hash
            && self.verify_nonce()
            && self
                .transactions
                .iter()
                .take(self.transactions.len() - 1)
                .all(|transaction| transaction.verify(chain))
            && self.transactions.last().unwrap().transaction_outputs[0].amount == MINING_REWARD
            && self
                .children
                .iter()
                .all(|child| child.verify(&self_hash, chain))
    }

    pub fn push(&mut self, block: &Block, chain: &Blockchain) -> bool {
        if block.prev_hash == self.hash() {
            if block.verify(&self.hash(), chain) {
                self.children.push(block.clone());
                return true;
            } else {
                return false;
            }
        } else {
            for child in &mut self.children {
                if child.push(block, chain) {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_longest_chain(&self) -> Vec<Block> {
        let mut longest_chain = vec![self.clone()];

        longest_chain.append(
            // get sub-chains from children
            &mut self
                .children
                .iter()
                .map(|child| child.get_longest_chain())
                // return the longest sub-chain
                .fold(vec![], |longest_chain, block_longest_chain| {
                    if block_longest_chain.len() >= longest_chain.len() {
                        block_longest_chain
                    } else {
                        longest_chain
                    }
                }),
        );

        longest_chain
    }

    pub fn all_blocks(&self) -> Vec<Block> {
        let mut all_blocks = vec![self.clone()];

        for mut child_all_blocks in self.children.iter().map(|child| child.all_blocks()) {
            all_blocks.append(&mut child_all_blocks);
        }

        all_blocks
    }
}
