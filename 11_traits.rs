// 11_traits.rs - 深入介绍Rust中的Trait系统

// Trait是Rust中实现接口抽象的方式，它定义了某些特定类型可以实现的行为

// 导入必要的模块
use std::fmt;
use std::iter::Iterator;
use std::ops::Add;

// 1. 定义Trait
// Trait定义了一组方法，类型可以选择实现这些方法
pub trait Summary {
    // 方法声明
    fn summarize(&self) -> String;
    
    // 默认实现的方法
    fn summarize_author(&self) -> String {
        String::from("未知作者")
    }
    
    // 使用其他Trait方法的默认实现
    fn summarize_with_author(&self) -> String {
        format!("{} - {}", self.summarize(), self.summarize_author())
    }
}

// 2. 为类型实现Trait
// 为String类型实现Summary trait
impl Summary for String {
    fn summarize(&self) -> String {
        format!("文章内容: {}", self)
    }
}

// 定义一些结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为NewsArticle实现Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    // 覆盖默认实现
    fn summarize_author(&self) -> String {
        format!("作者: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为Tweet实现Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 3. Trait约束 - 将trait作为参数
// 使用trait约束指定参数必须实现特定的trait
fn notify<T: Summary>(item: &T) {
    println!("新闻通知: {}", item.summarize());
}

// 使用+语法指定多个trait约束
fn notify_with_display<T: Summary + fmt::Display>(item: &T) {
    println!("通知内容: {}", item);
    println!("摘要: {}", item.summarize());
}

// 使用where子句使trait约束更易读
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + fmt::Display,
    U: Clone + fmt::Debug,
{
    // 函数体
    42
}

// 4. 返回实现了trait的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 5. 运算符重载 - 通过实现std::ops中的trait
// 定义一个结构体用于演示运算符重载
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 实现Add trait来重载+运算符
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 定义一个表示毫秒的结构体
struct Millimeters(u32);
struct Meters(u32);

// 为不同类型实现Add trait
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 6. 派生trait
// 可以使用derive属性自动为类型实现某些trait
// 例如: Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default等
#[derive(Debug, PartialEq, Clone, Copy)]
struct Number {
    value: i32,
}

// 7. 自定义trait用于通用行为
// 定义一个可格式化的trait
pub trait Formattable {
    fn format(&self) -> String;
}

// 为常见类型实现Formattable trait
impl Formattable for i32 {
    fn format(&self) -> String {
        format!("整数: {}", self)
    }
}

impl Formattable for f64 {
    fn format(&self) -> String {
        format!("浮点数: {}", self)
    }
}

impl<T: Formattable> Formattable for Vec<T> {
    fn format(&self) -> String {
        let mut result = String::from("[");
        
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&item.format());
        }
        
        result.push_str("]");
        result
    }
}

// 8. 使用trait作为参数的通用函数
fn print_formatted<T: Formattable>(item: &T) {
    println!("格式化输出: {}", item.format());
}

// 9. 关联类型
// 在trait中定义关联类型，可以使trait更灵活
pub trait Iterator {
    type Item;  // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}

// 简单的计数器迭代器示例
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// 为Counter实现Iterator trait
impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        
        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

// 10. 特征对象（Trait Objects）
// 特征对象允许我们使用动态分发
// 定义一个具有draw方法的trait
pub trait Draw {
    fn draw(&self);
}

// 实现Draw trait的结构体
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: {} ({}x{})", self.label, self.width, self.height);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制选择框 ({}x{}) 包含 {} 个选项", 
                 self.width, self.height, self.options.len());
    }
}

// UI组件结构体，包含Draw特征对象的向量
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    // 测试Summary trait的实现
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("推文摘要: {}", tweet.summarize());
    println!("文章摘要: {}", article.summarize());
    println!("带作者的文章摘要: {}", article.summarize_with_author());
    
    // 测试trait约束函数
    notify(&tweet);
    notify(&article);
    
    // 测试返回实现了trait的类型
    let summarizable = returns_summarizable();
    println!("返回的可摘要对象: {}", summarizable.summarize());
    
    // 测试运算符重载
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
    
    let mm = Millimeters(1200);
    let m = Meters(5);
    let combined = mm + m;
    println!("合并后的长度: {}毫米", combined.0);
    
    // 测试派生trait
    let num1 = Number { value: 42 };
    let num2 = Number { value: 42 };
    let num3 = num1.clone();
    
    println!("num1 == num2: {}", num1 == num2);
    println!("num1 == num3: {}", num1 == num3);
    println!("num1: {:?}", num1);
    
    // 测试Formattable trait
    print_formatted(&42);
    print_formatted(&3.14);
    
    let v = vec![1, 2, 3, 4, 5];
    print_formatted(&v);
    
    // 测试迭代器trait
    let mut counter = Counter::new(5);
    println!("迭代计数器:");
    while let Some(count) = counter.next() {
        println!("计数: {}", count);
    }
    
    // 测试特征对象
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    
    println!("运行屏幕组件:");
    screen.run();
}