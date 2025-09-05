#include <stdio.h>

/*
 * 05_functions.c - C语言函数示例
 * 本示例介绍函数定义、参数传递、返回值、递归和函数声明
 */

// 函数声明（原型）
int add(int a, int b);
void greet(char name[]);
int factorial(int n);
void swap_by_value(int x, int y);
void swap_by_reference(int *x, int *y);

// 全局变量
int global_counter = 0;

// 主函数
int main() {
    // 调用函数并获取返回值
    int result = add(5, 3);
    printf("函数调用示例：\n");
    printf("5 + 3 = %d\n", result);
    
    // 调用无返回值函数
    printf("\n无返回值函数调用：\n");
    greet("C语言学习者");
    
    // 调用递归函数
    int n = 5;
    printf("\n递归函数调用：\n");
    printf("%d的阶乘是：%d\n", n, factorial(n));
    
    // 值传递示例
    int x = 10, y = 20;
    printf("\n值传递示例：\n");
    printf("交换前：x = %d, y = %d\n", x, y);
    swap_by_value(x, y);
    printf("交换后：x = %d, y = %d (注意：值传递不改变原变量)\n", x, y);
    
    // 引用传递示例（使用指针）
    printf("\n引用传递示例（使用指针）：\n");
    printf("交换前：x = %d, y = %d\n", x, y);
    swap_by_reference(&x, &y);
    printf("交换后：x = %d, y = %d (引用传递改变了原变量)\n", x, y);
    
    // 全局变量的使用
    printf("\n全局变量的使用：\n");
    printf("初始全局计数器: %d\n", global_counter);
    global_counter += 5;
    printf("修改后全局计数器: %d\n", global_counter);
    
    // 函数嵌套调用
    printf("\n函数嵌套调用：\n");
    int nested_result = add(factorial(3), add(2, 4));
    printf("factorial(3) + (2 + 4) = %d\n", nested_result);
    
    return 0;
}

// 函数定义：计算两个数的和
int add(int a, int b) {
    // 局部变量
    int sum = a + b;
    // 访问全局变量
    global_counter++;
    return sum;
}

// 函数定义：向用户打招呼
void greet(char name[]) {
    printf("你好，%s！欢迎学习C语言函数。\n", name);
    // 无返回值，使用void
}

// 函数定义：计算阶乘（递归实现）
int factorial(int n) {
    // 基本情况
    if (n <= 1) {
        return 1;
    }
    // 递归调用
    return n * factorial(n - 1);
}

// 函数定义：通过值传递交换两个变量（不会改变原变量）
void swap_by_value(int x, int y) {
    int temp = x;
    x = y;
    y = temp;
    // 这里的x和y是局部变量，只在函数内部有效
    printf("函数内部交换后：x = %d, y = %d\n", x, y);
}

// 函数定义：通过引用传递交换两个变量（会改变原变量）
void swap_by_reference(int *x, int *y) {
    int temp = *x;  // 获取指针指向的值
    *x = *y;        // 修改指针指向的变量值
    *y = temp;
}