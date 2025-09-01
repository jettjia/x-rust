// 04_ownership.rs - 介绍Rust中的所有权概念

fn main() {
    // 1. 所有权的基本规则
    // a. Rust中的每个值都有一个所有者
    // b. 一次只能有一个所有者
    // c. 当所有者离开作用域时，值会被丢弃
    
    // 字符串字面量示例 - 不可变，存储在栈上
    let s1 = "hello";  // s1是字符串字面量的所有者
    println!("s1: {}", s1);
    
    // String类型示例 - 可变，存储在堆上
    let mut s2 = String::from("hello");  // s2是String值的所有者
    s2.push_str(", world!");  // 修改String值
    println!("s2: {}", s2);
    
    // 2. 移动（Move）语义
    // 将s2的所有权转移给s3
    let s3 = s2;  // s2不再有效，所有权被移动到s3
    println!("s3: {}", s3);
    // 下面这行会导致编译错误，因为s2不再拥有该值
    // println!("s2 after move: {}", s2);
    
    // 3. 克隆（Clone）- 深拷贝堆上的数据
    let s4 = String::from("hello");
    let s5 = s4.clone();  // s4和s5各自拥有不同的String值
    println!("s4: {}, s5: {}", s4, s5);
    
    // 4. 栈上数据的复制（Copy）- 对于基本类型
    let x = 5;  // x是i32值的所有者
    let y = x;  // i32实现了Copy trait，所以y是x的复制
    println!("x: {}, y: {}", x, y);  // 两个值都有效
    
    // 5. 引用与借用
    // 不可变引用
    let s6 = String::from("hello");
    let r1 = &s6;  // r1是s6的不可变引用
    let r2 = &s6;  // 可以有多个不可变引用
    println!("通过引用访问: {}, {}", r1, r2);
    
    // 可变引用
    let mut s7 = String::from("hello");
    let r3 = &mut s7;  // r3是s7的可变引用
    r3.push_str(", world!");  // 通过可变引用修改值
    println!("通过可变引用修改后: {}", r3);
    // 注意：同一时间只能有一个可变引用
    
    // 6. 引用的作用域
    {
        let r4 = &mut s7;  // r4的作用域开始
        r4.push_str(", Rust!");
        println!("r4: {}", r4);
    }  // r4的作用域结束，可变引用失效
    
    // 现在可以再次创建s7的可变引用
    let r5 = &mut s7;
    println!("r5: {}", r5);
    
    // 7. 切片（Slices）
    let s8 = String::from("hello world");
    let hello = &s8[0..5];  // 创建字符串切片
    let world = &s8[6..11];
    println!("切片1: {}, 切片2: {}", hello, world);
    
    // 简化的切片语法
    let slice1 = &s8[..5];  // 从开始到索引5（不包含）
    let slice2 = &s8[6..];  // 从索引6到结束
    let slice3 = &s8[..];   // 整个字符串
    println!("简化切片: {}, {}, {}", slice1, slice2, slice3);
    
    // 传递引用给函数
    let s9 = String::from("hello");
    let len = calculate_length(&s9);  // 传递引用，不获取所有权
    println!("字符串'{}'的长度是: {}", s9, len);
}

// 接受字符串引用作为参数的函数
fn calculate_length(s: &String) -> usize {
    // s是对String的引用，不拥有所有权
    s.len()  // 返回字符串长度
    // 当函数结束时，s离开作用域，但由于它只是引用，不会丢弃原始值
}