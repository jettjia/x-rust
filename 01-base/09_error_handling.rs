// 09_error_handling.rs - 介绍Rust中的错误处理机制

// Rust有两种主要的错误处理方式：
// 1. 不可恢复的错误：使用panic!宏使程序崩溃
// 2. 可恢复的错误：使用Result<T, E>枚举

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // 1. 不可恢复的错误 - panic!
    // 当程序遇到无法处理的错误时，可以使用panic!宏
    // 取消下面的注释查看panic的效果
    // panic!("程序崩溃了!");

    // 2. 可恢复的错误 - Result枚举
    // Result<T, E>有两个变体：Ok(T)表示成功，包含成功的值；Err(E)表示错误，包含错误信息
    let result = File::open("hello.txt");

    let file = match result {
        Ok(file) => file,
        Err(error) => panic!("打开文件时发生错误: {:?}", error),
    };

    // 3. 匹配不同类型的错误
    let result2 = File::open("hello.txt");

    let file2 = match result2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件时发生错误: {:?}", other_error),
        },
    };

    // 4. 使用unwrap和expect简化错误处理
    // unwrap：如果Result是Ok，返回Ok中的值；如果是Err，调用panic!
    let file3 = File::open("hello.txt").unwrap();

    // expect：类似于unwrap，但可以提供自定义的错误信息
    let file4 = File::open("hello.txt").expect("无法打开hello.txt文件");

    // 5. 传播错误 - 使用?操作符
    // 当函数需要返回错误给调用者时，可以使用?操作符
    // ?操作符会将Result中的Ok值解包，如果是Err则直接返回该错误
    let content = read_username_from_file();
    println!("用户名: {:?}", content);

    // 简化版的read_username_from_file
    let content2 = read_username_from_file_simplified();
    println!("用户名(简化版): {:?}", content2);

    // 更简洁的版本，使用链式调用
    let content3 = read_username_from_file_short();
    println!("用户名(简短版): {:?}", content3);

    // 6. 在main函数中使用?操作符
    // 注意：main函数的返回类型必须是Result<(), E>才能使用?
    // 取消下面的注释查看效果
    // let result = main_with_result();
    // println!("main_with_result结果: {:?}", result);

    // 7. 自定义错误类型
    let result = validate_username("user123");
    println!("验证用户名结果: {:?}", result);

    let result = validate_username("us");
    println!("验证短用户名结果: {:?}", result);
}

// 定义一个返回Result的函数，手动传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用?操作符简化错误传播
fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // 如果出错，直接返回错误
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 如果出错，直接返回错误
    Ok(s)
}

// 更简洁的版本，使用链式调用
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 还可以使用fs::read_to_string进一步简化
// fn read_username_from_file_very_short() -> Result<String, io::Error> {
//     std::fs::read_to_string("hello.txt")
// }

// main函数返回Result
// fn main_with_result() -> Result<(), Box<dyn std::error::Error>> {
//     let content = std::fs::read_to_string("hello.txt")?;
//     println!("文件内容: {}", content);
//     Ok(())
// }

// 8. 自定义错误类型
// 定义一个枚举来表示可能的错误
#[derive(Debug)]
enum ValidationError {
    TooShort,
    TooLong,
    InvalidCharacter,
}

// 实现Display trait，使错误可以转换为字符串
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::TooShort => write!(f, "用户名太短"),
            ValidationError::TooLong => write!(f, "用户名太长"),
            ValidationError::InvalidCharacter => write!(f, "用户名包含无效字符"),
        }
    }
}

// 实现Error trait
impl std::error::Error for ValidationError {
    fn description(&self) -> &str {
        match self {
            ValidationError::TooShort => "用户名长度不足",
            ValidationError::TooLong => "用户名长度超过限制",
            ValidationError::InvalidCharacter => "用户名含有非法字符",
        }
    }
}

// 使用自定义错误类型的函数
fn validate_username(username: &str) -> Result<String, ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::TooShort);
    }

    if username.len() > 20 {
        return Err(ValidationError::TooLong);
    }

    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::InvalidCharacter);
    }

    Ok(username.to_string())
}

// 9. 错误处理最佳实践
// - 对于可恢复的错误，使用Result
// - 对于不可恢复的错误，使用panic!
// - 当函数可能失败时，返回Result而不是panic
// - 使用?操作符简化错误传播
// - 在适当的层级处理错误，而不是在每个函数中都处理
// - 为自定义错误类型实现std::error::Error trait
