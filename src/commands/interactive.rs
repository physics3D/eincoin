use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::exit,
    thread,
    time::Duration,
};

use log::{error, info};
use rsa::{pkcs8::FromPublicKey, RsaPublicKey};

use crate::{
    blockchain::{Blockchain, Wallet},
    networking::{LogMiddleware, NetworkingManager, NodeMiddleware},
};

pub fn interactive(addr: String, port: String, private_key_file: PathBuf) {
    // interactive eincoin shell
    let wallet = Wallet::new_from_keyfile(private_key_file);
    let mut chain = Blockchain::new_empty();

    let mut networking_manager = NetworkingManager::new(Some(addr + ":" + &port), None).unwrap();
    networking_manager.add_middleware(LogMiddleware);
    networking_manager.add_middleware(NodeMiddleware::new(false, |_, _, _| {}));
    networking_manager.start_client_server();

    let sender = networking_manager.get_sender();
    let receiver = networking_manager.get_receiver().unwrap();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command: Vec<&str> = input.trim().split_whitespace().collect();

        // do networking stuff after readline
        // while there are still new messages
        // todo: make this in extra thread or so
        while let Ok(message) = receiver.try_recv() {
            networking_manager.run_middlewares(message, &mut chain);
        }

        // error handling for empty line
        if command.len() == 0 {
            continue;
        }

        println!();

        match command[0] {
            "balance" => {
                println!("{}", wallet.compute_balance(&mut chain));
            }
            "transaction" => {
                if command.len() != 3 {
                    error!("Usage: transaction <amount> <payee-public-key>");
                    continue;
                }

                let amount = match command[1].parse() {
                    Ok(amount) => amount,
                    Err(_) => {
                        error!("The amount has to be a number");
                        continue;
                    }
                };
                let payee_public_key = match RsaPublicKey::from_public_key_pem(
                    &match read_to_string(PathBuf::from(command[2])) {
                        Ok(string) => string,
                        Err(_) => {
                            error!("Failed to read the key from {}", command[2]);
                            continue;
                        }
                    },
                ) {
                    Ok(key) => key,
                    Err(err) => {
                        error!(
                            "{:?} is not a PEM-encoded private key file. Most probably you provided a public key file instead: {}",
                            command[2],
                            err
                        );
                        continue;
                    }
                };

                match wallet.send_money(amount, payee_public_key, sender.clone()) {
                    Ok(_) => {
                        info!("Sent {} eincoin", amount);
                        // todo: find a better way than that
                        thread::sleep(Duration::from_secs(1));
                    }
                    Err(err) => error!("Error while sending the money: {}", err),
                }
            }
            "chain" => {
                println!("{:#?}", chain);
            }
            "exit" => exit(0),
            _ => error!("Unknown command"),
        }
    }
}
