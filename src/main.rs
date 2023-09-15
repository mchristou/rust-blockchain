use chrono::Utc;
use sha2::{Digest, Sha256};

struct Block {
    index: usize,
    data: BookCheckout,
    timestamp: u64,
    hash: String,
    previous_hash: String,
    nonce: String,
}

impl Block {
    pub fn new(
        index: usize,
        previous_hash: String,
        difficulty: usize,
        checkout_item: BookCheckout,
    ) -> Self {
        let mut block = Block {
            index,
            previous_hash,
            timestamp: Utc::now().timestamp() as u64,
            data: checkout_item,
            hash: String::new(),
            nonce: String::new(),
        };

        for i in 0..u64::MAX {
            block.nonce = format!("{:x}", i);
            let hash = block.hash();

            if Self::validate_hash(hash.as_str(), difficulty) {
                return Block { hash, ..block };
            }
        }

        block
    }

    pub fn genesis() -> Self {
        let data = BookCheckout {
            is_genesis: true,
            ..Default::default()
        };

        Self::new(0, String::default(), 1, data)
    }

    fn validate_hash(hash: &str, difficulty: usize) -> bool {
        let prefix = "0".repeat(difficulty);

        hash.starts_with(prefix.as_str())
    }

    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
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
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::new();
        let genesis = Block::genesis();
        chain.push(genesis);

        Blockchain { chain, difficulty }
    }

    pub fn add_block(&mut self, data: BookCheckout) {
        let prev_block = self.chain.last().unwrap();
        let block = Block::new(
            prev_block.index + 1,
            prev_block.hash(),
            self.difficulty,
            data,
        );

        if self.block_is_valid(&block) {
            self.chain.push(block);
        }
    }

    fn block_is_valid(&mut self, new_block: &Block) -> bool {
        let prev_block = self.chain.last().expect("Blockchain has no valid blocks");

        prev_block.index + 1 == new_block.index
            && prev_block.hash == new_block.previous_hash
            && new_block.hash() == new_block.hash
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
            "Block #{} [Hash: {}, Prev. Hash: {}, Nonce: {}, Checkout data: {}]",
            self.index, self.hash, self.previous_hash, self.nonce, self.data
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
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {}", args[0]);
        return;
    }

    let difficulty = match args[1].parse::<usize>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid usize input");
            return;
        }
    };

    let mut blockchain = Blockchain::new(difficulty);

    let now = std::time::Instant::now();
    let book_checkout = Book::new("Book1", "123456").checkout("User1");
    blockchain.add_block(book_checkout);

    let other_book_checkout = Book::new("Book2", "7890").checkout("User2");
    blockchain.add_block(other_book_checkout);

    let elapsed = now.elapsed();

    if !blockchain.is_chain_valid() {
        panic!("invalid chain");
    }

    println!("{}", blockchain);
    println!("Time needed: {:?}", elapsed);
}
