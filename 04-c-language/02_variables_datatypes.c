#include <stdio.h>

/*
 * 02_variables_datatypes.c - C语言变量和数据类型示例
 * 本示例介绍C语言的变量声明、初始化、基本数据类型和常量
 */

// 全局变量声明 - 定义在函数外部
int global_var = 100;
const float PI = 3.14159f;  // 常量定义

int main() {
    // 局部变量声明和初始化
    int a = 10;              // 整型变量
    float b = 3.14f;         // 单精度浮点型变量
    double c = 2.71828;      // 双精度浮点型变量
    char d = 'A';            // 字符型变量
    char str[] = "Hello";    // 字符数组（字符串）
    
    // 输出变量值
    printf("基本数据类型示例：\n");
    printf("整型变量 a = %d\n", a);
    printf("浮点型变量 b = %.2f\n", b);
    printf("双精度变量 c = %.5f\n", c);
    printf("字符型变量 d = %c (ASCII码: %d)\n", d, d);
    printf("字符串 str = %s\n", str);
    
    // 输出全局变量和常量
    printf("\n全局变量和常量示例：\n");
    printf("全局变量 global_var = %d\n", global_var);
    printf("常量 PI = %.5f\n", PI);
    
    // 变量赋值操作
    a = 20;  // 修改变量值
    printf("\n修改变量后：\n");
    printf("a = %d\n", a);
    
    // 自动类型转换示例
    int int_num = 10;
    float float_num = 3.5;
    int result = int_num + float_num;  // float自动转换为int
    printf("\n类型转换示例：\n");
    printf("int_num + float_num = %d (注意：float被截断为int)\n", result);
    
    // 强制类型转换
    float强制转换 = (float)int_num / 3;  // 整数除法转换为浮点除法
    printf("强制类型转换：%d / 3 = %.2f\n", int_num, 强制转换);
    
    return 0;
}