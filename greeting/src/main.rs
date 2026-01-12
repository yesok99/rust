
use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct GreetingAccount {
    counter: u32,
    message: String,
    is_active: bool,
}


fn main() {


    test();

}
fn test() {
/* 
    //**************** 手动把数组转成字符串 ********************/
    let bytes = vec![2, 0, 0, 0, 104, 105];

    // 先取前 4 个字节作为长度
    let len_bytes: [u8; 4] = bytes[0..4].try_into().unwrap();
    let len = u32::from_le_bytes(len_bytes) as usize;

    // 再取后面的 len 个字节作为字符串内容
    let str_bytes = &bytes[4..4+len];
    let s = String::from_utf8(str_bytes.to_vec()).unwrap();

    println!("还原的字符串: {}", s); // 输出: hi

*/

/* 
    // //**************** 手动实现反序列化 ********************/
    // use std::io::Cursor;
    // use borsh::BorshDeserialize;

    // let bytes = vec![2, 0, 0, 0, 104, 105];
    // let mut reader = Cursor::new(&bytes);

    // let s2 = String::deserialize_reader(&mut reader).unwrap();
    // println!("反序列化得到: {}", s2);
*/
    
    ////**************** 序列化结构 ********************/
    /// 1.自动
    let account = GreetingAccount {
        counter: 10,
        message: "AABBCC".to_string(),
        is_active: true,
    };

    let bytes = account.try_to_vec().unwrap();
    println!("自动序列化结果： {:?}", bytes);

    // 2. 手动调用 serialize，写入到 buf
    let mut buf = Vec::new();
    account.serialize(&mut buf).unwrap();
    println!("手动序列化结果: {:?}", buf);


    ///**************** 反序列化 ********************/
    let account2 = GreetingAccount::try_from_slice(&bytes).unwrap();
    
    print!("{:?}",account2)
    // assert_eq!(account.counter, account2.counter);
    // assert_eq!(account.message, account2.message);
    // assert_eq!(account.is_active, account2.is_active);

    // account.message.bytes().for_each(|b| {
    //     println!("{:02X}", b);
    // });

    // let bytes1 = account.message.bytes().collect::<Vec<u8>>();
    // println!("{:02X?}", bytes);
    // let bytes = account.message.as_bytes();
    // println!("{:04x?}", bytes);


}


// 写入文件
// use std::io::prelude::*;
// use std::fs::File;

// fn main() -> std::io::Result<()> {
//     let mut buffer = File::create("foo.txt")?;

//     buffer.write_all(b"some bytes")?;
//     Ok(())

// }

/* 
impl GreetingAccount {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // counter
        buf.extend(&self.counter.to_le_bytes());

        // message
        let msg_bytes = self.message.as_bytes();
        buf.extend(&(msg_bytes.len() as u32).to_le_bytes());
        buf.extend(msg_bytes);

        // is_active
        buf.push(if self.is_active { 1 } else { 0 });

        buf
    }
}
*/