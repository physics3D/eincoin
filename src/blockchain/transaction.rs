use rsa::{Hash, PaddingScheme, PublicKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::util::sha256;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub amount: u32,
    pub payer: RsaPublicKey,
    pub payee: RsaPublicKey,
}

impl Transaction {
    pub fn verify(&self, signature: &[u8]) -> bool {
        self.payer
            .verify(
                PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                &self.hash(),
                &signature,
            )
            .is_ok()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", &self)
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(self.to_string().as_bytes())
    }
}
