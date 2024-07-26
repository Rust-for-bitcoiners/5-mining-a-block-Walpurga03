use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    /* Creates a vector hashes containing the txid of each transaction.
        transactions.iter() creates an iterator over the transaction list.
        map(|tx| tx.txid.clone()) applies a function to each element of the iterator that copies the ‘txid’ of the transaction.
        collect() collects the results in a vector of type Vec<String>.
    */
    let mut hashes: Vec<String> = transactions.iter().map(|tx| tx.txid.clone()).collect();

    /*Calculation of the Merkle root:
    As long as the list of hashes contains more than one element:
    If the number of hashes is odd, the last hash is doubled to make the list even.
    Each neighbouring hash is combined (concatenated) and hashed:
    The combined hashes are hashed to create a new list of hashes (new_hashes).
    The old list of hashes is replaced by the new list (new_hashes).
    */
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        let mut new_hashes = Vec::new();

        for i in (0..hashes.len()).step_by(2) {
            let combined = format!("{}{}", hashes[i], hashes[i + 1]);
            let mut hasher = Sha256::new();
            hasher.update(combined.as_bytes());
            let result = hasher.finalize();
            new_hashes.push(format!("{:x}", result));
        }

        hashes = new_hashes;
    }

    hashes[0].clone()
}
