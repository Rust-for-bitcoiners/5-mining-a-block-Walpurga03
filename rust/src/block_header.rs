use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub nonce: u64,
    pub difficulty_target: String,
}