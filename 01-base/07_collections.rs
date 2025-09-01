// 07_collections.rs - 介绍Rust中的常见集合类型及操作

fn main() {
    // 1. Vector - 动态数组，可以存储多个相同类型的值
    // 创建Vector
    let v1: Vec<i32> = Vec::new();

    // 使用vec!宏创建Vector并初始化
    let v2 = vec![1, 2, 3, 4, 5];

    // 创建可变的Vector
    let mut v3 = Vec::new();

    // 添加元素
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    // 读取Vector中的元素
    // 方法1: 使用索引
    let third = &v2[2];
    println!("v2的第三个元素: {}", third);

    // 方法2: 使用get方法，返回Option<&T>
    match v2.get(2) {
        Some(third) => println!("v2的第三个元素: {}", third),
        None => println!("索引超出范围"),
    }

    // 遍历Vector
    println!("遍历v3中的元素:");
    for i in &v3 {
        println!("{}", i);
    }

    // 遍历并修改Vector中的元素
    println!("遍历并修改v3中的元素:");
    for i in &mut v3 {
        *i += 10; // 使用解引用操作符修改值
        println!("{}", i);
    }

    // 2. String - UTF-8编码的字符串类型
    // 创建空String
    let s1 = String::new();

    // 从字符串字面量创建String
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    // 修改String
    let mut s4 = String::from("foo");
    s4.push_str("bar"); // 追加字符串切片
    s4.push('!'); // 追加单个字符

    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    // 字符串连接
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // s5被移动，不能再使用
    println!("s7: {}", s7);

    // 使用format!宏连接字符串
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10); // s8, s9, s10仍然可用
    println!("s11: {}", s11);

    // 字符串索引 - Rust字符串不支持直接索引
    let s12 = String::from("hello");
    // 下面这行代码会导致编译错误
    // let h = s12[0];

    // 遍历字符串
    println!("遍历字符串's12'的字符:");
    for c in s12.chars() {
        println!("{}", c);
    }

    // 3. HashMap - 键值对集合
    use std::collections::HashMap;

    // 创建HashMap
    let mut scores = HashMap::new();

    // 添加键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    // 从Vector创建HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores2: {:?}", scores2);

    // 访问HashMap中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("Blue队的分数: {}", s),
        None => println!("未找到Blue队的分数"),
    }

    // 遍历HashMap
    println!("遍历scores中的键值对:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新HashMap
    // 覆盖已有的值
    scores.insert(String::from("Blue"), 25);
    println!("更新后scores: {:?}", scores);

    // 只有在键不存在时才插入值
    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(50); // 不会覆盖已有的值
    println!("使用entry后scores: {:?}", scores);

    // 基于现有值更新值
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("单词计数: {:?}", word_count);
}
