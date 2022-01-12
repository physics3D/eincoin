use std::path::PathBuf;

use crate::{
    blockchain::{Blockchain, Wallet},
    networking::{
        LogMiddleware, MinerMiddleware, NetworkingManager, NodeMiddleware, ServerMiddleware,
    },
};

pub fn full_node(
    addr: String,
    port: String,
    miner: bool,
    server: Option<String>,
    private_key_file: Option<PathBuf>,
) {
    // its an ordinary client/server
    let mut chain = Blockchain::new_empty();

    let mut networking_manager = NetworkingManager::new(Some(addr + ":" + &port), server.clone());

    networking_manager.add_middleware(LogMiddleware);
    networking_manager.add_middleware(NodeMiddleware::new(server.is_some(), false, |_, _, _| {}));
    if miner {
        let wallet = Wallet::new_from_keyfile(private_key_file.unwrap());
        networking_manager.add_middleware(MinerMiddleware::new(wallet));
    }
    if server.is_some() {
        networking_manager.add_middleware(ServerMiddleware);
    }

    networking_manager.start_networking(&mut chain);
}
