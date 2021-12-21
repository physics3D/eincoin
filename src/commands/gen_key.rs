use crate::blockchain::Wallet;

use log::info;
use std::{fs::write, path::PathBuf};

pub fn gen_key(file: Option<String>) {
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
