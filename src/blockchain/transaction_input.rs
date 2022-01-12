use rsa::{Hash, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::util::sha256;

use super::{Blockchain, TransactionOutput};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionInput {
    pub prev_transaction_hash: Vec<u8>,
    pub prev_transaction_index: u32,
    pub payer: Option<RsaPublicKey>,
    pub signature: Vec<u8>,
}

#[derive(Serialize)]
struct TransactionInputForCheck {
    pub prev_transaction_hash: Vec<u8>,
    pub prev_transaction_index: u32,
    pub payer: Option<RsaPublicKey>,
}

impl TransactionInput {
    pub fn new(
        prev_transaction_hash: Vec<u8>,
        prev_transaction_index: u32,
        payer: Option<RsaPublicKey>,
        sign_key: &RsaPrivateKey,
    ) -> Self {
        let mut tx_in = TransactionInput {
            prev_transaction_hash,
            prev_transaction_index,
            payer,
            signature: vec![],
        };

        tx_in.signature = sign_key
            .sign(
                PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                &tx_in.hash(),
            )
            .unwrap();

        tx_in
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(
            &bincode::serialize(&TransactionInputForCheck {
                prev_transaction_hash: self.prev_transaction_hash.clone(),
                prev_transaction_index: self.prev_transaction_index,
                payer: self.payer.clone(),
            })
            .unwrap(),
        )
    }

    pub fn get_used_tx_out(&self, chain: &Blockchain) -> Option<TransactionOutput> {
        let all_tx_outs = chain.all_blocks();
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

        let matching_tx_outs: Vec<_> = all_tx_outs
            .iter()
            .filter(|(hash, index, _)| {
                *hash == self.prev_transaction_hash && *index as u32 == self.prev_transaction_index
            })
            .map(|(_, _, tx_out)| tx_out)
            .collect();

        if matching_tx_outs.len() != 1 {
            None
        } else {
            Some((*matching_tx_outs[0]).clone())
        }
    }

    pub fn verify(&self) -> bool {
        match &self.payer {
            Some(verify_key) => verify_key
                .verify(
                    PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                    &self.hash(),
                    &self.signature,
                )
                .is_ok(),
            None => true,
        }
    }
}
