// 15_unsafe.rs - 介绍Rust中的unsafe代码

// Rust提供了unsafe关键字，用于绕过Rust的一些安全检查
// 这允许我们执行一些Rust通常不允许的操作，但需要程序员自己确保代码的安全性

fn main() {
    // 1. unsafe块的基本用法
    let mut num = 5;
    
    // 创建一个指向num的原始指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    // 访问原始指针需要在unsafe块中进行
    unsafe {
        println!("r1指向的值: {}", *r1);
        println!("r2指向的值: {}", *r2);
        *r2 = 10;  // 修改值也需要unsafe块
    }
    
    println!("修改后的num: {}", num);
    
    // 2. 调用unsafe函数
    let v = vec![1, 2, 3, 4, 5];
    let ele = unsafe {
        get_unsafe_element(&v, 2)
    };
    println!("向量的第三个元素: {}", ele);
    
    // 3. 访问裸指针
    let mut values = [1, 2, 3, 4, 5];
    let ptr = values.as_mut_ptr();
    
    unsafe {
        // 使用指针算术
        for i in 0..values.len() {
            println!("values[{}] = {}", i, *ptr.add(i));
            // 修改值
            *ptr.add(i) += 10;
        }
    }
    
    println!("修改后的values: {:?}", values);
    
    // 4. 使用extern函数
    // 调用C语言的函数
    unsafe {
        let result = abs(-5);
        println!("绝对值: {}", result);
    }
    
    // 5. 实现unsafe trait
    let value = UnsafeValue { value: 42 };
    println!("UnsafeValue: {}", value.get_value());
    
    // 6. 使用union
    let u = MyUnion {
        i: 42
    };
    
    unsafe {
        println!("u.i = {}", u.i);
        // 访问未初始化的字段
        println!("u.f = {}", u.f);  // 这是未定义行为，仅作演示
    }
    
    // 7. 内存布局控制
    let point = Point { x: 10, y: 20 };
    let ptr = &point as *const Point as *const u8;
    
    unsafe {
        // 直接读取内存
        println!("x的第一个字节: {:#x}", *ptr);
        // 这仅用于演示，实际应用中应该使用std::mem::transmute或其他安全方法
    }
    
    // 8. 可变静态变量
    increment_counter();
    increment_counter();
    println!("计数器值: {}", unsafe { COUNTER });
    
    // 9. unsafe代码的最佳实践
    println!("unsafe代码最佳实践:");
    println!("1. 尽量减少unsafe代码的使用");
    println!("2. 将unsafe代码封装在安全的API后面");
    println!("3. 为unsafe代码编写全面的文档和测试");
    println!("4. 确保遵循所有安全要求");
}

// 1. 定义unsafe函数
unsafe fn get_unsafe_element<T>(vec: &Vec<T>, index: usize) -> &T {
    // 注意：这个函数实际上是不安全的，因为没有检查索引是否越界
    // 实际应用中应该添加适当的检查
    &vec[index]
}

// 2. 声明extern函数
// 这声明了一个来自C标准库的函数
extern "C" {
    fn abs(input: i32) -> i32;
}

// 3. 定义unsafe trait
unsafe trait UnsafeTrait {
    fn do_something_unsafe(&self);
}

// 实现unsafe trait
unsafe impl UnsafeTrait for i32 {
    fn do_something_unsafe(&self) {
        // 实现不安全的操作
        println!("对{}执行不安全操作", self);
    }
}

// 4. 使用unsafe trait的结构体
struct UnsafeValue<T> {
    value: T
}

impl<T> UnsafeValue<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

// 5. 定义union
union MyUnion {
    i: i32,
    f: f32,
}

// 6. 定义结构体用于内存布局演示
#[repr(C)]  // 使用C语言的内存布局
struct Point {
    x: i32,
    y: i32,
}

// 7. 定义可变静态变量
static mut COUNTER: u32 = 0;

// 封装对静态变量的访问
fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

// 8. 安全的包装函数示例
fn safe_abs(value: i32) -> i32 {
    unsafe {
        abs(value)
    }
}

// 9. 裸指针的类型
// *const T: 不可变裸指针，类似于C语言的const T*
// *mut T: 可变裸指针，类似于C语言的T*

// 10. unsafe代码的常见用例
// - FFI (Foreign Function Interface)：与其他语言交互
// - 性能优化：在某些情况下绕过Rust的安全检查以提高性能
// - 低级系统编程：访问硬件、内存管理等
// - 实现安全抽象：为其他Rust代码提供安全的API

// 11. unsafe代码的安全要求
// 当你使用unsafe代码时，你需要确保：
// - 不读取未初始化的内存
// - 不违反借用规则（同一时间一个可变引用或多个不可变引用）
// - 不造成悬垂引用
// - 不违反类型安全（例如，不将一个类型的值强制转换为另一个不兼容的类型）
// - 正确使用extern函数

// 12. 内存安全和unsafe代码
// 虽然unsafe代码允许我们绕过Rust的一些安全检查，但Rust的内存安全保证仍然适用于整个程序
// unsafe代码块中的代码仍然需要遵循Rust的其他规则，如类型检查、借用检查等
// unsafe只是允许我们执行一些通常不允许的操作，但我们仍然需要确保这些操作是安全的

// 13. 示例：实现Vec的简化版本
// 这是一个简化的Vec实现，仅用于演示unsafe代码的使用
struct SimpleVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SimpleVec<T> {
    fn new() -> Self {
        SimpleVec {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    // 添加元素的方法，包含unsafe代码
    fn push(&mut self, value: T) {
        // 这里应该有扩容逻辑
        // 为了简化，这里省略了
        unsafe {
            // 这行代码仅用于演示，实际上是不安全的
            // std::ptr::write(self.ptr.add(self.len), value);
            // self.len += 1;
        }
    }
}

// 实现Drop trait来释放内存
impl<T> Drop for SimpleVec<T> {
    fn drop(&mut self) {
        // 这里应该释放内存
        // 为了简化，这里省略了
    }
}