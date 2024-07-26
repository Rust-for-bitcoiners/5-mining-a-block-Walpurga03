// Configuration module that contains the input variables.
pub const BLOCK_REWARD: u64 = 50 * 100_000_000; // 50 BTC in Satoshis
pub const MINER_ADDRESS: &str = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
pub const NUM_TRANSACTIONS_TO_MINE: usize = 20; // Number of transactions to be mined
pub const MEMPOOL_JSON_PATH: &str = "mempool/mempool.json";
pub const MEMPOOL_DIR: &str = "mempool";
pub const PREVIOUS_BLOCK_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
pub const DIFFICULTY_TARGET: &str = "0000ffff00000000000000000000000000000000000000000000000000000000";