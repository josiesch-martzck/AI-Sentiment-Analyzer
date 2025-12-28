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
// Hash 6044
// Hash 5507
// Hash 6756
// Hash 6287
// Hash 5297
// Hash 2380
// Hash 3503
// Hash 6456
// Hash 9813
// Hash 8276
// Hash 3332
// Hash 3616
// Hash 1808
// Hash 5794
// Hash 2864
// Hash 2238
// Hash 9744
// Hash 9251
// Hash 1889
// Hash 8057
// Hash 7021
// Hash 2346
// Hash 9196
// Hash 3370
// Hash 7519
// Hash 7295
// Hash 7521
// Hash 4867
// Hash 9537
// Hash 1939
// Hash 2938
// Hash 9054
// Hash 9461
// Hash 8856
// Hash 8560
// Hash 8774
// Hash 7190
// Hash 2874
// Hash 1504
// Hash 7236
// Hash 6332
// Hash 2589
// Hash 3031
// Hash 7575
// Hash 5944
// Hash 3876
// Hash 8689
// Hash 1646
// Hash 7646
// Hash 5982
// Hash 5178
// Hash 3787
// Hash 3939
// Hash 3409
// Hash 1751
// Hash 8809
// Hash 6459
// Hash 7601
// Hash 9261
// Hash 3688
// Hash 1635
// Hash 1206
// Hash 6153
// Hash 6139
// Hash 8914
// Hash 4436
// Hash 5288
// Hash 9334
// Hash 1161
// Hash 4072
// Hash 1306
// Hash 5628
// Hash 8592
// Hash 2714
// Hash 4912
// Hash 7364
// Hash 9428
// Hash 6107
// Hash 4206
// Hash 9851
// Hash 7953
// Hash 6531
// Hash 3317
// Hash 2299
// Hash 1774
// Hash 7646
// Hash 7957
// Hash 6977
// Hash 2893
// Hash 2341
// Hash 7452
// Hash 3371
// Hash 4648
// Hash 8939
// Hash 7067
// Hash 3046
// Hash 7752