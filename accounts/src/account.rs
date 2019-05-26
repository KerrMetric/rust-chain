use rand::Rng;
use core::transaction::Transaction;
use utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub balance: i64,
    pub miner: bool,
}

impl Account {
    pub fn new() -> Self {
        Self { address: utils::hash::generate(format!("{}", rand::thread_rng().gen_range(1, 10001))),
            balance: 0,
            miner: false
        }
    }

    pub fn send_transaction(&self, to_address: String, value: i64, fee: i64) -> Transaction {
        return Transaction::new(self.address.clone(), to_address, value, fee)
    }
}
