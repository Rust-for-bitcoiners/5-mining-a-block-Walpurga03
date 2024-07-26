use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vin {
    pub txid: String,
    pub vout: u32,
    pub scriptsig: String,
    pub sequence: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vout {
    pub value: u64,
    pub scriptpubkey: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub txid: String,
    pub version: i32,
    pub locktime: u64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub size: u32,
    pub weight: u32,
    pub fee: u64,
    pub hex: String,
}
