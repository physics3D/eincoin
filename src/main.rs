use std::fs::{read_to_string, write, File};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use log::{error, info};
use networking::NetworkingManager;

use rsa::pkcs8::FromPublicKey;
use rsa::RsaPublicKey;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::blockchain::{Blockchain, Wallet};
use crate::cli::{setup_loggers, CliArgs, Command};
use crate::networking::{
    GenesisMiddleware, LogMiddleware, MinerMiddleware, NodeMiddleware, ServerMiddleware,
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
            info!("Generating keypair");
            let wallet = Wallet::new_random();
            let (private_key_string, public_key_string) = wallet.to_string();

            if let Some(path) = file {
                info!("Writing keypair to file {}", path);
                write(
                    PathBuf::from(path.clone() + ".priv.pem"),
                    private_key_string,
                )
                .unwrap();
                write(PathBuf::from(path + ".pub.pem"), public_key_string).unwrap();
            } else {
                info!("Printing keypair to stdout");
                println!("{}", private_key_string);
                println!();
                println!("{}", public_key_string);
            }
        }
        Command::GenPubKey { private_key_file } => {
            let wallet = Wallet::new_from_keyfile(private_key_file.clone());
            write(
                PathBuf::from(private_key_file.to_string_lossy().to_string() + ".pub.pem"),
                wallet.to_string().1,
            )
            .unwrap();
        }
        Command::GenCompletions { shell, file } => {
            let shell_shell = Shell::from_str(&shell);
            if let Ok(shell_type) = shell_shell {
                CliArgs::clap().gen_completions_to(
                    env!("CARGO_BIN_NAME"),
                    shell_type,
                    &mut match File::create(file) {
                        Ok(the_file) => the_file,
                        Err(err) => {
                            error!("Writing to file failed: {}", err);
                            exit(1);
                        }
                    },
                );
            } else {
                error!("Unknown shell!");
                exit(1);
            }
        }
        Command::FullNode {
            addr,
            port,
            miner,
            server,
            private_key_file,
        } => {
            // its an ordinary client/server
            let mut chain = Blockchain::new_empty();

            let mut networking_manager =
                NetworkingManager::new(Some(addr + ":" + &port), server.clone()).unwrap();

            networking_manager.add_middleware(LogMiddleware);
            networking_manager.add_middleware(NodeMiddleware::new(server.is_some(), |_, _, _| {}));
            if miner {
                let wallet = Wallet::new_from_keyfile(private_key_file.unwrap());
                networking_manager.add_middleware(MinerMiddleware::new(wallet));
            }
            if server.is_some() {
                networking_manager.add_middleware(ServerMiddleware);
            }

            networking_manager.start_networking(&mut chain);
        }
        Command::Genesis {
            port,
            private_key_file,
        } => {
            // its a genesis node setting up a new blockchain
            let wallet = Wallet::new_from_keyfile(private_key_file);
            let mut chain = Blockchain::new(wallet.public_key.clone());

            let mut networking_manager = NetworkingManager::new(None, Some(port)).unwrap();

            networking_manager.add_middleware(LogMiddleware);
            networking_manager.add_middleware(GenesisMiddleware);
            networking_manager.add_middleware(MinerMiddleware::new(wallet));
            networking_manager.add_middleware(ServerMiddleware);

            networking_manager.start_networking(&mut chain);
        }
        Command::Transaction {
            addr,
            port,
            amount,
            payee_public_key,
            private_key_file,
        } => {
            let wallet = Wallet::new_from_keyfile(private_key_file);
            let mut chain = Blockchain::new_empty();

            let mut networking_manager =
                NetworkingManager::new(Some(addr + ":" + &port), None).unwrap();

            let payee_public_key = RsaPublicKey::from_public_key_pem(
                &read_to_string(payee_public_key.clone()).unwrap(),
            )
            .unwrap();

            networking_manager.add_middleware(LogMiddleware);
            networking_manager.add_middleware(NodeMiddleware::new(false, move |_, sender, _| {
                match wallet.send_money(amount, payee_public_key.clone(), sender) {
                    Ok(_) => {
                        info!("Sent the money");
                        // todo: find a better way than that
                        thread::sleep(Duration::from_secs(1));
                    }
                    Err(err) => error!("Error while seding the money: {}", err),
                }
                exit(0);
            }));

            networking_manager.start_networking(&mut chain);
        }
        Command::Balance {
            addr,
            port,
            private_key_file,
        } => {
            let wallet = Wallet::new_from_keyfile(private_key_file);
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
