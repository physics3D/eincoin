use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};

use crate::consts::INITIAL_COIN_AMOUNT;

use super::{Block, Transaction, TransactionInput, TransactionOutput};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub chain: Option<Block>,
    pub unmined_transactions: Vec<Transaction>,
    pub utxos: Vec<(Vec<u8>, u32, TransactionOutput)>,
}

impl Blockchain {
    pub fn new(initial_payee_public_key: RsaPublicKey) -> Self {
        let mut blockchain = Self::new_empty();

        blockchain.chain = Some(Block::new(
            vec![],
            vec![Transaction::new(
                INITIAL_COIN_AMOUNT,
                None,
                initial_payee_public_key,
                &mut blockchain,
            )
            .unwrap()],
        ));

        blockchain
    }

    pub fn new_empty() -> Self {
        Self {
            chain: None,
            unmined_transactions: vec![],
            utxos: vec![],
        }
    }

    pub fn verify(&self) -> bool {
        if let Some(root) = &self.chain {
            // we can't verify the root block, so we verify its children manually
            let root_hash = root.hash();
            root.children
                .iter()
                .all(|child| child.verify(&root_hash, &self))
        } else {
            true
        }
    }

    pub fn push_block(&mut self, block: Block) -> bool {
        let tx_ins = block
            .transactions
            .iter()
            .flat_map(|transaction| transaction.transaction_inputs.iter())
            .map(|tx_in| tx_in.clone())
            .collect();

        let success = if let Some(root) = &mut self.chain.clone() {
            root.push(&block, &self)
        } else {
            self.chain = Some(block);
            true
        };

        if success {
            self.update_utxos(tx_ins);
        }

        success
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

    pub fn compute_utxos(&mut self) {
        let all_tx_outs = self.all_blocks();
        let all_tx_outs: Vec<_> = all_tx_outs
            .iter()
            .flat_map(|block| {
                block.transactions.iter().flat_map(|transaction| {
                    let hash = transaction.hash();

                    transaction
                        .transaction_outputs
                        .iter()
                        .enumerate()
                        .map(move |(i, tx_out)| (hash.clone(), i, tx_out))
                })
            })
            .collect();

        let all_tx_ins: Vec<_> = self
            .all_blocks()
            .iter()
            .flat_map(|block| {
                block.transactions.iter().flat_map(|transaction| {
                    transaction.transaction_inputs.iter().map(|tx_in| {
                        (
                            tx_in.prev_transaction_hash.clone(),
                            tx_in.prev_transaction_index,
                        )
                    })
                })
            })
            .collect();

        self.utxos = all_tx_outs
            .iter()
            .filter(|tx_out| !all_tx_ins.contains(&(tx_out.0.clone(), tx_out.1 as u32)))
            .map(|(hash, i, utxo)| (hash.clone(), *i as u32, (*utxo).clone()))
            .collect();
    }

    pub fn update_utxos(&mut self, transaction_inputs: Vec<TransactionInput>) {
        let tx_ins: Vec<_> = transaction_inputs
            .iter()
            .map(|tx_in| {
                (
                    tx_in.prev_transaction_hash.clone(),
                    tx_in.prev_transaction_index,
                )
            })
            .collect();

        self.utxos = self
            .utxos
            .iter()
            .filter(|tx_out| !tx_ins.contains(&(tx_out.0.clone(), tx_out.1 as u32)))
            .map(|(hash, i, utxo)| (hash.clone(), *i as u32, (*utxo).clone()))
            .collect();
    }
}
