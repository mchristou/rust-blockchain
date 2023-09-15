use crate::block::{Block, BookCheckout};

pub struct Blockchain {
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

    pub fn is_chain_valid(&self) -> bool {
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

    fn block_is_valid(&mut self, new_block: &Block) -> bool {
        let prev_block = self.chain.last().expect("Blockchain has no valid blocks");

        prev_block.index + 1 == new_block.index
            && prev_block.hash == new_block.previous_hash
            && new_block.hash() == new_block.hash
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
