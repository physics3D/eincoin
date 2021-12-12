use rsa::{Hash, PaddingScheme, PublicKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::util::sha256;

use super::Wallet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub amount: u32,
    pub payer: Option<RsaPublicKey>,
    pub payee: RsaPublicKey,
    pub signature: Option<Vec<u8>>,
}

// extra struct only to check the signature (we can't check with the signature IN the struct)
#[derive(Serialize)]
struct TransactionForCheck {
    pub amount: u32,
    pub payer: Option<RsaPublicKey>,
    pub payee: RsaPublicKey,
}

impl Transaction {
    pub fn new(amount: u32, wallet: Option<Wallet>, payee: RsaPublicKey) -> Self {
        let mut transaction = Self {
            amount,
            payer: None,
            payee,
            signature: None,
        };

        if let Some(payer) = wallet {
            transaction.payer = Some(payer.public_key);

            let signature = payer
                .private_key
                .sign(
                    PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                    &transaction.hash(),
                )
                .unwrap();

            transaction.signature = Some(signature);
        }

        transaction
    }

    pub fn verify(&self) -> bool {
        if let Some(signature) = &self.signature {
            return self
                .payer
                .as_ref()
                .unwrap()
                .verify(
                    PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                    &self.hash(),
                    &signature,
                )
                .is_ok();
        }

        true
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(
            &bincode::serialize(&TransactionForCheck {
                amount: self.amount,
                payer: self.payer.clone(),
                payee: self.payee.clone(),
            })
            .unwrap(),
        )
    }
}
