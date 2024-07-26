use sha2::{Sha256, Digest};
use crate::transaction::{Transaction, Vin, Vout};

pub fn create_coinbase_transaction(miner_address: &str, reward: u64) -> Transaction {
    // Create the input (Vin):
    let vin = vec![Vin {
        txid: "0".repeat(64),
        vout: 0,
        scriptsig: "coinbase".to_string(),
        sequence: 0xffffffff,
    }];
    // Creating the output (Vout)
    let vout = vec![Vout {
        value: reward,
        scriptpubkey: miner_address.to_string(),
    }];

    // Initialisation of the transaction
    let mut transaction = Transaction {
        txid: "".to_string(),
        version: 1,
        locktime: 0,
        vin,
        vout,
        size: 0,
        weight: 0,
        fee: 0,
        hex: "".to_string(),
    };

    // Serial representation of the transaction
    let serialized = serde_json::to_vec(&transaction).unwrap();

    //Calculation of the transaction ID (txid):
    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    let hash1 = hasher.finalize_reset();
    hasher.update(hash1);
    let txid_bytes = hasher.finalize();
    let txid = format!("{:x}", txid_bytes);

    //Hexadecimal representation
    let hex = serialized.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    transaction.txid = txid;
    transaction.hex = hex.clone();

    //Return of the transaction
    transaction
}
