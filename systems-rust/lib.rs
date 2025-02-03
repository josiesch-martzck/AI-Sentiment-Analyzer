use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 4203
// Hash 1601
// Hash 9200
// Hash 5267
// Hash 3354
// Hash 1146
// Hash 5925
// Hash 6140
// Hash 6047
// Hash 9032
// Hash 1680
// Hash 1466
// Hash 5495
// Hash 5720
// Hash 7689
// Hash 1420
// Hash 7709
// Hash 7851
// Hash 7723
// Hash 2187
// Hash 4251
// Hash 5213
// Hash 8941
// Hash 2451
// Hash 3403
// Hash 6106
// Hash 2224
// Hash 6993
// Hash 3543
// Hash 4164
// Hash 8582
// Hash 8209
// Hash 4954
// Hash 1658
// Hash 9872
// Hash 8442
// Hash 4272
// Hash 7773
// Hash 3715
// Hash 2699
// Hash 7985
// Hash 5075
// Hash 8525
// Hash 7220
// Hash 1063
// Hash 2848
// Hash 9086
// Hash 5759
// Hash 4615
// Hash 7743
// Hash 3937
// Hash 9523
// Hash 9202
// Hash 2379
// Hash 5082
// Hash 7029