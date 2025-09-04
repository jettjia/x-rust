// async_simple_example.rs - Rust异步编程示例

// 引入必要的异步运行时
use tokio::time::{sleep, Duration};

// 主函数
fn main() {
    // 创建并启动tokio运行时，运行我们的异步主函数
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async_main());
}

// 异步主函数，使用async关键字标记
async fn async_main() {
    println!("=== 异步编程简单示例 ===");

    // 1. 基础的异步函数调用
    println!("\n1. 调用单个异步函数：");
    let result = simple_async_task(1).await;
    println!("异步任务结果: {}", result);

    // 2. 并发执行多个异步任务
    println!("\n2. 并发执行多个异步任务：");
    let task1 = simple_async_task(1);
    let task2 = simple_async_task(2);
    let task3 = simple_async_task(3);

    // 等待所有任务完成并获取结果
    let (result1, result2, result3) = tokio::join!(task1, task2, task3);

    println!("所有任务完成，结果: {}, {}, {}", result1, result2, result3);

    // 3. 演示异步代码的非阻塞特性
    println!("\n3. 演示异步代码的非阻塞特性：");
    let fast_task = simple_async_task_with_delay(1, 1); // 1秒后完成
    let slow_task = simple_async_task_with_delay(2, 3); // 3秒后完成

    // 先等待快任务完成
    let fast_result = fast_task.await;
    println!("快任务已完成: {}", fast_result);
    println!("此时慢任务仍在运行...");

    // 等待慢任务完成
    let slow_result = slow_task.await;
    println!("慢任务也已完成: {}", slow_result);

    println!("\n=== 示例执行完毕 ===");
}

// 一个简单的异步函数，接收一个ID参数并返回一个字符串
async fn simple_async_task(id: u32) -> String {
    println!("异步任务 {} 开始执行", id);
    // 模拟一些异步工作（例如网络请求或文件IO）
    sleep(Duration::from_secs(1)).await;
    format!("任务 {} 已完成", id)
}

// 一个带延迟的异步函数，接收任务ID和延迟时间（秒）
async fn simple_async_task_with_delay(id: u32, delay_seconds: u64) -> String {
    println!("异步任务 {} 开始执行，将持续 {} 秒", id, delay_seconds);
    sleep(Duration::from_secs(delay_seconds)).await;
    format!("耗时任务 {} 已完成（耗时 {} 秒）", id, delay_seconds)
}
