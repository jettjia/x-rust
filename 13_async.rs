// 13_async.rs - 介绍Rust中的异步编程

// Rust的异步编程主要通过async/await语法和Future trait实现
// 要使用异步功能，我们需要引入tokio或async-std等运行时

// 注意：要运行此示例，您需要在Cargo.toml中添加tokio依赖
// [dependencies]
// tokio = { version = "1", features = ["full"] }

// use tokio::time::{sleep, Duration};

fn main() {
    // 在实际项目中，我们会使用异步运行时来执行异步代码
    // 例如使用tokio运行时：
    // tokio::runtime::Runtime::new().unwrap().block_on(async_main());
    
    println!("异步编程示例");
    println!("注意：要运行完整的异步代码示例，需要添加tokio依赖");
    
    // 以下是异步编程的主要概念介绍
    // 1. async/await语法
    // 2. Future trait
    // 3. 异步函数
    // 4. 异步块
    // 5. 处理多个并发任务
    // 6. 异步IO操作
}

// 1. 定义异步函数
// 异步函数使用async关键字标记，返回Future类型
// async fn async_main() {
//     println!("开始执行异步任务");
//     
//     // 调用另一个异步函数并等待其完成
//     let result = do_something_async().await;
//     println!("异步任务结果: {}", result);
//     
//     // 并发执行多个异步任务
//     let future1 = do_something_async();
//     let future2 = do_something_else_async();
//     
//     // 等待两个任务都完成
//     let (result1, result2) = tokio::join!(future1, future2);
//     println!("并发任务结果: {}, {}", result1, result2);
//     
//     // 异步IO操作示例
//     // let content = read_file_async("hello.txt").await.unwrap();
//     // println!("文件内容: {}", content);
// }

// async fn do_something_async() -> String {
//     println!("开始执行第一个异步操作");
//     // 模拟耗时操作
//     sleep(Duration::from_secs(1)).await;
//     println!("第一个异步操作完成");
//     String::from("操作1完成")
// }

// async fn do_something_else_async() -> String {
//     println!("开始执行第二个异步操作");
//     // 模拟耗时操作
//     sleep(Duration::from_secs(2)).await;
//     println!("第二个异步操作完成");
//     String::from("操作2完成")
// }

// 2. 异步块
// 异步块是使用async关键字创建的表达式，返回Future
// async块示例：
// let future = async {
//     println!("在异步块中执行");
//     sleep(Duration::from_secs(1)).await;
//     "异步块完成"
// };

// 3. Future trait简介
// Future表示一个异步计算的值
// Future trait的简化定义如下：
// trait Future {
//     type Output;
//     fn poll(
//         self: Pin<&mut Self>, 
//         cx: &mut Context<'_>
//     ) -> Poll<Self::Output>;
// }

// 其中Poll是一个枚举，表示Future的状态：
// enum Poll<T> {
//     Ready(T),      // 计算完成，值为T
//     Pending,       // 计算尚未完成，需要再次poll
// }

// 4. 处理并发任务的几种方式
// a. 使用join!宏并发执行多个任务并等待所有任务完成
// let (result1, result2, result3) = tokio::join!(task1, task2, task3);

// b. 使用select!宏等待多个任务中的任何一个完成
// tokio::select! {
//     result = task1 => println!("任务1完成: {:?}", result),
//     result = task2 => println!("任务2完成: {:?}", result),
// }

// c. 使用spawn创建独立的异步任务
// let handle = tokio::spawn(async {
//     // 独立任务的代码
//     "任务完成"
// });
// 
// // 可以等待任务完成并获取结果
// let result = handle.await.unwrap();

// 5. 异步IO操作
// async fn read_file_async(path: &str) -> Result<String, std::io::Error> {
//     use tokio::fs::File;
//     use tokio::io::AsyncReadExt;
//     
//     let mut file = File::open(path).await?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).await?;
//     Ok(contents)
// }

// 6. 异步编程的优势
// - 高效处理IO密集型任务，不需要为每个连接创建线程
// - 代码结构清晰，接近同步代码的风格
// - 避免了回调地狱
// - 提高系统吞吐量

// 7. 异步编程的注意事项
// - 阻塞操作会阻塞整个执行器，应使用对应的异步版本
// - 要注意管理任务的生命周期，避免泄漏
// - 理解并正确使用await点
// - 注意处理错误，特别是在异步链中

// 8. 其他异步运行时
// 除了tokio，Rust还有其他异步运行时，如async-std
// [dependencies]
// async-std = { version = "1", features = ["full"] }

// 使用async-std运行时的示例：
// #[async_std::main]
// async fn main() {
//     // 异步代码
// }