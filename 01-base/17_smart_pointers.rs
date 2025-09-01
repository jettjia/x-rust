// 17_smart_pointers.rs - 介绍Rust中的智能指针

// 智能指针是Rust中具有指针特性但还拥有额外元数据和功能的数据结构
// 智能指针通常实现了Deref和Drop trait

use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::thread;

// 1. 自定义智能指针示例
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现Deref trait，使MyBox可以像引用一样使用
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 实现Drop trait，自定义析构行为
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox被销毁");
    }
}

// 2. 自定义引用计数智能指针（简化版）
struct MyRc<T> {
    data: T,
    count: Cell<usize>,
}

impl<T> MyRc<T> {
    fn new(data: T) -> Rc<MyRc<T>> {
        Rc::new(MyRc {
            data,
            count: Cell::new(1),
        })
    }
    
    fn clone(rc: &Rc<MyRc<T>>) -> Rc<MyRc<T>> {
        let current_count = rc.count.get();
        rc.count.set(current_count + 1);
        Rc::clone(rc)
    }
    
    fn strong_count(rc: &Rc<MyRc<T>>) -> usize {
        rc.count.get()
    }
}

fn main() {
    // 3. Box<T> - 在堆上分配内存
    println!("=== Box<T> 智能指针 ===");
    let b = Box::new(5);  // 在堆上分配内存存储5
    println!("b = {}", b);  // 自动解引用
    
    // 使用自定义智能指针
    let m = MyBox::new(10);
    println!("m = {}", *m);  // 通过Deref trait支持解引用操作
    
    // 4. 使用Box<T>实现递归数据结构
    println!("\n=== 使用Box<T>实现递归数据结构 ===");
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("递归列表的第一个元素: {}", list.head());
    
    // 5. Rc<T> - 引用计数智能指针（单线程）
    println!("\n=== Rc<T> 引用计数智能指针 ===");
    let rc1 = Rc::new(String::from("引用计数的字符串"));
    println!("初始引用计数: {}", Rc::strong_count(&rc1));
    
    // 创建一个新的引用
    let rc2 = Rc::clone(&rc1);
    println!("创建rc2后的引用计数: {}", Rc::strong_count(&rc1));
    
    // 在作用域内创建另一个引用
    {
        let rc3 = Rc::clone(&rc1);
        println!("创建rc3后的引用计数: {}", Rc::strong_count(&rc1));
        println!("rc3的值: {}", rc3);
    }  // rc3离开作用域，引用计数减1
    
    println!("rc3离开作用域后的引用计数: {}", Rc::strong_count(&rc1));
    println!("rc2的值: {}", rc2);
    
    // 6. 使用Rc<T>实现多重所有权
    println!("\n=== 使用Rc<T>实现多重所有权 ===");
    let node1 = Rc::new(Node { value: 1, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(Rc::clone(&node1)) });
    let node3 = Rc::new(Node { value: 3, next: Some(Rc::clone(&node1)) });
    
    // 检查node1的引用计数
    println!("node1的引用计数: {}", Rc::strong_count(&node1));
    
    // 7. RefCell<T> - 运行时可变借用检查
    println!("\n=== RefCell<T> 运行时可变借用 ===");
    let data = RefCell::new(5);
    println!("初始值: {}", *data.borrow());
    
    // 可变借用并修改值
    *data.borrow_mut() += 10;
    println!("修改后的值: {}", *data.borrow());
    
    // 演示RefCell<T>的运行时借用规则检查
    {
        let mut mutable_borrow = data.borrow_mut();
        *mutable_borrow += 5;
        println!("再次修改后的值: {}", *mutable_borrow);
        
        // 注意：以下代码会在运行时导致panic，因为同一时间只能有一个可变借用
        // let immutable_borrow = data.borrow();
        // println!("尝试同时借用: {}", *immutable_borrow);
    }  // mutable_borrow离开作用域，借用被释放
    
    // 现在可以再次借用
    println!("最终值: {}", *data.borrow());
    
    // 8. 组合Rc<T>和RefCell<T>实现共享可变状态
    println!("\n=== 组合Rc<T>和RefCell<T> ===");
    let shared_data = Rc::new(RefCell::new(42));
    
    let reference1 = Rc::clone(&shared_data);
    let reference2 = Rc::clone(&shared_data);
    
    // 通过第一个引用修改值
    *reference1.borrow_mut() = 100;
    println!("通过reference2读取修改后的值: {}", *reference2.borrow());
    
    // 9. 使用Rc<RefCell<T>>实现双向链表
    println!("\n=== 使用Rc<RefCell<T>>实现双向链表 ===");
    let head = Rc::new(DoublyLinkedNode {
        value: 1,
        prev: None,
        next: None
    });
    
    let tail = Rc::new(DoublyLinkedNode {
        value: 2,
        prev: Some(Rc::clone(&head)),
        next: None
    });
    
    // 需要使用unsafe块或辅助函数来设置head的next指针
    
    // 10. Arc<T> - 线程安全的引用计数
    println!("\n=== Arc<T> 线程安全的引用计数 ===");
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("线程安全计数器的最终值: {}", *shared_counter.lock().unwrap());
    
    // 11. 循环引用问题
    println!("\n=== 循环引用问题 ===");
    // 演示如何创建循环引用
    let a = Rc::new(CyclicNode {
        value: String::from("Node A"),
        next: RefCell::new(None)
    });
    
    let b = Rc::new(CyclicNode {
        value: String::from("Node B"),
        next: RefCell::new(None)
    });
    
    // 创建循环引用
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.next.borrow_mut() = Some(Rc::clone(&a));
    
    println!("a的引用计数: {}", Rc::strong_count(&a));
    println!("b的引用计数: {}", Rc::strong_count(&b));
    
    // 注意：由于循环引用，当a和b离开作用域时，它们指向的内存不会被释放
    // 这是内存泄漏的一种形式
    
    // 12. 避免循环引用的方法
    println!("\n=== 避免循环引用的方法 ===");
    // 方法1: 使用Weak<T>打破循环
    let parent = Rc::new(Parent {
        name: String::from("Parent"),
        children: RefCell::new(vec![])
    });
    
    let child = Rc::new(Child {
        name: String::from("Child"),
        parent: Rc::downgrade(&parent)  // 创建弱引用
    });
    
    parent.children.borrow_mut().push(Rc::clone(&child));
    
    // 使用upgrade()方法访问弱引用指向的值
    if let Some(parent_ref) = child.parent.upgrade() {
        println!("子节点的父节点名称: {}", parent_ref.name);
    }
    
    println!("parent的强引用计数: {}", Rc::strong_count(&parent));
    println!("parent的弱引用计数: {}", Rc::weak_count(&parent));
    
    // 13. 智能指针的选择指南
    println!("\n=== 智能指针的选择指南 ===");
    println!("- Box<T>: 当需要在堆上分配内存且所有权明确时");
    println!("- Rc<T>: 当需要在单线程环境中共享所有权时");
    println!("- Arc<T>: 当需要在多线程环境中共享所有权时");
    println!("- RefCell<T>: 当需要在运行时检查借用规则时");
    println!("- Rc<RefCell<T>>: 当需要在单线程环境中共享可变状态时");
    println!("- Arc<Mutex<T>>: 当需要在多线程环境中共享可变状态时");
    println!("- Weak<T>: 当需要打破循环引用时");
}

// 用于演示Box<T>实现递归数据结构的枚举
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn head(&self) -> i32 {
        match self {
            List::Cons(value, _) => *value,
            List::Nil => panic!("Cannot get head of an empty list"),
        }
    }
}

// 用于演示Rc<T>的简单节点结构体
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

// 用于演示双向链表的节点结构体
struct DoublyLinkedNode {
    value: i32,
    prev: Option<Rc<DoublyLinkedNode>>,
    next: Option<Rc<DoublyLinkedNode>>,
}

// 用于演示循环引用的节点结构体
struct CyclicNode {
    value: String,
    next: RefCell<Option<Rc<CyclicNode>>>,
}

// 用于演示Weak<T>的结构体
struct Parent {
    name: String,
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    name: String,
    parent: std::rc::Weak<Parent>,
}