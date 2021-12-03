use rsa::RSAPublicKey;

use crate::util::sha256;

#[derive(Debug)]
pub struct Transaction {
    pub amount: u32,
    pub payer: RSAPublicKey,
    pub payee: RSAPublicKey,
}

impl Transaction {
    pub fn to_string(&self) -> String {
        format!("{:?}", &self)
    }

    pub fn hash(&self) -> Vec<u8> {
        sha256(self.to_string().as_bytes())
    }
}
