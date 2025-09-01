// 05_structs.rs - 介绍Rust中的结构体定义和使用

// 1. 定义结构体
// 结构体是一种自定义数据类型，允许我们将多个相关的值组合在一起
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 2. 定义元组结构体 - 没有命名字段的结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 3. 定义无字段结构体 - 也称为单元结构体
struct AlwaysEqual;

// 4. 结构体的方法定义
struct Rectangle {
    width: u32,
    height: u32,
}

// 为Rectangle结构体实现方法
impl Rectangle {
    // 方法 - 第一个参数是self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 带参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 关联函数 - 不使用self作为第一个参数
    // 通常用作构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 为结构体实现多个impl块
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    // 5. 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 6. 访问结构体字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    
    // 7. 修改结构体实例 - 需要将实例声明为mut
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("updated_email@example.com");
    user2.sign_in_count += 1;
    
    println!("更新后的邮箱: {}", user2.email);
    println!("更新后的登录次数: {}", user2.sign_in_count);
    
    // 8. 结构体更新语法 - 从其他实例复制字段
    let user3 = User {
        email: String::from("third@example.com"),
        username: String::from("thirdusername"),
        // ..user2 表示其余字段从user2复制
        ..user2
    };
    
    println!("user3的活跃状态: {}", user3.active);
    println!("user3的登录次数: {}", user3.sign_in_count);
    
    // 注意：user2的email和username字段已经被移动到user1中
    // 所以下面这行代码会导致编译错误
    // println!("user2的用户名: {}", user2.username);
    
    // 9. 使用元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // 访问元组结构体的字段
    println!("黑色的RGB值: {}, {}, {}", black.0, black.1, black.2);
    println!("原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 10. 使用无字段结构体
    let subject = AlwaysEqual;
    // 无字段结构体没有字段可以访问
    
    // 11. 使用结构体方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形的面积: {}", rect1.area());
    println!("矩形的周长: {}", rect1.perimeter());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("rect1能容纳rect2吗? {}", rect1.can_hold(&rect2));
    println!("rect1能容纳rect3吗? {}", rect1.can_hold(&rect3));
    
    // 12. 调用关联函数
    let square = Rectangle::square(30);
    println!("正方形的面积: {}", square.area());
    println!("正方形的周长: {}", square.perimeter());
}