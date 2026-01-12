use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle_10 = tokio::spawn(async move{
        sleep(Duration::from_secs(10)).await;
        println!("sleep(10) 完成");
    });

    let handle_5 = tokio::spawn(async move{
        sleep(Duration::from_secs(5)).await;
        println!("sleep(5) 完成");
    });

    println!("aaa");

    // 等待所有任务完成
    let _ = tokio::join!(handle_10, handle_5);
}