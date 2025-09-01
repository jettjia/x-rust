// 14_macros.rs - 介绍Rust中的宏（Macros）

// 宏是Rust中的一种元编程工具，允许我们编写可以生成其他代码的代码

// 导入标准库中的宏
use std::collections::HashMap;

// 1. 声明宏（Declarative Macros）
// 使用macro_rules!定义声明宏
macro_rules! say_hello {
    // 匹配模式：无参数
    () => {
        println!("Hello!");
    };
    // 匹配模式：带一个参数
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// 2. 实现vec!类似的宏
macro_rules! vec2 {
    // 匹配空列表
    () => {
        Vec::new()
    };
    // 匹配一个或多个表达式，使用重复模式
    ($($element:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($element);)+  // 重复生成push语句
            temp_vec
        }
    };
}

// 3. 实现自定义的assert!宏
macro_rules! my_assert {
    // 基本形式：只有条件
    ($condition:expr) => {
        if !$condition {
            panic!("断言失败: {}", stringify!($condition));
        }
    };
    // 带消息的形式
    ($condition:expr, $($message:tt)+) => {
        if !$condition {
            panic!("断言失败: {}: {}", stringify!($condition), format_args!($($message)+));
        }
    };
}

// 4. 实现一个简单的日志宏
macro_rules! log {
    // 不同级别的日志
    (info, $($args:tt)+) => {
        println!("[INFO] {}", format_args!($($args)+))
    };
    (warn, $($args:tt)+) => {
        println!("[WARN] {}", format_args!($($args)+))
    };
    (error, $($args:tt)+) => {
        println!("[ERROR] {}", format_args!($($args)+))
    };
}

// 5. 实现一个简单的测试宏
macro_rules! test {
    ($name:ident, $body:block) => {
        #[test]
        fn $name() $body
    };
}

// 6. 实现一个简单的html!宏（类似一些前端框架中的宏）
macro_rules! html {
    ($tag:ident) => {
        format!("<{}></{}>", stringify!($tag), stringify!($tag))
    };
    ($tag:ident { $($attr:ident = $val:expr),* }) => {
        {
            let mut result = format!("<{}", stringify!($tag));
            $(result.push_str(&format!(" {}=\"{}\"", stringify!($attr), $val));)*
            result.push_str(">");
            result.push_str("</");
            result.push_str(stringify!($tag));
            result.push_str(">");
            result
        }
    };
    ($tag:ident $content:expr) => {
        format!("<{}>{}</{}>", stringify!($tag), $content, stringify!($tag))
    };
}

fn main() {
    // 测试say_hello宏
    say_hello!();
    say_hello!("Rust");

    // 测试vec2宏
    let v1: Vec<i32> = vec2![];
    let v2 = vec2![1, 2, 3, 4, 5];
    let v3 = vec2![1, 2, 3, 4, 5,]; // 支持尾部逗号

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    // 测试my_assert宏
    my_assert!(1 + 1 == 2);
    // 取消下面的注释会导致程序崩溃
    // my_assert!(1 + 1 == 3, "数学计算错误");

    // 测试log宏
    log!(info, "这是一条信息日志");
    log!(warn, "这是一条警告日志");
    log!(error, "这是一条错误日志，编号: {}", 404);

    // 测试html宏
    let div1 = html!(div);
    let div2 = html!(div { class="container", id="main" });
    let div3 = html!(div html!(p "Hello World"));

    println!("div1: {}", div1);
    println!("div2: {}", div2);
    println!("div3: {}", div3);

    // 7. 标准库中的常用宏
    // println! - 打印到标准输出并换行
    println!("这是println!宏");

    // format! - 创建格式化的字符串
    let formatted = format!("这是一个{}字符串", "格式化的");
    println!("{}", formatted);

    // vec! - 创建向量
    let v = vec![1, 2, 3, 4, 5];
    println!("vec!: {:?}", v);

    // assert! - 断言条件为真
    assert!(v.len() == 5);

    // Option和Result相关宏
    let some_value: Option<i32> = Some(5);
    let none_value: Option<i32> = None;

    // unwrap!宏（简化版）
    let value = some_value.unwrap();
    println!("unwrap后的 value: {}", value);

    // 取消下面的注释会导致程序崩溃
    // let value = none_value.unwrap();

    // 8. 更多宏的高级特性
    // 宏可以捕获变量
    let x = 5;
    let y = 10;
    macro_rules! add_values {
        () => {
            x + y // 捕获外部变量x和y
        };
    }

    println!("x + y = {}", add_values!());

    // 宏可以递归调用
    macro_rules! count {
        () => {0};
        ($x:expr) => {1};
        ($x:expr, $($xs:expr),*) => {1 + count!($($xs),*)};
    }

    println!("参数数量: {}", count!(1, 2, 3, 4, 5));
}

// 9. 过程宏（Procedural Macros）简介
// 过程宏是更复杂的宏，可以通过Rust代码操作语法树
// 过程宏分为三种类型：
// a. 派生宏（Derive Macros）：为结构体、枚举等实现trait
// b. 属性宏（Attribute Macros）：定义自定义属性
// c. 函数式宏（Function-like Macros）：类似声明宏，但使用Rust代码实现

// 注意：过程宏需要在单独的crate中定义，并且需要特殊的配置

// 派生宏示例（只是展示，不会实际工作）
// #[derive(MyDebug)]
// struct Person {
//     name: String,
//     age: u32,
// }

// 属性宏示例
// #[route("/api/users")]
// fn get_users() {
//     // 函数体
// }

// 10. 宏的优缺点
// 优点：
// - 减少重复代码
// - 提供编译时元编程能力
// - 可以创建DSL（领域特定语言）
//
// 缺点：
// - 语法复杂，较难理解
// - 编译错误信息可能不友好
// - 可能导致代码膨胀

// 11. 宏的最佳实践
// - 优先使用函数，只有在需要元编程时才使用宏
// - 保持宏的简单性，避免过于复杂的宏
// - 为宏提供清晰的文档和注释
// - 测试宏的各种使用场景
