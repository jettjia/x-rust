// 16_multithreading.rs - 介绍Rust中的多线程和并发

// 导入必要的模块
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 基础线程创建
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(200));
    }

    // 等待子线程完成
    handle.join().unwrap();

    // 2. 向线程传递数据
    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("线程中的数据: {:?}", data);
        // 注意：data的所有权已经移动到了线程中
    });

    handle.join().unwrap();

    // 3. 共享状态 - 使用Arc和Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // MutexGuard离开作用域时自动释放锁
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    // 4. 读写锁 RwLock
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // 创建5个读取线程
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_data = data.read().unwrap();
            println!("读取线程 {} 读取数据: {:?}", i, *read_data);
            thread::sleep(Duration::from_millis(50));
        });
        handles.push(handle);
    }

    // 创建2个写入线程
    for i in 0..2 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_data = data.write().unwrap();
            write_data.push(i * 10);
            println!("写入线程 {} 更新数据: {:?}", i, *write_data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 5. 通道 (Channels)
    let (tx, rx) = mpsc::channel();

    // 创建多个发送者
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send(String::from("来自线程1的消息")).unwrap();
        thread::sleep(Duration::from_millis(100));
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        tx2.send(String::from("来自线程2的消息")).unwrap();
        thread::sleep(Duration::from_millis(50));
    });

    thread::spawn(move || {
        tx.send(String::from("来自线程3的消息")).unwrap();
    });

    // 接收消息
    for received in rx {
        println!("收到: {}", received);
    }

    // 6. 条件变量 (Condvar)
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("子线程: 正在准备...");
        thread::sleep(Duration::from_secs(2));
        *started = true;
        println!("子线程: 已准备就绪!");
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("主线程: 等待子线程准备...");
        started = cvar.wait(started).unwrap();
    }
    println!("主线程: 检测到子线程已准备就绪!");

    // 7. 原子操作
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("原子计数器结果: {}", counter.load(Ordering::Relaxed));

    // 8. 线程本地存储
    thread_local!(static VALUE: AtomicUsize = AtomicUsize::new(1));

    // 在主线程中访问线程本地存储
    VALUE.with(|v| println!("主线程初始值: {}", v.load(Ordering::Relaxed)));
    VALUE.with(|v| v.store(10, Ordering::Relaxed));
    VALUE.with(|v| println!("主线程修改后: {}", v.load(Ordering::Relaxed)));

    // 在子线程中访问线程本地存储
    thread::spawn(|| {
        VALUE.with(|v| println!("子线程初始值: {}", v.load(Ordering::Relaxed)));
        VALUE.with(|v| v.store(20, Ordering::Relaxed));
        VALUE.with(|v| println!("子线程修改后: {}", v.load(Ordering::Relaxed)));
    })
    .join()
    .unwrap();

    // 再次检查主线程的值
    VALUE.with(|v| println!("主线程最终值: {}", v.load(Ordering::Relaxed)));

    // 9. 线程池
    // 注意：标准库没有内置的线程池，但可以使用第三方库如rayon
    // 这里只是简单模拟线程池的概念
    simulate_thread_pool();

    // 10. 线程的优雅退出
    // 使用join来等待线程完成

    // 11. 多线程的最佳实践
    println!("\n多线程最佳实践:");
    println!("1. 优先使用通道(channel)进行线程间通信，而非共享状态");
    println!("2. 使用Arc和Mutex/RwLock安全地共享状态");
    println!("3. 尽量减少共享状态的数量");
    println!("4. 使用join等待所有线程完成");
    println!("5. 对于简单的并行计算，考虑使用rayon等第三方库");
    println!("6. 注意避免死锁");
    println!("7. 考虑使用原子操作代替锁以提高性能");
}

// 模拟简单的线程池
fn simulate_thread_pool() {
    let (tx, rx) = mpsc::channel();
    // 使用Arc<Mutex<>>来安全地在线程间共享单一的Receiver
    let shared_rx = Arc::new(Mutex::new(rx));

    // 创建固定数量的工作线程
    let num_threads = 4;
    let mut handles = vec![];

    for id in 0..num_threads {
        let rx = Arc::clone(&shared_rx);
        let handle = thread::spawn(move || {
            println!("工作线程 {} 已启动", id);
            // 工作线程需要循环尝试获取锁并接收消息
            loop {
                let mut lock = match rx.lock() {
                    Ok(lock) => lock,
                    Err(_) => break, // 如果互斥锁被污染，则退出线程
                };
                match lock.try_recv() {
                    Ok(task) => {
                        println!("工作线程 {} 执行任务: {}", id, task);
                        // 模拟工作
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(TryRecvError::Empty) => {
                        // 通道为空，但发送者可能还存在
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(TryRecvError::Disconnected) => {
                        // 所有发送者都已断开连接
                        break;
                    }
                }
            }
            println!("工作线程 {} 已关闭", id);
        });
        handles.push(handle);
    }

    // 提交任务
    for i in 0..10 {
        tx.send(format!("任务 {}", i)).unwrap();
    }

    // 关闭发送端，使接收端接收到None
    drop(tx);

    // 等待所有工作线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有任务已完成");
}

// 12. 避免死锁的示例
fn deadlock_prevention() {
    // 死锁通常发生在多个线程互相等待对方持有的资源时
    // 避免死锁的策略包括：
    // 1. 以固定的顺序获取锁
    // 2. 避免持有锁的同时进行可能导致等待的操作
    // 3. 设置超时
    // 4. 使用RwLock而不是Mutex（如果读操作远多于写操作）

    // 这里是一个以固定顺序获取锁的例子
    let resource_a = Arc::new(Mutex::new("资源A"));
    let resource_b = Arc::new(Mutex::new("资源B"));

    let a1 = Arc::clone(&resource_a);
    let b1 = Arc::clone(&resource_b);

    let handle1 = thread::spawn(move || {
        let lock_a = a1.lock().unwrap();
        println!("线程1: 持有资源A");
        thread::sleep(Duration::from_millis(100));
        let lock_b = b1.lock().unwrap();
        println!("线程1: 持有资源B和A");
        // 这里可以使用两个资源
    });

    let a2 = Arc::clone(&resource_a);
    let b2 = Arc::clone(&resource_b);

    let handle2 = thread::spawn(move || {
        // 注意：这里也是先获取resource_a，再获取resource_b
        // 与线程1的顺序相同，避免死锁
        let lock_a = a2.lock().unwrap();
        println!("线程2: 持有资源A");
        thread::sleep(Duration::from_millis(100));
        let lock_b = b2.lock().unwrap();
        println!("线程2: 持有资源B和A");
        // 这里可以使用两个资源
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
