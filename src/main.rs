use chrono::Utc;
use sha2::{Digest, Sha256};

struct Block {
    index: usize,
    data: BookCheckout,
    timestamp: u64,
    hash: String,
    previous_hash: String,
}

impl Block {
    pub fn new(index: usize, previous_hash: String, checkout_item: BookCheckout) -> Self {
        let block = Block {
            index,
            previous_hash,
            timestamp: Utc::now().timestamp() as u64,
            data: checkout_item,
            hash: String::new(),
        };

        Block {
            hash: block.hash(),
            ..block
        }
    }

    pub fn genesis() -> Self {
        let data = BookCheckout {
            is_genesis: true,
            ..Default::default()
        };

        let block = Block {
            index: 0,
            previous_hash: String::default(),
            timestamp: Utc::now().timestamp() as u64,
            data,
            hash: String::new(),
        };

        Block {
            hash: block.hash(),
            ..block
        }
    }

    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{:?}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );

        hasher.update(data.as_bytes());

        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Default)]
pub struct BookCheckout {
    book_id: String,
    user: String,
    checkout_date: String,
    is_genesis: bool,
}

pub struct Book {
    pub id: String,
    pub title: String,
    pub isbn: String,
}

impl Book {
    pub fn new(title: &str, isbn: &str) -> Self {
        let id = md5::compute(format!("{}{}", title, isbn).as_bytes());
        let id = format!("{:x}", id);

        Book {
            id,
            title: title.to_string(),
            isbn: isbn.to_string(),
        }
    }

    pub fn checkout(self, user: &str) -> BookCheckout {
        BookCheckout {
            book_id: self.id,
            is_genesis: false,
            user: user.to_string(),
            checkout_date: Utc::now().timestamp().to_string(),
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        let genesis = Block::genesis();
        chain.push(genesis);

        Blockchain { chain }
    }

    pub fn add_block(&mut self, data: BookCheckout) {
        let prev_block = self.chain.last().unwrap();
        let block = Block::new(prev_block.index + 1, prev_block.hash(), data);

        self.chain.push(block);
    }

    fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i > 0 && block.previous_hash != self.chain[i - 1].hash {
                return false;
            }
            if block.hash() != block.hash {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block #{} [Hash: {}, Prev. Hash: {}, Checkout data: {}]",
            self.index, self.hash, self.previous_hash, self.data
        )
    }
}
impl std::fmt::Display for BookCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Book ID #{} [User: {}, Checkout date: {}, Is genesis: {}]",
            self.book_id, self.user, self.checkout_date, self.is_genesis
        )
    }
}

impl std::fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.chain {
            writeln!(f, "{}", block)?;
        }

        Ok(())
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    let book_checkout = Book::new("Book1", "123456").checkout("User1");
    blockchain.add_block(book_checkout);

    let other_book_checkout = Book::new("Book2", "7890").checkout("User2");
    blockchain.add_block(other_book_checkout);

    if !blockchain.is_chain_valid() {
        panic!("invalid chain");
    }

    println!("{}", blockchain);
}
