use std::{fs, path::PathBuf, process::exit};

use log::{error, warn};

use rand::{rngs::OsRng, Rng};
use rsa::{
    errors::Error,
    pkcs8::{FromPrivateKey, FromPublicKey, ToPrivateKey, ToPublicKey},
    Hash, PaddingScheme, RsaPrivateKey, RsaPublicKey,
};

use crate::consts::KEY_PAIR_LENGTH;

use super::{Blockchain, Transaction};

pub struct Wallet {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

impl Wallet {
    pub fn new_random() -> Self {
        let mut rng = OsRng;

        let private_key = RsaPrivateKey::new(&mut rng, KEY_PAIR_LENGTH).unwrap();
        let public_key = private_key.to_public_key();

        Self {
            private_key,
            public_key,
        }
    }

    pub fn new_from_keyfiles(private_key_file: PathBuf, public_key_file: PathBuf) -> Self {
        let private_key_string = match fs::read_to_string(&private_key_file) {
            Ok(file_content) => file_content,
            Err(err) => {
                error!(
                    "Failed to read the key from {:?} because of {}",
                    private_key_file, err
                );
                exit(0);
            }
        };
        let public_key_string = match fs::read_to_string(&public_key_file) {
            Ok(file_content) => file_content,
            Err(err) => {
                error!(
                    "Failed to read the key from {:?} because of {}",
                    private_key_file, err
                );
                exit(0);
            }
        };

        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_string).unwrap();
        let public_key_computed = RsaPublicKey::from_public_key_pem(&public_key_string).unwrap();
        let public_key = private_key.to_public_key();

        if public_key_computed != public_key {
            warn!("Public keys don't match");
        }

        Self {
            private_key,
            public_key,
        }
    }

    pub fn to_string(&self) -> (String, String) {
        let private_key_string = self.private_key.to_pkcs8_pem().unwrap().to_string();
        let public_key_string = self.public_key.to_public_key_pem().unwrap();

        (private_key_string, public_key_string)
    }

    pub fn send_money(
        &self,
        amount: u32,
        payee_public_key: RsaPublicKey,
        chain: &mut Blockchain,
    ) -> Result<(), Error> {
        let transaction = Transaction {
            amount,
            payer: self.public_key.clone(),
            payee: payee_public_key.clone(),
        };

        let signature = self
            .private_key
            .sign(
                PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                &transaction.hash(),
            )
            .unwrap();

        chain.add_block(vec![transaction], &self.public_key, vec![signature])?;

        Ok(())
    }

    pub fn compute_balance(&self, chain: &mut Blockchain) -> u32 {
        let mut money = 0;

        for block in &chain.chain {
            for transaction in &block.transactions {
                if transaction.payee == self.public_key {
                    money += transaction.amount;
                } else if transaction.payer == self.public_key {
                    money -= transaction.amount;
                }
            }
        }

        money
    }
}
