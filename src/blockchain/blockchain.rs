use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};

use crate::consts::INITIAL_COIN_AMOUNT;

use super::{Block, Transaction};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub chain: Option<Block>,
    pub unmined_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(initial_payee_public_key: RsaPublicKey) -> Self {
        let genesis_block = Block::new(
            vec![],
            vec![Transaction::new(
                INITIAL_COIN_AMOUNT,
                None,
                initial_payee_public_key,
            )],
        );

        Self {
            chain: Some(genesis_block),
            unmined_transactions: vec![],
        }
    }

    pub fn new_empty() -> Self {
        Self {
            chain: None,
            unmined_transactions: vec![],
        }
    }

    pub fn verify(&self) -> bool {
        if let Some(root) = &self.chain {
            // we can't verify the root block, so we verify its children manually
            let root_hash = root.hash();
            root.children.iter().all(|child| child.verify(&root_hash))
        } else {
            true
        }
    }

    pub fn push_block(&mut self, block: Block) -> bool {
        if let Some(root) = &mut self.chain {
            root.push(&block)
        } else {
            self.chain = Some(block);
            true
        }
    }

    pub fn main_chain(&self) -> Vec<Block> {
        if let Some(root) = &self.chain {
            root.get_longest_chain()
        } else {
            vec![]
        }
    }

    pub fn all_blocks(&self) -> Vec<Block> {
        if let Some(root) = &self.chain {
            root.all_blocks()
        } else {
            vec![]
        }
    }
}
