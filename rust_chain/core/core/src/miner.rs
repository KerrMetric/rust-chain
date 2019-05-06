extern crate crypto;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

use chrono::Local;

use crate::block::Block;
use crate::block::Header;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Miner {
    pub(crate) block_chain: Vec<Block>,
}

impl Miner {
    pub fn new() -> Miner {
        Miner { block_chain: vec![] }
    }

    pub fn print(&self) {
        for block in &self.block_chain {
            // FIXME: JSONで出力
            println!("{:?}", block);
        }
    }

    pub fn pow(&mut self, transactions: Vec<Transaction>) {
        let (target_height, parent_hash) = match self.block_chain.last() {
            Some(parent_block) => (parent_block.height + 1, parent_block.header.block_hash.to_string()),
            None => (0, "0x0000000000000000000000000000000000000000".to_string()),
        };
        let result = self.exec(&parent_hash);
        let block = self.create_block(target_height, &parent_hash, result, transactions);
        self.block_chain.push(block);
    }

    fn exec(&self, parent_hash: &String) -> (String, i64, i64) {
        let target = "00001000111111111111111111111111111111111111111111111111111111".to_string();
        let mut hash = "11111111111111111111111111111111111111111111111111111111111111".to_string();
        let mut nonce: i64 = 0;
        let mut time_stamp = Local::now().timestamp();

        while hash > target {
            nonce += 1;
            time_stamp = Local::now().timestamp();
            hash = self.calc(parent_hash, nonce, time_stamp);
        }

        (format!("0x{}", hash), nonce, time_stamp)
    }

    fn calc(&self, parent_hash: &String, nonce: i64, time_stamp: i64) -> String {
        let raw_data = format!("{}{}{}", parent_hash, nonce.to_string(), time_stamp.to_string());
        let mut hasher = Sha256::new();
        hasher.input_str(&raw_data);
        hasher.result_str()
    }

    fn create_block(&self,
                    target_height: i32,
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
}
