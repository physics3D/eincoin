use crate::block::Block;
use crate::consts::KEY_PAIR_LENGTH;
use crate::transaction::Transaction;
use chrono::Utc;
use rand::rngs::OsRng;
use rsa::errors::Error;
use rsa::{Hash, PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(initial_payee_public_key: RSAPublicKey) -> Self {
        let mut rng = OsRng;
        let genesis = RSAPrivateKey::new(&mut rng, KEY_PAIR_LENGTH)
            .unwrap()
            .to_public_key();

        let mut genesis_block = Block {
            prev_hash: vec![],
            transaction: Transaction {
                amount: 100,
                payer: genesis,
                payee: initial_payee_public_key,
            },
            date: Utc::now().to_string(),
            nonce: 0,
        };
        genesis_block.mine();

        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn last_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn add_block(
        &mut self,
        transaction: Transaction,
        sender_public_key: &RSAPublicKey,
        signature: Vec<u8>,
    ) -> Result<(), Error> {
        let is_verified = sender_public_key
            .verify(
                PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),
                &transaction.hash(),
                &signature,
            )
            .is_ok();

        if is_verified {
            let mut new_block = Block {
                prev_hash: self.last_block().hash(),
                transaction,
                date: Utc::now().to_string(),
                nonce: Block::generate_nonce(),
            };
            new_block.mine();
            self.chain.push(new_block);
            return Ok(());
        }

        println!("wrong signature");
        Err(Error::Verification)
    }

    pub fn verify(&self) -> bool {
        println!("Verifying...");

        self.chain.iter().all(|block| block.verify())
    }

    pub fn get_wallet_money(&self, wallet_public_key: RSAPublicKey) -> u32 {
        let mut money = 0;

        for block in &self.chain {
            if block.transaction.payee == wallet_public_key {
                money += block.transaction.amount;
            } else if block.transaction.payer == wallet_public_key {
                money -= block.transaction.amount;
            }
        }

        money
    }
}
