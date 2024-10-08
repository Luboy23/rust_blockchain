
// 定义 Blockchain 结构体，表示整个区块链
use crate::block::Block;
use crate::errors::Result;
use failure::err_msg; // 引入 err_msg 函数
const TARGET_HEXT: usize = 4;

#[derive(Debug)] // 派生 Debug trait，用于调试
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIterator<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    // 创建并初始化一个包含创世区块的区块链
    pub fn new() -> Result<Blockchain> {
        let db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) => {
                let last_hash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    current_hash: last_hash,
                    db,
                })
            }
            None => {
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                let bc = Blockchain {
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }

    // 添加新的区块到区块链中
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let last_hash = match self.db.get("LAST")? {
            Some(hash) => hash,
            None => return Err(err_msg("No last block found. You may need to initialize the chain.")),
        };
        let new_block = Block::new_block(data, String::from_utf8(last_hash.to_vec())?, TARGET_HEXT)?; // 创建一个新的区块
    
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }

    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }
}

impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut b = Blockchain::new().unwrap();
        b.add_block("data 1".to_string());
        b.add_block("data 2".to_string());
        b.add_block("data 3".to_string());

        for item in b.iter() {
            println!("item {:?}", item)
        }
    }
}
