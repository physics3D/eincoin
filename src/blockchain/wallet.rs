use std::{
    fs,
    path::PathBuf,
    process::exit,
    sync::{Arc, Mutex},
};

use bus::Bus;
use log::error;

use rand::rngs::OsRng;
use rsa::{
    errors::Error,
    pkcs8::{FromPrivateKey, ToPrivateKey, ToPublicKey},
    RsaPrivateKey, RsaPublicKey,
};

use crate::{
    consts::KEY_PAIR_LENGTH,
    networking::{InternalMessage, MessageDest, MessageSource, MessageType},
};

use super::{Blockchain, Transaction};

#[derive(Clone)]
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

    pub fn new_from_keyfile(private_key_file: PathBuf) -> Self {
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

        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_string).unwrap();
        let public_key = private_key.to_public_key();

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
        sender: Arc<Mutex<Bus<InternalMessage>>>,
    ) -> Result<(), Error> {
        // why do we need that dereferencing
        let transaction = Transaction::new(amount, Some(self.clone()), payee_public_key.clone());

        sender.lock().unwrap().broadcast(InternalMessage::new(
            MessageType::Transaction(transaction),
            MessageSource::Localhost,
            MessageDest::Broadcast,
        ));

        Ok(())
    }

    pub fn compute_balance(&self, chain: &mut Blockchain) -> u32 {
        let mut money = 0;

        for block in &chain.chain {
            for transaction in &block.transactions {
                if transaction.payee == self.public_key {
                    money += transaction.amount;
                } else if let Some(payer) = &transaction.payer {
                    if *payer == self.public_key {
                        money -= transaction.amount;
                    }
                }
            }
        }

        money
    }
}
