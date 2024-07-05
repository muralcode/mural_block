use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}
