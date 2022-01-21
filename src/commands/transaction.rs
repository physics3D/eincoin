use std::{fs::read_to_string, path::PathBuf, process::exit, thread, time::Duration};

use log::info;
use rsa::{pkcs8::FromPublicKey, RsaPublicKey};

use crate::{
    blockchain::{Blockchain, Wallet},
    networking::{NetworkingManager, NodeMiddleware},
    util::LogExpect,
};

pub fn transaction(
    addr: String,
    port: String,
    amount: u32,
    payee_public_key: PathBuf,
    private_key_file: PathBuf,
    transaction_fee: u32,
) {
    let wallet = Wallet::new_from_keyfile(private_key_file);
    let mut chain = Blockchain::new_empty();

    let mut networking_manager = NetworkingManager::new(Some(addr + ":" + &port), None);

    let payee_public_key = RsaPublicKey::from_public_key_pem(&read_to_string(
        &payee_public_key,
    )
    .log_expect(&format!("Failed to read the key form {:?}", &payee_public_key)))
    .log_expect(&format!(
                "{:?} is not a PEM-encoded private key file. Most probably you provided a private key file instead",
                &payee_public_key
        ));

    networking_manager.add_middleware(NodeMiddleware::new(
        false,
        false,
        move |_, sender, blockchain| {
            wallet
                .send_money(
                    amount,
                    transaction_fee,
                    payee_public_key.clone(),
                    sender,
                    blockchain,
                )
                .log_expect("Error while sending the money");
            info!("Sent {} eincoin", amount);
            // todo: find a better way than that
            thread::sleep(Duration::from_secs(1));
            exit(0);
        },
    ));

    networking_manager.start_networking(&mut chain);
}
