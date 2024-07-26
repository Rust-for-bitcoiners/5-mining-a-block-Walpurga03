use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::error::Error;
use crate::transaction::Transaction;
use crate::block_header::BlockHeader;

// Function for writing the block to a file
pub fn write_block_to_file(header: &BlockHeader, coinbase: &Transaction, transactions: &[Transaction]) -> io::Result<()> {
    let mut file = File::create("out.txt")?;
    writeln!(file, "Block Header:")?;
    writeln!(file, "Previous Block Hash: {}", header.previous_block_hash)?;
    writeln!(file, "Merkle Root: {}", header.merkle_root)?;
    writeln!(file, "Nonce: {}", header.nonce)?;
    writeln!(file, "Difficulty Target: {}", header.difficulty_target)?;
    writeln!(file, "\nCoinbase Transaction:")?;
    writeln!(file, "{}", serde_json::to_string_pretty(&coinbase).unwrap())?;
    writeln!(file, "\nTransactions:")?;
    writeln!(file, "{}", coinbase.txid)?;
    for tx in transactions.iter().skip(1) {
        writeln!(file, "{}", tx.txid)?;
    }
    Ok(())
}

// Function for reading the transaction IDs from the mempool
pub fn read_mempool_json(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data = fs::read_to_string(file_path)?;
    let txids: Vec<String> = serde_json::from_str(&data)?;
    Ok(txids)
}

// Function for reading a transaction from a file
pub fn read_transaction_from_file(file_path: &str) -> Result<Transaction, Box<dyn Error>> {
    let data = fs::read_to_string(file_path)?;
    let transaction: Transaction = serde_json::from_str(&data)?;
    Ok(transaction)
}
