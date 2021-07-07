#![allow(unused)]

/// Nodes collect new transactions into blocks, hash them into a hash tree,
/// and scan through nonce values to make the blocks's hash satisfy
/// proof-of-work requirements.
/// When they solve proof-of-work, they broadcast the block to other nodes
/// and block is added to the blockchain. The first transaction in the
/// block is a special one that creates a new coin owned by the creator
/// of the block.
pub struct Block {
    // header
    pub timestamp: u64,
    pub hash_prev_block: String,
    pub hash_merkle_root: String,
    pub nonce: u64,
    pub difficulty: u128,
}

impl Block {
    fn new(timestamp: u64, hash_prev_block: &str, difficulty: u128) -> Self {
        Self {
            timestamp,
            difficulty,
            nonce: 0,
            hash_merkle_root: String::new(),
            hash_prev_block: hash_prev_block.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_new_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let b = Block::new(now, "0x1234567890", 9999999999999u128);
        assert_eq!(b.nonce, 0);
        assert_eq!(b.timestamp, now);
        assert_eq!(b.difficulty, 9999999999999u128);
        assert_eq!(b.hash_merkle_root, String::from(""));
        assert_eq!(b.hash_prev_block, String::from("0x1234567890"));
    }
}
