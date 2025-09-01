// 03_flow_control.rs - 介绍Rust中的流程控制语句

fn main() {
    // 1. if 条件表达式
    let number = 7;

    if number > 10 {
        println!("数字大于10");
    } else if number < 5 {
        println!("数字小于5");
    } else {
        println!("数字在5到10之间");
    }

    // if表达式可以作为值赋给变量
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("根据条件赋值的结果: {}", number);

    // 2. loop循环 - 无限循环，直到显式退出
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break可以带一个值作为循环的返回值
            break counter * 2;
        }
    };
    println!("loop循环的结果: {}", result);

    // 3. while循环 - 条件为真时继续执行
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("升空!");

    // 4. for循环 - 用于遍历集合
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("元素值: {}", element);
    }

    // for循环与Range结合使用
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("升空!");

    // for循环使用索引
    for (index, value) in a.iter().enumerate() {
        println!("索引 {} 的值是 {}", index, value);
    }

    // 5. match表达式 - 类似于switch，但更强大
    let value = 3;

    match value {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        // _ 是通配符，匹配所有其他情况
        _ => println!("其他数字"),
    }

    // match也可以返回值
    let status = match check_status(200) {
        Status::Success => "成功",
        Status::Error => "错误",
        Status::Pending => "等待中",
    };
    println!("操作{}", status);
}

// 用于match表达式的简单枚举
enum Status {
    Success,
    Error,
    Pending,
}

// 根据状态码返回对应的Status枚举值
fn check_status(code: u32) -> Status {
    match code {
        200 => Status::Success,
        400..=499 => Status::Error,
        500..=599 => Status::Error,
        _ => Status::Pending,
    }
}
