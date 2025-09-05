#include <stdio.h>

/*
 * 11_preprocessor.c - C语言预处理指令示例
 * 本示例介绍#define、#include、#if、#ifdef等预处理指令
 */

// 1. 宏定义（#define）
#define PI 3.14159          // 定义常量宏
#define MAX(a, b) ((a) > (b) ? (a) : (b))  // 定义带参数的宏
#define SQUARE(x) ((x) * (x))  // 定义带参数的宏
#define PRINT_MESSAGE() printf("Hello from macro!\n")  // 定义无参数的宏
#define DEBUG               // 定义一个简单的宏（开关）

// 2. 条件编译指令
#ifdef DEBUG
    #define LOG(msg) printf("[DEBUG]: %s\n", msg)  // 调试模式下的日志宏
#else
    #define LOG(msg)  // 非调试模式下，LOG宏为空
#endif

// 3. 文件包含（#include）
// 已经在文件开头包含了stdio.h

// 4. 宏取消定义
#define TEMP_VALUE 100
#undef TEMP_VALUE  // 取消TEMP_VALUE的定义

// 5. 字符串化操作符（#）
#define STRINGIFY(x) #x

// 6. 标记粘贴操作符（##）
#define CONCAT(a, b) a##b

// 7. 条件编译的其他形式
#if __STDC_VERSION__ >= 199901L
    #define C99_FEATURE_AVAILABLE 1
#else
    #define C99_FEATURE_AVAILABLE 0
#endif

int main() {
    // 使用常量宏
    double radius = 5.0;
    double area = PI * radius * radius;
    printf("预处理指令示例：\n\n");
    printf("1. 常量宏的使用：\n");
    printf("圆的半径: %.1f\n", radius);
    printf("圆的面积: %.2f (使用PI = %f)\n\n", area, PI);
    
    // 使用带参数的宏
    int x = 10, y = 20;
    printf("2. 带参数宏的使用：\n");
    printf("MAX(%d, %d) = %d\n", x, y, MAX(x, y));
    printf("SQUARE(%d) = %d\n", x, SQUARE(x));
    PRINT_MESSAGE();
    
    // 演示宏展开时的括号重要性
    int a = 2, b = 3;
    printf("\n3. 宏定义中括号的重要性：\n");
    printf("SQUARE(a + b) = %d (正确: ((a) + (b)) * ((a) + (b)))\n", SQUARE(a + b));
    
    // 使用调试日志宏
    printf("\n4. 条件编译和调试日志：\n");
    LOG("这是一条调试信息");
    LOG("程序正在运行中");
    
    // 检查是否定义了某个宏
    printf("\n5. 检查宏定义：\n");
#ifdef DEBUG
    printf("DEBUG宏已定义\n");
#else
    printf("DEBUG宏未定义\n");
#endif
    
#ifndef TEMP_VALUE
    printf("TEMP_VALUE宏未定义（已被取消）\n");
#else
    printf("TEMP_VALUE宏已定义\n");
#endif
    
    // 使用字符串化操作符
    printf("\n6. 字符串化操作符（#）：\n");
    printf("STRINGIFY(hello) = %s\n", STRINGIFY(hello));
    printf("STRINGIFY(1 + 2) = %s\n", STRINGIFY(1 + 2));
    
    // 使用标记粘贴操作符
    printf("\n7. 标记粘贴操作符（##）：\n");
    int var1 = 100;
    int var2 = 200;
    printf("CONCAT(var, 1) = %d\n", CONCAT(var, 1));
    printf("CONCAT(var, 2) = %d\n", CONCAT(var, 2));
    
    // 检查C99特性是否可用
    printf("\n8. 检查编译器标准：\n");
#if C99_FEATURE_AVAILABLE
    printf("当前编译器支持C99标准\n");
#else
    printf("当前编译器不支持C99标准\n");
#endif
    
    // 行号和文件名宏
    printf("\n9. 预定义宏：\n");
    printf("当前文件名: %s\n", __FILE__);
    printf("当前行号: %d\n", __LINE__);
    printf("当前函数名: %s\n", __func__);
    printf("当前日期: %s\n", __DATE__);
    printf("当前时间: %s\n", __TIME__);
    
    // #error 和 #warning 指令演示（注释掉以避免编译错误）
    // #warning "这是一个警告消息"
    // #error "这是一个错误消息"
    
    // 条件编译的嵌套使用
    printf("\n10. 嵌套条件编译：\n");
#if defined(DEBUG)
    #if C99_FEATURE_AVAILABLE
        printf("调试模式已启用，且支持C99标准\n");
    #else
        printf("调试模式已启用，但不支持C99标准\n");
    #endif
#else
    printf("调试模式已禁用\n");
#endif
    
    printf("\n预处理指令示例完成！\n");
    
    return 0;
}