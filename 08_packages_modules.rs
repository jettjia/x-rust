// 08_packages_modules.rs - 介绍Rust中的包、模块和路径

// 在Rust中，包(Package)是一个或多个Crates的集合
// Crate是一个编译单元，可以生成可执行文件或库
// 模块(Module)用于组织代码，控制作用域和私有性

// 以下是模块定义的示例
// 注意：在实际的Rust项目中，这些模块可能会被拆分成多个文件

// 定义一个模块
mod front_of_house {
    // 模块可以嵌套
    pub mod hosting {
        // 在Rust中，默认情况下，函数、类型、模块和常量是私有的
        // 使用pub关键字使其变为公有的
        pub fn add_to_waitlist() {
            // 可以访问同一模块内的私有函数
            seat_at_table();
        }

        fn seat_at_table() {
            println!("安排座位");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("记录订单");
        }

        pub fn serve_order() {
            println!("提供食物");
        }

        pub fn take_payment() {
            println!("收取付款");
        }
    }
}

// 使用pub mod定义一个公共模块
pub mod back_of_house {
    // 结构体的字段默认是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 私有字段
    }

    // 为公共结构体实现方法
    impl Breakfast {
        // 构造函数通常是关联函数，命名为new
        pub fn new(toast: String) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("蓝莓"),
            }
        }

        // 公共方法可以访问私有字段
        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    // 枚举的所有变体默认都是公有的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // 公共枚举的方法
    impl Appetizer {
        pub fn order(self) {
            match self {
                Appetizer::Soup => println!("一份汤被点了"),
                Appetizer::Salad => println!("一份沙拉被点了"),
            }
        }
    }
}

// 使用use关键字将路径引入作用域
use crate::front_of_house::hosting;
// 可以重命名引入的项以避免名称冲突
use crate::back_of_house::Breakfast as MorningMeal;
// 通配符*可以引入所有公共项
// use crate::back_of_house::*;

fn main() {
    // 通过完整路径调用函数
    crate::front_of_house::hosting::add_to_waitlist();

    // 通过引入的路径调用函数
    hosting::add_to_waitlist();

    // 创建结构体实例
    let mut meal = MorningMeal::new(String::from("全麦吐司"));
    println!("早餐吐司类型: {}", meal.toast);
    println!("早餐水果: {}", meal.fruit());

    // 尝试修改私有字段会导致编译错误
    // meal.seasonal_fruit = String::from("草莓");

    // 可以修改公共字段
    meal.toast = String::from("白吐司");
    println!("修改后的吐司类型: {}", meal.toast);

    // 使用枚举
    let order1 = back_of_house::Appetizer::Soup;
    order1.order();

    let order2 = back_of_house::Appetizer::Salad;
    order2.order();

    // 调用模块中的其他函数
    serve_customer();
}

// 在函数中使用模块
fn serve_customer() {
    // 注意：此函数在根模块中定义，因此可以直接访问其他根模块中的项
    hosting::add_to_waitlist();

    let meal = MorningMeal::new(String::from("黑麦吐司"));
    println!("为顾客提供: {}", meal.toast);

    // 使用crate关键字从根模块开始访问
    crate::front_of_house::serving::take_order();
    crate::front_of_house::serving::serve_order();
    crate::front_of_house::serving::take_payment();
}

// 模块文件结构示例
// 在实际项目中，你可能会有以下文件结构：
// src/
//  ├── main.rs           // 主入口文件
//  ├── lib.rs            // 库入口文件（如果是库项目）
//  ├── front_of_house/   // 对应front_of_house模块的目录
//  │   ├── mod.rs        // front_of_house模块的定义
//  │   ├── hosting.rs    // hosting子模块
//  │   └── serving.rs    // serving子模块
//  └── back_of_house.rs  // back_of_house模块的定义

// 注意：要运行此代码，你需要按照上述文件结构组织文件
// 或者将所有模块定义放在同一个文件中
