use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};

use crate::util::sha256;

use super::{Blockchain, TransactionInput, TransactionOutput, Wallet};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub transaction_inputs: Vec<TransactionInput>,
    pub transaction_outputs: Vec<TransactionOutput>,
}

// extra struct only to check the signature (we can't check with the signature IN the struct)
#[derive(Serialize)]
struct TransactionForCheck {
    pub transaction_inputs: Vec<TransactionInput>,
    pub transaction_outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(
        amount: u32,
        transaction_fee: u32,
        wallet: Option<Wallet>,
        payee: RsaPublicKey,
        chain: &mut Blockchain,
    ) -> Result<Self, String> {
        let total_to_pay = amount + transaction_fee;

        let mut transaction = Self {
            transaction_inputs: vec![],
            transaction_outputs: vec![],
        };

        if let Some(keypair) = wallet {
            let mut utxos: Vec<_> = chain
                .utxos
                .iter()
                .filter(|(_, _, tx_out)| tx_out.payee == keypair.public_key)
                .collect();

            utxos.sort_by_key(|(_, _, tx_out)| tx_out.amount);

            let mut utxos_iter = utxos.iter();
            let mut total_amount = 0;

            loop {
                let (hash, index, utxo) = match utxos_iter.next() {
                    Some(value) => value,
                    None => break,
                };

                total_amount += utxo.amount;

                transaction.transaction_inputs.push(TransactionInput::new(
                    hash.clone(),
                    *index as u32,
                    Some(utxo.payee.clone()),
                    &keypair.private_key,
                ));
            }

            if total_amount < total_to_pay {
                return Err("You do not have enough money in this wallet".to_string());
            }

            transaction
                .transaction_outputs
                .push(TransactionOutput::new(amount, payee));

            // change transaction output
            if total_amount > total_to_pay {
                transaction.transaction_outputs.push(TransactionOutput::new(
                    total_amount - total_to_pay,
                    keypair.public_key.clone(),
                ));
            }
        } else {
            transaction.transaction_inputs.push(TransactionInput {
                prev_transaction_hash: vec![],
                prev_transaction_index: 0,
                signature: vec![],
                payer: None,
            });

            transaction
                .transaction_outputs
                .push(TransactionOutput::new(amount, payee));
        }

        Ok(transaction)
    }

    pub fn tx_ins_sum(&self, chain: &Blockchain) -> Option<u32> {
        let mut tx_in_sum = 0;

        for tx_in in &self.transaction_inputs {
            match tx_in.get_used_tx_out(chain) {
                Some(tx_out) => tx_in_sum += tx_out.amount,
                None => return None,
            }
        }

        Some(tx_in_sum)
    }

    pub fn tx_outs_sum(&self) -> u32 {
        self.transaction_outputs
            .iter()
            .map(|tx_out| tx_out.amount)
            .sum()
    }

    pub fn verify(&self, chain: &Blockchain) -> bool {
        let tx_ins_sum = self.tx_ins_sum(chain);

        if tx_ins_sum.is_none() {
            return false;
        }

        self.tx_outs_sum() <= tx_ins_sum.unwrap()
            && self.transaction_inputs.iter().all(|tx_in| tx_in.verify())
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(
            &bincode::serialize(&TransactionForCheck {
                transaction_inputs: self.transaction_inputs.clone(),
                transaction_outputs: self.transaction_outputs.clone(),
            })
            .unwrap(),
        )
    }
}
