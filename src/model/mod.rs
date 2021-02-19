extern crate sha2;

mod utils;

#[derive(Debug)]
pub struct Block {
    index: usize,
    timestamp: u128,
    hash: String,
    previous: String,
    data: String,
    nonce: u32,
}

impl Block {
    pub fn new(
        index: usize,
        _timestamp: u128,
        previous: String,
        data: String
    ) -> Block {
        let time = utils::get_time();
        let mut new_b: Block = Block {
            index: index,
            timestamp: time,
            hash: String::from(""),
            previous: previous,
            data: data,
            nonce: 0,
        };
        let h = utils::calc_hash(
            to_string(&new_b)
        );
        new_b.hash = h;
        return new_b;
    }
    pub fn mine_block(&mut self,difficulty: usize) {
        while &self.hash[0..difficulty] != utils::fmt_zeros(difficulty) {
            self.nonce += 1;
            self.hash = utils::calc_hash(
                to_string(&self)
            );
        }
        
    }
}

pub struct Blockchain {
    difficulty: usize,
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Blockchain {
        let f = String::from("");
        let time = utils::get_time();
        let first = Block::new(0,time,f,String::from("Queen Ro"));
        let base = vec![first];
        Blockchain {
            difficulty: difficulty,
            blocks: base
        }
    }
    
    pub fn new_block(&mut self, data: String) {
        let prev = &self.get_previous();
        let time = utils::get_time();
        if self.is_first_valid() {
            let mut b = Block::new(
                self.set_index(),
                time,
                prev.hash.to_owned(),
                data
            );
            b.mine_block(self.difficulty);
            self.add_block(b);
        }
    }
    fn add_block(&mut self,b: Block) {
        self.blocks.push(b);
    }
    fn get_previous(&self) ->  &Block {
        let prev = self.blocks.len() - 1;
        &self.blocks[prev]
    }
    fn set_index(&self) -> usize {
        let next = self.blocks.len() + 1;
        next
    }
    fn is_first_valid(&self) -> bool {
        let block = &self.blocks[0];
        if block.index != 0 {
            return false;
        }
        if block.previous.len() > 0 {
            return false;
        }
        if block.hash.len() == 0
            || block.hash != utils::calc_hash(to_string(&block))  {
            return false;
        }
        return true;
    }
    fn is_last_valid(&self) -> bool {
        let last = self.blocks.len() - 1;
        let bef = last - 1;
        if  self.blocks[bef].hash != self.blocks[last].previous {
            return false;
        }
        return true;
    }
    pub fn is_chain_valid(&self) -> bool {
        if !self.is_last_valid() || !self.is_first_valid() {
            return false;
        }
        return true;
    }
}

fn to_string(b: &Block) -> String {
    let mut to_str = String::new();
    to_str.push_str(&b.index.to_owned().to_string());
    to_str.push_str(&format!("{:?}",&b.timestamp.to_owned()));
    to_str.push_str(&b.previous.to_owned().to_string());
    to_str.push_str(&b.data.to_owned().to_string());
    to_str.push_str(&b.nonce.to_owned().to_string());
    return to_str;
}