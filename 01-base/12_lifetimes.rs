// 12_lifetimes.rs - 介绍Rust中的生命周期（Lifetimes）

// 生命周期是Rust中用于确保引用有效性的机制，防止悬垂引用

fn main() {
    // 1. 生命周期的基本概念
    // 生命周期确保引用在其指向的值被销毁前不会失效
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("较长的字符串是: {}", result);
    
    // 2. 生命周期标注语法
    // 生命周期参数以'开头，通常使用单个字母表示
    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("较长的字符串是: {}", result);
    } // string4在这里被销毁
    // result在这里仍然有效，因为它引用的是string3
    
    // 3. 结构体中的生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {}", i.part);
    
    // 4. 生命周期省略规则
    // Rust有一套生命周期省略规则，允许在某些情况下省略显式的生命周期标注
    let s1 = String::from("hello");
    let len = calculate_length(s1.as_str());
    println!("字符串长度: {}", len);
    
    // 5. 方法中的生命周期
    let book = ImportantExcerpt {
        part: "重要的部分",
    };
    println!("引用部分: {}", book.announce_and_return_part("新书"));
    
    // 6. 静态生命周期
    // 'static生命周期表示值在整个程序运行期间都有效
    let s: &'static str = "我有一个静态生命周期";
    println!("静态字符串: {}", s);
    
    // 7. 多个生命周期参数
    let string5 = String::from("first");
    let string6 = String::from("second");
    let string7 = String::from("third");
    
    let result = longest_with_an_announcement(
        string5.as_str(),
        string6.as_str(),
        string7.as_str(),
    );
    println!("最长的字符串: {}", result);
    
    // 8. 泛型参数、trait约束和生命周期结合使用
    let a = 5;
    let b = 10;
    let c = 15;
    
    let result = compare_and_choose(&a, &b, &c);
    println!("选择的结果: {}", result);
}

// 1. 带生命周期标注的函数
// 'a是生命周期参数，表示x和y的生命周期至少与'a一样长
// 返回值的生命周期也为'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 2. 结构体中的生命周期标注
// 这个结构体包含一个字符串切片的引用
// 生命周期参数'a表示part字段的引用必须与结构体实例存活时间一样长
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 3. 为结构体实现方法时的生命周期标注
impl<'a> ImportantExcerpt<'a> {
    // 第一个参数是self，其生命周期与结构体的生命周期相同
    fn level(&self) -> i32 {
        3
    }
    
    // 这里应用了生命周期省略规则
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("宣布: {}", announcement);
        self.part
    }
}

// 4. 生命周期省略的例子
// 这个函数的完整形式应该是：fn calculate_length<'a>(s: &'a str) -> usize
// 但由于生命周期省略规则，可以省略生命周期标注
fn calculate_length(s: &str) -> usize {
    s.len()
}

// 5. 多个生命周期参数
// 这个函数有三个生命周期参数，但只有'a与返回值相关
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("宣布: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 6. 泛型参数、trait约束和生命周期结合使用
fn compare_and_choose<'a, T>(a: &'a T, b: &'a T, c: &'a T) -> &'a T
where
    T: std::cmp::PartialOrd + std::fmt::Display,
{
    if a > b && a > c {
        a
    } else if b > a && b > c {
        b
    } else {
        c
    }
}

// 7. 生命周期省略规则详解
// Rust编译器有三条生命周期省略规则：
// 
// 规则1：每个引用类型的参数都有自己的生命周期参数
// 规则2：如果只有一个输入生命周期参数，那么它被赋予给所有输出生命周期参数
// 规则3：如果方法有多个输入生命周期参数，而其中一个是 &self 或 &mut self，
//       那么 self 的生命周期被赋予给所有输出生命周期参数
// 
// 这些规则使得我们在大多数常见情况下不需要显式标注生命周期

// 8. 生命周期与引用类型
// 不同类型的引用有不同的生命周期行为
// 
// - &'a T: 不可变引用，生命周期为'a
// - &'a mut T: 可变引用，生命周期为'a
// 
// 注意：同一时间，对于同一数据，只能有一个可变引用或多个不可变引用

// 9. 静态生命周期
// 'static生命周期是一个特殊的生命周期，表示值在整个程序运行期间都有效
// 
// 字符串字面量默认具有'static生命周期
// let s: &'static str = "这是一个字符串字面量";
// 
// 这意味着字符串字面量存储在程序的只读内存中，而不是在栈或堆上

// 10. 生命周期与所有权
// 生命周期与所有权密切相关，但它们关注的是不同的方面：
// - 所有权关注的是值的归属和释放
// - 生命周期关注的是引用的有效性
// 
// 两者结合确保了Rust的内存安全