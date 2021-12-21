use std::path::PathBuf;

use crate::blockchain::Wallet;

use std::fs::write;

pub fn gen_pub_key(private_key_file: PathBuf) {
    let wallet = Wallet::new_from_keyfile(private_key_file.clone());
    write(
        PathBuf::from(private_key_file.to_string_lossy().to_string() + ".pub.pem"),
        wallet.to_string().1,
    )
    .unwrap();
}
