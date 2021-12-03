use sha2::{Digest, Sha256};

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);

    let result = hasher.finalize();

    result.to_vec()
}
