// 01_variables.rs - 介绍Rust中的变量和基本数据类型

fn main() {
    // 变量声明与初始化
    let x = 5;  // 默认不可变变量
    println!("x的值是: {}", x);
    
    // 可变变量需要使用mut关键字
    let mut y = 10;
    println!("y的初始值是: {}", y);
    y = 15;  // 可以修改可变变量的值
    println!("y修改后的值是: {}", y);
    
    // 声明变量类型
    let z: i32 = 20;  // 显式指定为32位整数
    println!("z的值是: {}", z);
    
    // 常量必须声明类型，且在编译时已知
    const MAX_POINTS: u32 = 100_000;  // 可以用下划线增强可读性
    println!("最大分数: {}", MAX_POINTS);
    
    // 隐藏（Shadowing）：可以使用相同的变量名重新定义变量
    let a = 5;
    println!("a的第一次定义: {}", a);
    let a = "hello";  // 隐藏了之前的a，并且类型可以不同
    println!("a的第二次定义: {}", a);
    
    // 基本数据类型示例
    let integer: i32 = -42;              // 有符号整数
    let unsigned_integer: u32 = 42;      // 无符号整数
    let floating_point: f64 = 3.14159;   // 浮点数
    let boolean: bool = true;            // 布尔值
    let character: char = 'z';           // 字符
    
    println!("整数: {}", integer);
    println!("无符号整数: {}", unsigned_integer);
    println!("浮点数: {}", floating_point);
    println!("布尔值: {}", boolean);
    println!("字符: {}", character);
}