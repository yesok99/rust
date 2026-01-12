
use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct GreetingAccount {
    // pub counter: u32,
    pub message: String,
    // pub is_active: bool,
}
pub fn main() {
    // 1. 构造一个 GreetingAccount
    let account = GreetingAccount {
        // counter: 1,
        message: "Hello".to_string(),
        // is_active: true,
    };

    // 2. 序列化成字节
    let serialized = account.try_to_vec().unwrap();
    println!("序列化后的字节: {:?}", serialized);
 
    // 3. 反序列化回来
    let deserialized = GreetingAccount::try_from_slice(&serialized).unwrap();
    println!("反序列化结果: {:?}", deserialized);
}
