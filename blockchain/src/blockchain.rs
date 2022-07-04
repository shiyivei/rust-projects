extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use time::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    //交易三部分
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    //区块头：五部分
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    //区块包含三部分，头、交易和区块高度
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
pub struct Chain {
    // 链五部分：由区块组成的链和由交易组成的交易序列
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        //实例化一个chain
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        //利用chain创建区块
        chain.generate_new_block();
        chain
    }

    //创建新交易
    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }

    //获取链上最新区块的哈希值
    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };

        Chain::hash(&block.header)
    }

    //更新区块难度
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    //更新挖矿奖励
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    //创建新的区块
    pub fn generate_new_block(&mut self) -> bool {
        //获取当前系统时间
        let time_now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        //1.初始化区块头
        let header = BlockHeader {
            timestamp: time_now as i64,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        //2.奖励，实际上是一笔交易，有sender， receiver，amount
        let reward_trans = Transaction {
            //把奖励发出者写死
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        //2.利用上面的数据填充区块
        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        //push是装元素的
        block.transactions.push(reward_trans);
        //append是连接两个vector的
        //前者是区块中的交易，后者是整个链上所有交易
        block.transactions.append(&mut self.curr_trans);
        //区块计数是区块中的交易数量
        block.count = block.transactions.len() as u32;
        //把区块中的交易组织成默克尔树
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        //1.新建vec
        let mut merkle = Vec::new();

        //2.遍历交易，将其哈希装入vec
        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        //2.计算vec长度的奇偶性
        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap(); //默克尔树都是双数哦
            merkle.push(last);
        }

        while merkle.len() > 1 {
            //用了一个while循环，merkle树就构建成了
            let mut h1 = merkle.remove(0); //vec自带remove函数
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2.clone()); //链接字符串
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap() //返回根哈希
    }

    //工作量证明
    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize]; //加密切片吗，可以的，因为参数是泛型
                                                             //[0,1,2..]

            //解析难度
            match slice.parse::<i32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block Hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();

        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}
