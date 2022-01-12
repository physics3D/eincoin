use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionOutput {
    pub amount: u32,
    pub payee: RsaPublicKey,
}

#[derive(Serialize)]
struct TransactionOutputForCheck {
    pub amount: u32,
    pub payee: RsaPublicKey,
}

impl TransactionOutput {
    pub fn new(amount: u32, payee: RsaPublicKey) -> Self {
        let tx_out = TransactionOutput { amount, payee };

        tx_out
    }
}
