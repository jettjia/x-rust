# Rust 学习资源

这个仓库包含了一系列Rust语言的示例代码，按照序号排列，覆盖了Rust的各种核心概念和语法特性。这些文件旨在帮助初学者逐步学习和掌握Rust编程语言。

## 项目结构

每个文件都按照主题进行了分类，并使用数字序号命名，以便按照建议的学习顺序进行阅读和实践。

| 文件名                         | 主题               | 内容简介                                                   |
| ------------------------------ | ------------------ | ---------------------------------------------------------- |
| `01_variables.rs`              | 变量和基本数据类型 | 介绍Rust中的变量声明、可变性、常量、隐藏以及基本数据类型   |
| `02_functions.rs`              | 函数               | 介绍Rust中的函数定义、参数、返回值、递归等概念             |
| `03_flow_control.rs`           | 流程控制           | 介绍if表达式、循环（loop、while、for）和match表达式        |
| `04_ownership.rs`              | 所有权             | 介绍Rust的核心概念：所有权、借用、引用和切片               |
| `05_structs.rs`                | 结构体             | 介绍结构体的定义、实例化、字段访问、方法实现等             |
| `06_enums_pattern_matching.rs` | 枚举和模式匹配     | 介绍枚举的定义、关联数据、方法以及模式匹配的使用           |
| `07_collections.rs`            | 常见集合           | 介绍Vector、String和HashMap等集合类型及操作                |
| `08_packages_modules.rs`       | 包和模块           | 介绍Rust的模块系统、路径、可见性等概念                     |
| `09_error_handling.rs`         | 错误处理           | 介绍Rust的错误处理机制，包括panic!、Result和自定义错误类型 |
| `10_generics.rs`               | 泛型               | 介绍泛型函数、泛型结构体、泛型方法以及trait约束            |
| `11_traits.rs`                 | Trait              | 深入介绍Trait的定义、实现、trait约束、关联类型和特征对象   |
| `12_lifetimes.rs`              | 生命周期           | 介绍生命周期的概念、标注语法、省略规则和实际应用           |
| `13_async.rs`                  | 异步编程           | 介绍异步编程概念、async/await语法、Future trait和异步运行时 |
| `14_macros.rs`                 | 宏                 | 介绍声明宏、过程宏和标准库中常用宏的使用                   |
| `15_unsafe.rs`                 | unsafe代码         | 介绍unsafe块、裸指针、extern函数和内存安全相关概念         |
| `16_multithreading.rs`         | 多线程和并发       | 介绍线程创建、共享状态、通道、锁和原子操作等并发编程技术   |

## 如何运行示例代码

要运行这些示例代码，您需要在系统上安装Rust。如果您还没有安装Rust，可以按照以下步骤进行安装：

1. 访问 [Rust官方网站](https://www.rust-lang.org/)，按照指示下载并安装Rust
2. 安装完成后，可以使用`rustc --version`命令验证安装是否成功

运行单个示例文件的步骤：

1. 打开终端，导航到项目目录
2. 使用`rustc`命令编译特定的Rust文件，例如：
   ```bash
   rustc 01_variables.rs
   ```
3. 编译成功后，会生成一个可执行文件，使用以下命令运行：
   ```bash
   ./01_variables
   ```

或者，您可以使用Cargo（Rust的包管理器和构建工具）来管理和运行这些示例：

1. 在项目目录中初始化一个新的Cargo项目：
   ```bash
   cargo init
   ```
2. 将示例文件放入`src`目录
3. 使用`cargo run`命令运行特定的示例，例如：
   ```bash
   cargo run --bin 01_variables
   ```

## 资源推荐

- [Rust程序设计语言（官方书籍）](https://doc.rust-lang.org/stable/book/)
- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Documentation](https://docs.rs/)
