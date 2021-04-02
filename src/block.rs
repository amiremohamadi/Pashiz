#![allow(unused)]
use crate::utils::{self, Encodable};

/// Nodes collect new transactions into blocks, hash them into a hash tree,
/// and scan through nonce values to make the blocks's hash satisfy
/// proof-of-work requirements.
/// When they solve proof-of-work, they broadcast the block to other nodes
/// and block is added to the blockchain. The first transaction in the
/// block is a special one that creates a new coin owned by the creator
/// of the block.
pub struct Block {
    pub version: [u8; 2],
    pub index: u32,
    pub timestamp: u64,

    pub hash_prev_block: [u8; 32],
    pub hash_merkle_root: [u8; 32],

    pub nonce: u64,
}

impl Block {
    fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        self.version.serialize(&mut b);
        self.index.serialize(&mut b);
        self.timestamp.serialize(&mut b);
        self.hash_prev_block.serialize(&mut b);
        self.hash_merkle_root.serialize(&mut b);
        self.nonce.serialize(&mut b);
        b
    }

    fn hash(&self) -> [u8; 32] {
        utils::sha256_bytes(&self.to_bytes()[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash() {
        let block = Block {
            version: [0x0, 0x1],
            index: 0,
            timestamp: 0xffff0000,
            hash_prev_block: [0x00; 32],
            hash_merkle_root: [0x00; 32],
            nonce: 0xff00ff00,
        };
        assert_eq!(
            block.hash(),
            [
                250, 161, 123, 5, 162, 47, 106, 22, 13, 158, 237, 200, 140, 194, 223, 211, 32, 38,
                164, 184, 245, 49, 129, 177, 120, 228, 175, 86, 147, 39, 176, 156
            ]
        );
    }
}
