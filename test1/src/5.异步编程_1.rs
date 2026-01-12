use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 启动两个异步 sleep 任务（不阻塞）
    let sleep_10 = sleep(Duration::from_secs(10));
    let sleep_5 = sleep(Duration::from_secs(5));
    
    // 这两个任务会在后台运行，不阻塞当前线程
    tokio::spawn(async move {
        sleep_10.await;
        println!("sleep(10) 完成");
    });
    
    tokio::spawn(async move {
        sleep_5.await;
        println!("sleep(5) 完成");
    });

    // 主线程继续执行，立即打印
    println!("aaa");
    
    // 防止主函数提前退出（可选）
    // tokio::time::sleep(Duration::from_secs(11)).await;
}