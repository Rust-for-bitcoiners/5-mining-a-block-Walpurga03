mod coinbase;
mod transaction;
mod merkle;
mod block_header;
mod utils;
mod config;

use std::error::Error;
use block_header::BlockHeader;
use coinbase::create_coinbase_transaction;
use merkle::calculate_merkle_root;
use utils::{read_mempool_json, read_transaction_from_file, write_block_to_file};
use config::{BLOCK_REWARD, MINER_ADDRESS, NUM_TRANSACTIONS_TO_MINE, MEMPOOL_JSON_PATH, MEMPOOL_DIR, PREVIOUS_BLOCK_HASH, DIFFICULTY_TARGET};

fn main() -> Result<(), Box<dyn Error>> {
    // 1. create Coinbase transaction
    let coinbase_tx = create_coinbase_transaction(MINER_ADDRESS, BLOCK_REWARD);
    let mut transactions = vec![coinbase_tx.clone()]; // Place Coinbase transaction at the top of the list

    // 2. selection of transactions in the mempool
    let txids = read_mempool_json(MEMPOOL_JSON_PATH)?;
    for txid in txids.iter().take(NUM_TRANSACTIONS_TO_MINE) { // Select additional transactions
        let file_path = format!("{}/{}.json", MEMPOOL_DIR, txid);
        match read_transaction_from_file(&file_path) {
            Ok(transaction) => transactions.push(transaction),
            Err(e) => eprintln!("Fehler beim Lesen der Datei {}: {}", file_path, e),
        }
    }

    // 3. calculate Merkle root
    let merkle_root = calculate_merkle_root(&transactions);
    
    // 4. create block header
    let nonce: u64 = 0;
    let previous_block_hash: String = PREVIOUS_BLOCK_HASH.to_string();
    let header = BlockHeader {
        previous_block_hash,
        merkle_root,
        nonce,
        difficulty_target: DIFFICULTY_TARGET.to_string(),
    };

    // 5. Write block to the file
    write_block_to_file(&header, &coinbase_tx, &transactions)?;

    println!("Block successfully written.");
    Ok(())
}
