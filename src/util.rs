use std::{error::Error, fmt::Display};

use log::error;

use sha2::{Digest, Sha256};

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);

    let result = hasher.finalize();

    result.to_vec()
}

#[derive(Debug)]
pub struct EincoinError {
    msg: String,
}

impl EincoinError {
    pub fn new(msg: String) -> Self {
        error!("{}", msg);
        Self { msg: msg }
    }
}

impl Display for EincoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Error for EincoinError {}
