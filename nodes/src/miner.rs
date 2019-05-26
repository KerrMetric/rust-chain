use chrono::Local;

use core::transaction::Transaction;
use core::block::Block;
use core::block::Header;
use utils::hash;
use crate::node::Node;

pub trait Miner {
    fn mining(&mut self, transactions: Vec<Transaction>);
}

impl Miner for Node {
    fn mining(&mut self, transactions: Vec<Transaction>) {
        let (target_height, parent_hash) = match self.block_chain.last() {
            Some(parent_block) => (parent_block.height + 1, parent_block.header.block_hash.to_string()),
            None => (0, "0x0000000000000000000000000000000000000000".to_string()),
        };
        let result = pow(&parent_hash);
        let block = create_block(target_height, &parent_hash, result, transactions);
        self.block_chain.push(block);
    }
}

fn pow(parent_hash: &String) -> (String, i64, i64) {
    let target = "00011000111111111111111111111111111111111111111111111111111111".to_string();
    let mut hash = "11111111111111111111111111111111111111111111111111111111111111".to_string();
    let mut nonce: i64 = 0;
    let mut time_stamp = Local::now().timestamp();

    while hash > target {
        nonce += 1;
        time_stamp = Local::now().timestamp();
        hash = calc(parent_hash, nonce, time_stamp);
    }

    (format!("0x{}", hash), nonce, time_stamp)
}

fn calc(parent_hash: &String, nonce: i64, time_stamp: i64) -> String {
    hash::generate(format!("{}{}{}", parent_hash, nonce.to_string(), time_stamp.to_string()))
}

fn create_block(target_height: i32,
                parent_hash: &String,
                pow_result: (String, i64, i64),
                transactions: Vec<Transaction>) -> Block {
    let header = Header { parent_hash: parent_hash.to_string(),
                block_hash: pow_result.0,
                nonce: pow_result.1,
                transactions: transactions,
                time_stamp: pow_result.2,
                };
    // TODO: Block Sizeの計算
    Block { height: target_height, size: 0, header: header, }
}