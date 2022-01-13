use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bus::Bus;

use rand::rngs::OsRng;
use rsa::{
    pkcs8::{FromPrivateKey, ToPrivateKey, ToPublicKey},
    RsaPrivateKey, RsaPublicKey,
};

use crate::{
    consts::KEY_PAIR_LENGTH,
    networking::{InternalMessage, MessageDest, MessageSource, MessageType},
    util::LogExpect,
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
        let private_key_string = fs::read_to_string(&private_key_file).log_expect(&format!(
            "Failed to read the key from {:?}",
            private_key_file
        ));

        let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_string).log_expect(&format!(
            "{:?} is not a PEM-encoded private key file. Most probably you provided a public key file instead",
            private_key_file
        ));
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
        chain: &mut Blockchain,
    ) -> Result<(), String> {
        let transaction =
            Transaction::new(amount, Some(self.clone()), payee_public_key.clone(), chain)?;

        sender.lock().unwrap().broadcast(InternalMessage::new(
            MessageType::Transaction(transaction),
            MessageSource::Localhost,
            MessageDest::Broadcast,
        ));

        Ok(())
    }

    pub fn compute_balance(&self, chain: &mut Blockchain) -> u32 {
        chain
            .utxos
            .iter()
            .filter(|(_, _, tx_out)| tx_out.payee == self.public_key)
            .map(|(_, _, tx_out)| tx_out.amount)
            .sum()
    }
}
