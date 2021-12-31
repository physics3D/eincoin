use std::{path::PathBuf, process::exit};

use crate::{
    blockchain::{Blockchain, Wallet},
    networking::{LogMiddleware, NetworkingManager, NodeMiddleware},
};

pub fn balance(addr: String, port: String, private_key_file: PathBuf) {
    let wallet = Wallet::new_from_keyfile(private_key_file);
    let mut chain = Blockchain::new_empty();

    let mut networking_manager = NetworkingManager::new(Some(addr + ":" + &port), None);

    networking_manager.add_middleware(LogMiddleware);
    networking_manager.add_middleware(NodeMiddleware::new(false, move |_, _, chain| {
        println!(
            "Your wallet's current balance is: {}",
            wallet.compute_balance(chain)
        );
        exit(0);
    }));

    networking_manager.start_networking(&mut chain);
}
