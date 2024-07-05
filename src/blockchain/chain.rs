use super::block::Block;
use sha2::{Sha256, Digest};
use log::info;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: vec![Self::create_genesis_block()] }
    }

    fn create_genesis_block() -> Block {
        Block {
            index: 0,
            previous_hash: String::from("0"),
            hash: String::from("genesis_hash"),
            data: String::from("Genesis Block"),
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let new_block = Block {
            index: previous_block.index + 1,
            previous_hash: previous_block.hash.clone(),
            hash: Self::calculate_hash(previous_block.index + 1, &previous_block.hash, &data),
            data,
        };
        info!("Adding new block: {:?}", new_block);
        self.chain.push(new_block);
    }

    //  pub fn add_block_poc(&mut self, data: String, node_id: usize, cluster_id: usize) {
    //     let previous_block = self.chain.last().unwrap_or_else(|| {
    //         let genesis_block = Block {
    //             index: 0,
    //             previous_hash: String::from("genesis_hash"),
    //             hash: String::from("genesis_hash"),
    //             data: String::from("Genesis Block"),
    //             node_id: 0,
    //             cluster_id: 0,
    //         };
    //         self.chain.push(genesis_block.clone());
    //         self.chain.last().unwrap()
    //     });

    //     let new_block = Block {
    //         index: previous_block.index + 1,
    //         previous_hash: previous_block.hash.clone(),
    //         hash: Blockchain::calculate_hash(previous_block.index + 1, &previous_block.hash, &data),
    //         data,
    //         node_id,
    //         cluster_id,
    //     };

    //      info!("Adding new block: {:?}", new_block);
    //     self.chain.push(new_block);

    // }

    fn calculate_hash(index: u64, previous_hash: &str, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string().as_bytes());
        hasher.update(previous_hash.as_bytes());
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
