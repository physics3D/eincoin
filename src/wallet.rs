use rand::rngs::OsRng;

use rsa::{errors::Error, Hash, PaddingScheme, RSAPrivateKey, RSAPublicKey};

use crate::{blockchain::Blockchain, consts::KEY_PAIR_LENGTH, transaction::Transaction};

pub struct Wallet {
    pub private_key: RSAPrivateKey,
    pub public_key: RSAPublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;

        let private_key =
            RSAPrivateKey::new(&mut rng, KEY_PAIR_LENGTH).expect("failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

        Self {
            private_key,
            public_key,
        }
    }

    pub fn send_money(
        &self,
        amount: u32,
        payee_public_key: RSAPublicKey,
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

        chain.add_block(transaction, &self.public_key, signature)?;

        Ok(())
    }
}
