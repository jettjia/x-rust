// 10_generics.rs - 介绍Rust中的泛型（Generics）

// 泛型允许我们编写可以处理不同类型数据的代码，而不需要为每种类型单独编写函数或数据结构

fn main() {
    // 1. 泛型函数
    // 定义了一个可以比较任何实现了PartialOrd trait的类型的函数
    let result1 = largest(&[1, 2, 3, 4, 5]);
    println!("整数数组中的最大值: {}", result1);

    let result2 = largest(&["a", "b", "c"]);
    println!("字符串切片中的最大值: {}", result2);

    // 2. 泛型结构体
    // 定义了一个可以存储任何类型的点
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let string_point = Point {
        x: "Hello",
        y: "World",
    };

    println!("整数点: ({}, {})", integer_point.x, integer_point.y);
    println!("浮点点点: ({}, {})", float_point.x, float_point.y);
    println!("字符串点: ({}, {})", string_point.x, string_point.y);

    // 3. 带有多个泛型参数的结构体
    let pair = Pair { x: 5, y: "Hello" };
    println!("pair: x={}, y={}", pair.x, pair.y);

    // 调用泛型结构体的方法
    pair.cmp_display();

    // 4. 泛型枚举
    // Option<T>是Rust标准库中的泛型枚举
    let some_integer = Some(5);
    let some_string = Some("a string");
    let absent_value: Option<i32> = None;

    println!("some_integer: {:?}", some_integer);
    println!("some_string: {:?}", some_string);
    println!("absent_value: {:?}", absent_value);

    // Result<T, E>也是Rust标准库中的泛型枚举
    let success_result: Result<i32, String> = Ok(42);
    let error_result: Result<i32, String> = Err(String::from("发生错误"));

    println!("success_result: {:?}", success_result);
    println!("error_result: {:?}", error_result);

    // 5. 泛型方法
    let point1 = Point { x: 5, y: 10 };
    let point2 = point1.x(); // 调用泛型方法x()
    println!("point1的x坐标: {}", point2);

    let point3 = Point { x: 5, y: 10 };
    let point4 = Point { x: 1, y: 2 };
    // 添加显式类型注解，指定返回值类型为Point<i32>
    let combined_point: Point<i32> = point3.mixup(point4);
    println!("混合后的点: ({}, {})", combined_point.x, combined_point.y);

    // 6. 带有trait约束的泛型
    // 定义一个可以计算面积的函数，要求参数实现Shape trait
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    println!("圆的面积: {}", area(&circle));
    println!("矩形的面积: {}", area(&rectangle));

    // 7. 多个trait约束
    // 定义一个函数，要求参数实现多个trait
    let circle2 = Circle { radius: 5.0 };
    print_info(&circle2);

    let rectangle2 = Rectangle {
        width: 10,
        height: 20,
    };
    print_info(&rectangle2);

    // 8. where子句 - 使多个trait约束更易读
    let value1 = 42;
    let value2 = 100;

    compare_and_print(value1, value2);

    // 9. 泛型在性能上的考量
    // Rust的泛型在编译时会进行单态化（monomorphization），为每种具体类型生成对应的代码
    // 这意味着泛型代码在运行时不会有性能损失
}

// 1. 泛型函数定义
// T是泛型参数，它可以是任何类型
// PartialOrd是一个trait约束，要求T实现了PartialOrd trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 2. 泛型结构体定义
struct Point<T> {
    x: T,
    y: T,
}

// 3. 带有多个泛型参数的结构体
struct Pair<T, U> {
    x: T,
    y: U,
}

// 为泛型结构体实现方法
impl<T: std::fmt::Debug, U: std::fmt::Debug> Pair<T, U> {
    fn cmp_display(&self) {
        println!("对的值: x={:?}, y={:?}", self.x, self.y);
    }
}

// 4. 为特定类型实现方法
impl Pair<i32, f32> {
    fn to_sum(&self) -> f32 {
        self.x as f32 + self.y
    }
}

// 5. 泛型方法定义
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// 带有多个泛型参数的方法
impl<T> Point<T> {
    fn mixup<V, U>(self, other: Point<V>) -> Point<U>
    where
        T: Into<U>,
        V: Into<U>,
    {
        Point {
            x: self.x.into(),
            y: other.x.into(),
        }
    }
}

// 6. 定义一个trait
// trait定义了一组方法，类型可以实现这些方法
pub trait Shape {
    fn area(&self) -> f64;
}

// 实现trait
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
}

// 定义一些结构体用于实现trait
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// 带有trait约束的函数
fn area<T: Shape>(shape: &T) -> f64 {
    shape.area()
}

// 7. 多个trait约束
// Display trait允许类型使用println!宏输出
use std::fmt::Display;

fn print_info<T: Shape + Display>(shape: &T) {
    println!("形状信息: {}", shape);
    println!("面积: {}", shape.area());
}

// 为Circle和Rectangle实现Display trait
impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "圆(半径: {})", self.radius)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "矩形(宽: {}, 高: {})", self.width, self.height)
    }
}

// 8. where子句
// 添加额外约束使T类型可以和U类型进行比较
fn compare_and_print<T, U>(a: T, b: U)
where
    T: Display + PartialOrd<U>,
    U: Display,
{
    if a < b {
        println!("{} < {}", a, b);
    } else if a > b {
        println!("{} > {}", a, b);
    } else {
        println!("{} = {}", a, b);
    }
}

// 9. 泛型的单态化示例
// 下面是一个简单的泛型函数
fn identity<T>(x: T) -> T {
    x
}

// 在编译时，Rust会为每种使用的具体类型生成对应的代码
// 例如，当调用identity(5)和identity("hello")时，
// 编译器会生成两个函数：一个接受i32参数，一个接受&str参数
