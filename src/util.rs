use std::{
    fmt::Display,
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

use log::error;

use sha2::{Digest, Sha256};

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);

    let result = hasher.finalize();

    result.to_vec()
}

pub fn time_since_unix_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub trait LogExpect<T> {
    fn log_expect(self, message: &str) -> T;
}

impl<T, E: Display> LogExpect<T> for Result<T, E> {
    fn log_expect(self, message: &str) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                error!("{}: {}", message, err);
                exit(1);
            }
        }
    }
}
