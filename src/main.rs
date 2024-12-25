use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: u128, data: String, previous_hash: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

