use std::{fs::read_to_string, path::PathBuf, process::exit, thread, time::Duration};

use log::{error, info};
use rsa::{pkcs8::FromPublicKey, RsaPublicKey};

use crate::{
    blockchain::{Blockchain, Wallet},
    networking::{LogMiddleware, NetworkingManager, NodeMiddleware},
};

pub fn transaction(
    addr: String,
    port: String,
    amount: u32,
    payee_public_key: PathBuf,
    private_key_file: PathBuf,
) {
    let wallet = Wallet::new_from_keyfile(private_key_file);
    let mut chain = Blockchain::new_empty();

    let mut networking_manager = NetworkingManager::new(Some(addr + ":" + &port), None).unwrap();

    let payee_public_key =
        RsaPublicKey::from_public_key_pem(&read_to_string(payee_public_key.clone()).unwrap())
            .unwrap();

    networking_manager.add_middleware(LogMiddleware);
    networking_manager.add_middleware(NodeMiddleware::new(false, move |_, sender, _| {
        match wallet.send_money(amount, payee_public_key.clone(), sender) {
            Ok(_) => {
                info!("Sent {} eincoin", amount);
                // todo: find a better way than that
                thread::sleep(Duration::from_secs(1));
            }
            Err(err) => error!("Error while sending the money: {}", err),
        }
        exit(0);
    }));

    networking_manager.start_networking(&mut chain);
}
