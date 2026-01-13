use reqwest::Client;
use serde::Deserialize;
use tokio;
// 定义响应结构（根据 BSC API 调整）
#[derive(Deserialize)]
struct BscResponse {
    result: String, // 余额（单位：wei）
}

async fn fetch_bnb_balance(node_url: &str, wallet: &str) -> Result<u64, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(node_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [wallet, "latest"],
            "id": 1
        }))
        .send()
        .await?;

    let response: BscResponse = res.json().await?;
    let balance_wei = u64::from_str_radix(&response.result[2..], 16).unwrap(); // 16 进制转十进制
    Ok(balance_wei) // / 1_000_000_000_000_000_000) // 转换为 BNB 单位
}



#[tokio::main]
async fn main() {
    let wallet = "0xd30Fa7f4F7748636a0434E73c00fF1FEc64EC679"; // 替换为实际钱包地址

    // 并行发起两个异步请求
    let task1 = tokio::spawn(fetch_bnb_balance(
        "https://bsc-dataseed1.ninicoin.io",
        wallet,
    ));
    let task2 = tokio::spawn(fetch_bnb_balance(
        "https://bsc-dataseed2.binance.org",
        wallet,
    ));

    // 等待两个任务完成
    let (balance1, balance2) = tokio::join!(task1, task2);
    // 统一处理结果
    match (balance1, balance2) {
        // (Ok(Ok(b1)), Ok(Ok(b2))) => {
        //     println!("节点1余额: {} BNB", b1);
        //     println!("节点2余额: {} BNB", b2);
        //     println!("数据{}一致", if b1 == b2 { "Ok" } else { "不" });
        // }
        (Ok(b1), Ok(b2)) => {
            println!("节点1余额: {:?} BNB", b1);
            println!("节点2余额: {:?} BNB", b2);
            // println!("数据{}一致", if b1 == b2 { "Ok" } else { "不" });
        }
        (Err(e1), _) => eprintln!("节点1错误: {}", e1),
        (_, Err(e2)) => eprintln!("节点2错误: {}", e2),
        _ => eprintln!("其他错误"),
    }
    
}