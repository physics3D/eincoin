use std::path::PathBuf;

use crate::{
    blockchain::{Blockchain, Wallet},
    networking::{
        GenesisMiddleware, LogMiddleware, MinerMiddleware, NetworkingManager, ServerMiddleware,
    },
};

pub fn genesis(port: String, private_key_file: PathBuf) {
    // its a genesis node setting up a new blockchain
    let wallet = Wallet::new_from_keyfile(private_key_file);
    let mut chain = Blockchain::new(wallet.public_key.clone());

    let mut networking_manager = NetworkingManager::new(None, Some(port));

    networking_manager.add_middleware(LogMiddleware);
    networking_manager.add_middleware(GenesisMiddleware);
    networking_manager.add_middleware(MinerMiddleware::new(wallet));
    networking_manager.add_middleware(ServerMiddleware);

    networking_manager.start_networking(&mut chain);
}
