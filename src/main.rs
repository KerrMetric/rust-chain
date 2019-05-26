use core::node::Node;
use core::transaction::Transaction;
use core::miner::Miner;

fn main() {
    println!("Start Rust Chain!");

    let mut node = Node::new(vec!());
    for _ in 1..=3 {
        let transactions = vec!(Transaction::dummy_new());
        node.mining(transactions);
    }
    node.print();
}