use sha2::Digest;
use sha2::Sha256;
use chrono::Utc;
use log::warn;

const DIFFICULTY_PREFIX: &'static str = "0000";
pub struct App {
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub id: u64,
    pub data: String,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn hex_to_binary_string(hash: &[u8]) -> String {
    let mut res = String::from("");
    for n in hash {
        res.push_str(&format!("{:b}", n));
    }
    res
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis = Block {
            id: 0,
            data: String::from("genesis"),
            hash: "0000f0e671ceac529fee3f68db0ae3937a85e23b3c98d51a1a7796c3c042f17a".to_string(),
            prev_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            timestamp: Utc::now().timestamp(),
            nonce: 69,
        };
        self.blocks.push(genesis);
    }

    fn add_new_block(&mut self, block: Block) {}

    fn is_valid_block(&self, block: &Block, previous_block: &Block) -> bool {
        if (block.prev_hash != previous_block.hash) {
            warn!("invalid block, block's hash doesnt match previous block's hash");
            false
        } else if !(hex_to_binary_string(&hex::decode(&block.hash).expect("decode block hash"))
            .starts_with(DIFFICULTY_PREFIX))
        {
            false
        } else if (block.id != previous_block.id + 1) {
            false
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.prev_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            false
        } else {
            true
        }
    }
}
fn main() {}
