// 02_functions.rs - 介绍Rust中的函数定义和使用

// 函数定义：使用fn关键字，后面跟着函数名、参数列表、返回类型和函数体
fn main() {
    println!("主函数开始执行");
    
    // 调用无参数无返回值的函数
    print_message();
    
    // 调用带参数的函数
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // 调用带多个参数并返回多个值的函数
    let (product, difference) = calculate(10, 4);
    println!("10 * 4 = {}, 10 - 4 = {}", product, difference);
    
    // 调用有条件返回的函数
    let result = check_number(7);
    println!("检查数字7的结果: {}", result);
    
    // 调用递归函数
    let factorial_result = factorial(5);
    println!("5的阶乘是: {}", factorial_result);
    
    println!("主函数执行结束");
}

// 无参数无返回值的函数
fn print_message() {
    println!("这是一个简单的函数");
}

// 带参数和返回值的函数
// 参数格式：参数名: 类型
// 返回类型使用->指定
fn add(x: i32, y: i32) -> i32 {
    // Rust中最后一个表达式的结果会自动作为返回值
    // 注意：没有分号的表达式才会返回值
    x + y
}

// 返回多个值（使用元组）
fn calculate(a: i32, b: i32) -> (i32, i32) {
    // 返回元组 (乘积, 差)
    (a * b, a - b)
}

// 条件返回函数
fn check_number(n: i32) -> String {
    if n > 0 {
        return String::from("正数");
    } else if n < 0 {
        return String::from("负数");
    } else {
        return String::from("零");
    }
}

// 递归函数
fn factorial(n: u32) -> u32 {
    // 基本情况：0和1的阶乘是1
    if n <= 1 {
        1
    } else {
        // 递归调用
        n * factorial(n - 1)
    }
}