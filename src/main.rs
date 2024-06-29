
use hex;
use sha2::{Sha256, Digest};

#[derive(Clone, Debug, Hash)]
struct Node {
    hash_state: String,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(hash: String) -> Node {
        Node {
            hash_state: hash,
            next: None,
        }
    }

    fn insert(&mut self, node: Node) {
        if self.next.is_none() {
            self.next = Some(Box::new(node));
        } else {
            let next = self.next.as_mut().unwrap();
            next.insert(node);
        }
    }
}

fn build_chain(root_hash: String, transactions: Vec<String>) -> Node {
    // Create a new node with the root hash
    let mut root = Node::new(root_hash);

    // Iterate over the transactions
    for transaction in transactions {
        // Hash the transaction
        let tx_hash = Sha256::digest(transaction.as_bytes());
        // Create a new node with the transaction hash
        let node = Node::new(hex::encode(tx_hash));
        // Insert the node into the root
        root.insert(node);

        // Create a combined hash of the transaction and the root hash state
        let mut combined_hash_vec = tx_hash.to_vec();
        combined_hash_vec.extend(hex::decode(&root.hash_state).expect("Failed to decode hash state"));
        
        // Hash the combined hash vector
        let combined_hash = format!("{:x}", Sha256::digest(&combined_hash_vec));

        // Update the root hash state
        root.hash_state = combined_hash;
    }
    // Return the root node
    root.clone()
}

fn main() {
    // Example root hash
    let root_hash = Sha256::digest("hello world".as_bytes());    
    // Example transactions
    let transactions = vec![
        "transaction1".to_string(),
        "transaction2".to_string(),
        "transaction3".to_string(),
        "transaction4".to_string(),
    ];
    // Build the chain
    let chain = build_chain(format!("{:x}", root_hash), transactions);
    // Pretty print the chain
    println!("Chain: {:#?}", chain);
}