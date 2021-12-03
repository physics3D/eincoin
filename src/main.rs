use blockchain::Blockchain;
use wallet::Wallet;

mod block;
mod blockchain;
mod consts;
mod transaction;
mod util;
mod wallet;

fn main() {
    let satoshi = Wallet::new();
    let bob = Wallet::new();
    let alice = Wallet::new();

    let mut chain = Blockchain::new(satoshi.public_key.clone());

    println!(
        "satoshi: {}",
        chain.get_wallet_money(satoshi.public_key.clone())
    );
    println!("bob: {}", chain.get_wallet_money(bob.public_key.clone()));
    println!(
        "alice: {}",
        chain.get_wallet_money(alice.public_key.clone())
    );

    satoshi
        .send_money(50, bob.public_key.clone(), &mut chain)
        .unwrap();
    satoshi
        .send_money(23, alice.public_key.clone(), &mut chain)
        .unwrap();
    satoshi
        .send_money(5, bob.public_key.clone(), &mut chain)
        .unwrap();

    println!("{}", chain.verify());

    println!("satoshi: {}", chain.get_wallet_money(satoshi.public_key));
    println!("bob: {}", chain.get_wallet_money(bob.public_key));
    println!("alice: {}", chain.get_wallet_money(alice.public_key));

    // println!("{:#?}", chain);
}
