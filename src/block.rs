use std::time::SystemTime; // 引入标准库中的 SystemTime，用于获取当前系统时间
use crypto::{digest::Digest, sha2::Sha256}; // 引入 crypto 库中的 Digest trait 和 Sha256 结构体，用于进行哈希运算
use log::info;
use serde::{Deserialize, Serialize}; // 引入 log 库中的 info 宏，用于日志记录

// 定义一个通用结果类型 Result，用于错误处理，T 表示成功的返回值类型，failure::Error 表示错误类型
use crate::{errors::Result, transaction::Transaction};

// 定义目标哈希的前缀长度为 4，表示我们需要找到哈希值前 4 位是 '0'
const TARGET_HEXT: usize = 4;

// 定义 Block 结构体，表示区块链中的区块
#[derive(Debug, Clone, Serialize, Deserialize)] // 派生 Debug 和 Clone trait，用于调试和复制
pub struct Block {
    timestamp: u128, // 时间戳，记录区块创建的时间
    transactions: Vec<Transaction>, // 交易信息，区块中包含的交易数据
    prev_block_hash: String, // 前一个区块的哈希值，形成链式结构
    hash: String, // 当前区块的哈希值
    height: usize, // 区块的高度，表示该区块在链中的位置
    nonce: i32 // 随机数，用于工作量证明算法
}

impl Block {
    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    
    pub(crate) fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
    }

    // 获取当前区块的哈希值，返回哈希值的副本
    pub fn get_hash(&self) -> String {
        self.hash.clone() // 返回区块哈希的副本
    }

    // 创建并返回创世区块（第一个区块）
    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        // 调用 new_block 函数创建创世区块，交易信息为 "Genesis Block"，前一区块哈希为空，区块高度为 0
        Block::new_block(vec![coinbase], String::new(), 0).unwrap()
    }

    // 创建新的区块，接收交易数据、前一区块的哈希值和区块高度作为参数，返回 Result 包含新创建的区块
    pub fn new_block(data: Vec<Transaction>, prev_block_hash: String, height: usize) -> Result<Block> {
        // 获取当前时间戳，以毫秒为单位
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)? // 计算自 Unix 纪元以来的时间
            .as_millis(); // 转换为毫秒表示

        // 创建 Block 实例，初始哈希值为空，nonce 为 0
        let mut block = Block {
            timestamp: timestamp,
            transactions: data,
            prev_block_hash: prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };

        // 运行工作量证明算法，寻找符合条件的哈希值
        block.run_proof_of_work()?;
        Ok(block) // 返回创建的区块
    }

    // 工作量证明算法，寻找符合目标的哈希值
    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block"); // 记录日志，表示开始挖矿
        // 不断递增 nonce，直到找到符合目标哈希前缀的哈希值
        while !self.validate()? {
            self.nonce += 1; // nonce 自增
        }
        // 准备区块的数据并进行哈希运算
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new(); // 创建一个新的 Sha256 哈希计算器
        hasher.input(&data[..]); // 输入要进行哈希计算的数据
        self.hash = hasher.result_str(); // 获取哈希值并赋值给区块的 hash 字段
        Ok(())
    }

    // 准备哈希计算的数据，将区块的多个字段序列化为字节数组
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        // 将前一区块哈希、交易数据、时间戳、目标前缀长度和 nonce 组成一个元组
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        // 使用 bincode 序列化库将元组序列化为字节数组
        let bytes = bincode::serialize(&content)?;
        Ok(bytes) // 返回序列化后的字节数组
    }

    // 验证当前区块的哈希是否符合目标，即哈希的前 TARGET_HEXT 位是否为 '0'
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?; // 准备哈希数据
        let mut hasher = Sha256::new(); // 创建 Sha256 哈希计算器
        hasher.input(&data[..]); // 输入要验证的数据
        let mut vec1: Vec<u8> = vec![]; // 创建一个用于比较的字节数组
        vec1.resize(TARGET_HEXT, '0' as u8); // 填充数组的前 TARGET_HEXT 个元素为 '0'
        // 检查生成的哈希值前 TARGET_HEXT 位是否为 '0'
        Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
    }
}


#[cfg(test)] // 测试模块，用于编写单元测试
mod tests {
    use crate::blockchain::Blockchain;

    #[test] // 测试函数
    fn test_add_block() -> Result<(), failure::Error> {
        let mut b = Blockchain::new()?; // 使用 `?` 解包结果
        // b.add_block("data".to_string())?;
        // b.add_block("data2".to_string())?;
        // b.add_block("data3".to_string())?;
        dbg!(b);
        Ok(())
    }
}