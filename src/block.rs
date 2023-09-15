use chrono::Utc;
use sha2::{Digest, Sha256};

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

pub struct Block {
    pub index: usize,
    pub hash: String,
    pub previous_hash: String,
    nonce: String,
    data: BookCheckout,
    timestamp: u64,
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

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );

        hasher.update(data.as_bytes());

        format!("{:x}", hasher.finalize())
    }

    fn validate_hash(hash: &str, difficulty: usize) -> bool {
        let prefix = "0".repeat(difficulty);

        hash.starts_with(prefix.as_str())
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
