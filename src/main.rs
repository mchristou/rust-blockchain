mod block;
mod chain;

use crate::{block::Book, chain::Blockchain};

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
