#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * 17_c_vs_cpp.c - C与C++比较示例
 * 本示例展示C和C++的主要区别和兼容性
 * 包括：
 * 1. 基本语法区别
 * 2. 面向对象特性
 * 3. 类型系统差异
 * 4. 标准库差异
 * 5. 内存管理
 * 6. 兼容性和互操作性
 */

// 为了让这个C文件能够编译，我们会使用条件编译来处理C++特有的代码
// 在实际的C++项目中，这些代码不需要条件编译

// C风格的结构体和函数
struct Point {
    int x;
    int y;
};

// C风格的函数
void print_point(struct Point p) {
    printf("Point(%d, %d)\n", p.x, p.y);
}

// C风格的全局变量
int global_counter = 0;

// 简单的C风格内存管理示例
void c_memory_example() {
    printf("\n=== C风格内存管理 ===\n");
    
    // 使用malloc分配内存
    int *arr = (int *)malloc(5 * sizeof(int));
    if (arr != NULL) {
        // 初始化数组
        for (int i = 0; i < 5; i++) {
            arr[i] = i * 10;
        }
        
        // 打印数组内容
        printf("C数组内容: ");
        for (int i = 0; i < 5; i++) {
            printf("%d ", arr[i]);
        }
        printf("\n");
        
        // 释放内存
        free(arr);
    }
}

// C风格的字符串处理
void c_string_example() {
    printf("\n=== C风格字符串处理 ===\n");
    
    char *str1 = "Hello, C!";
    char str2[20];
    
    // 使用strcpy复制字符串
    strcpy(str2, str1);
    printf("str1: %s\n", str1);
    printf("str2: %s\n", str2);
    
    // 使用strlen获取字符串长度
    printf("str1长度: %lu\n", strlen(str1));
    
    // 使用strcat拼接字符串
    strcat(str2, " Welcome!");
    printf("拼接后str2: %s\n", str2);
}

// 简单展示C++与C的区别的函数（这些代码不会在C编译器中执行）
// 在实际C++代码中，这些注释会被移除
void cpp_vs_c_examples() {
    // 在C++中，下面的代码是有效的：
    /*
    // 1. C++中的命名空间
    namespace MyNamespace {
        int add(int a, int b) {
            return a + b;
        }
    }
    
    // 2. C++中的类和对象
    class Rectangle {
    private:
        int width;
        int height;
    public:
        Rectangle(int w, int h) : width(w), height(h) {}
        int area() { return width * height; }
        void setDimensions(int w, int h) {
            width = w;
            height = h;
        }
    };
    
    // 3. C++中的引用
    int x = 10;
    int& ref_x = x;
    ref_x = 20; // 这会修改x的值
    
    // 4. C++中的模板
    template <typename T>
    T maximum(T a, T b) {
        return a > b ? a : b;
    }
    
    // 5. C++中的异常处理
    try {
        int* ptr = new int[1000000]; // 分配大量内存
        delete[] ptr;
    } catch (std::bad_alloc& e) {
        std::cerr << "内存分配失败: " << e.what() << std::endl;
    }
    
    // 6. C++中的标准库容器
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::string cpp_str = "Hello, C++!";
    std::map<std::string, int> scores;
    scores["Alice"] = 95;
    scores["Bob"] = 87;
    
    // 7. C++中的智能指针
    std::unique_ptr<int> unique_ptr(new int(42));
    std::shared_ptr<int> shared_ptr1 = std::make_shared<int>(100);
    std::shared_ptr<int> shared_ptr2 = shared_ptr1; // 引用计数增加
    */
}

// C++调用C代码的示例（展示兼容性）
// 在C++文件中，通常使用extern "C"来调用C函数
/*
#ifdef __cplusplus
extern "C" {
#endif

// C函数声明
void c_function();

#ifdef __cplusplus
}
#endif
*/

// C调用C++代码的示例（通过封装）
// C++代码需要被封装在extern "C"块中
/*
#ifdef __cplusplus
extern "C" {
#endif

// C可调用的接口函数
void call_cpp_function() {
    // 这里可以调用C++代码
}

#ifdef __cplusplus
}
#endif
*/

// C和C++的主要区别总结
void summarize_differences() {
    printf("\n=== C与C++的主要区别总结 ===\n");
    printf("1. 编程范式:\n");
    printf("   - C: 过程式编程\n");
    printf("   - C++: 支持过程式、面向对象、泛型和函数式编程\n");
    printf("\n2. 类型系统:\n");
    printf("   - C++有更严格的类型检查\n");
    printf("   - C++支持引用、const引用、类型转换运算符\n");
    printf("   - C++有bool类型（C99也引入了_Bool）\n");
    printf("\n3. 内存管理:\n");
    printf("   - C: 手动管理（malloc/free）\n");
    printf("   - C++: 手动管理（new/delete）+ 智能指针\n");
    printf("\n4. 标准库:\n");
    printf("   - C++有更丰富的标准库（STL）\n");
    printf("   - C++支持容器、算法、迭代器等高级特性\n");
    printf("\n5. 面向对象特性:\n");
    printf("   - C++支持类、继承、多态、封装、抽象\n");
    printf("   - C通过结构体和函数指针模拟面向对象\n");
    printf("\n6. 其他特性:\n");
    printf("   - C++支持命名空间、模板、异常处理、运算符重载\n");
    printf("   - C++有更复杂的编译器和链接过程\n");
}

// C和C++的兼容性说明
void compatibility_notes() {
    printf("\n=== C与C++的兼容性 ===\n");
    printf("1. C++兼容大部分C代码，但有一些例外\n");
    printf("2. C++编译器可以编译大部分C程序\n");
    printf("3. 在C++中调用C代码需要使用extern \"C\"\n");
    printf("4. 在C中调用C++代码需要通过C接口封装C++功能\n");
    printf("5. 一些C99特性在C++中不被支持或有不同实现\n");
    printf("6. 一些关键字在C和C++中有不同含义\n");
}

// 何时选择C而非C++
void when_to_use_c() {
    printf("\n=== 何时选择C而非C++ ===\n");
    printf("1. 嵌入式系统或资源受限环境\n");
    printf("2. 操作系统内核和底层系统编程\n");
    printf("3. 需要直接控制硬件的应用\n");
    printf("4. 简单、直接的程序，不需要面向对象特性\n");
    printf("5. 需要与C语言库或系统紧密集成\n");
    printf("6. 对编译输出大小和性能有严格要求\n");
}

// 主函数
int main() {
    printf("C与C++比较示例\n");
    printf("======================\n");
    
    // 演示C风格的结构体和函数
    struct Point p = {10, 20};
    print_point(p);
    
    // 演示C风格内存管理
    c_memory_example();
    
    // 演示C风格字符串处理
    c_string_example();
    
    // 总结C和C++的主要区别
    summarize_differences();
    
    // 说明C和C++的兼容性
    compatibility_notes();
    
    // 讨论何时选择C而非C++
    when_to_use_c();
    
    printf("\n示例结束！\n");
    
    return 0;
}