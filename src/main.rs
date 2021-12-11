use std::fs::read_to_string;
use std::process::exit;

use log::info;
use networking::NetworkingManager;

use rsa::pkcs8::FromPublicKey;
use rsa::RsaPublicKey;
use structopt::StructOpt;

use crate::blockchain::{Blockchain, Transaction, Wallet};
use crate::cli::{gen_key, setup_loggers, CliArgs, Command};
use crate::networking::{
    GenesisMiddleware, InternalMessage, LogMiddleware, MessageDest, MessageSource, MessageType,
    MinerMiddleware, NodeMiddleware, ServerMiddleware,
};

mod blockchain;
mod cli;
mod consts;
mod networking;
mod util;

fn main() {
    let cli_args = CliArgs::from_args();

    setup_loggers(cli_args.clone());

    info!("Started eincoin node");

    match cli_args.subcommand {
        Command::GenKey { file } => {
            // just generate the key
            gen_key(file);
        }
        Command::FullNode {
            addr,
            port,
            miner,
            server,
        } => {
            // its an ordinary client/server
            let mut chain = Blockchain::new_empty();

            let mut networking_manager =
                NetworkingManager::new(Some(addr + ":" + &port), server.clone()).unwrap();

            networking_manager.add_middleware(LogMiddleware);
            networking_manager.add_middleware(NodeMiddleware::new(server.is_some(), |_, _, _| {}));
            if miner {
                networking_manager.add_middleware(MinerMiddleware::new());
            }
            if server.is_some() {
                networking_manager.add_middleware(ServerMiddleware);
            }

            networking_manager.start_networking(&mut chain);
        }
        Command::Genesis {
            port,
            private_key_file,
            public_key_file,
        } => {
            // its a genesis node setting up a new blockchain
            let wallet = Wallet::new_from_keyfiles(private_key_file, public_key_file);
            let mut chain = Blockchain::new(wallet.public_key);

            let mut networking_manager = NetworkingManager::new(None, Some(port)).unwrap();

            networking_manager.add_middleware(LogMiddleware);
            networking_manager.add_middleware(GenesisMiddleware);
            networking_manager.add_middleware(MinerMiddleware::new());
            networking_manager.add_middleware(ServerMiddleware);

            networking_manager.start_networking(&mut chain);
        }
        Command::Transaction {
            addr,
            port,
            amount,
            payee_public_key,
            private_key_file,
            public_key_file,
        } => {
            let wallet = Wallet::new_from_keyfiles(private_key_file, public_key_file);
            let mut chain = Blockchain::new_empty();

            let mut networking_manager =
                NetworkingManager::new(Some(addr + ":" + &port), None).unwrap();

            networking_manager.add_middleware(LogMiddleware);
            networking_manager.add_middleware(NodeMiddleware::new(false, move |_, sender, _| {
                let payee_public_key = RsaPublicKey::from_public_key_pem(
                    &read_to_string(payee_public_key.clone()).unwrap(),
                )
                .unwrap();

                sender.lock().unwrap().broadcast(InternalMessage::new(
                    MessageType::Transaction(Transaction {
                        amount,
                        payer: wallet.public_key.clone(),
                        payee: payee_public_key,
                    }),
                    MessageSource::Localhost,
                    MessageDest::Broadcast,
                ));
            }));

            networking_manager.start_networking(&mut chain);
        }
        Command::Balance {
            addr,
            port,
            private_key_file,
            public_key_file,
        } => {
            let wallet = Wallet::new_from_keyfiles(private_key_file, public_key_file);
            let mut chain = Blockchain::new_empty();

            let mut networking_manager =
                NetworkingManager::new(Some(addr + ":" + &port), None).unwrap();

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
    }

    info!("Terminating eincoin node");
}
