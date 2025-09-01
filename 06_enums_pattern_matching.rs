// 06_enums_pattern_matching.rs - 介绍Rust中的枚举和模式匹配

// 1. 定义枚举
// 枚举是一种自定义数据类型，它允许我们列举所有可能的值
enum IpAddrKind {
    V4,
    V6,
}

// 2. 枚举关联数据
enum IpAddr {
    V4(u8, u8, u8, u8), // V4变体关联了4个u8类型的值
    V6(String),         // V6变体关联了一个String类型的值
}

// 3. 枚举方法
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        // 这里可以根据枚举的不同变体执行不同的操作
        match self {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到坐标 x: {}, y: {}", x, y),
            Message::Write(text) => println!("写入消息: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
}

// 4. Option枚举 - Rust标准库中定义的枚举
// Option<T> 有两个变体：Some(T) 和 None

fn main() {
    // 5. 创建枚举实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 传递枚举值给函数
    route(four);
    route(six);

    // 6. 使用带关联数据的枚举
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // 7. 调用枚举方法
    let m1 = Message::Write(String::from("Hello, world!"));
    m1.call();

    let m2 = Message::Move { x: 10, y: 20 };
    m2.call();

    // 8. Option枚举的使用
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number包含值: {:?}", some_number);
    println!("some_string包含值: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // 9. 模式匹配（Pattern Matching）
    // match 表达式允许我们根据值的不同变体执行不同的代码
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("硬币的价值: {}美分", value);

    // 10. 匹配Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // 11. 匹配中的通配符和占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 其他情况都执行这个分支
        other => move_player(other),
    }

    // 12. if let 简洁控制流
    // 当我们只关心一个匹配模式时，可以使用if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("配置最大值为: {}", max);
    }

    // if let与else结合使用
    let mut count = 0;
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("来自{:?}州的25美分硬币!", state);
    } else {
        count += 1;
        println!("不是25美分硬币，计数增加到: {}", count);
    }
}

// 用于演示枚举的函数
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由IPv4地址"),
        IpAddrKind::V6 => println!("路由IPv6地址"),
    }
}

// 定义用于演示的硬币枚举
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter变体关联了一个UsState枚举
}

// 定义美国州枚举
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // 其他州省略
}

// 使用match表达式计算硬币的价值
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自{:?}州的25美分硬币!", state);
            25
        }
    }
}

// 使用match表达式处理Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 用于演示的简单函数
fn add_fancy_hat() {
    println!("添加华丽的帽子!");
}

fn remove_fancy_hat() {
    println!("移除华丽的帽子!");
}

fn move_player(num_spaces: u8) {
    println!("移动{}步!", num_spaces);
}
