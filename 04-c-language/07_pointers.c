#include <stdio.h>

/*
 * 07_pointers.c - C语言指针示例
 * 本示例介绍指针的概念、声明、初始化、操作和应用
 */

int main() {
    // 指针的声明和初始化
    int num = 10;
    int *ptr = &num;  // ptr指向num的地址
    
    printf("指针基础示例：\n");
    printf("变量num的值: %d\n", num);
    printf("变量num的地址: %p\n", &num);
    printf("指针ptr的值（即num的地址）: %p\n", ptr);
    printf("通过指针访问num的值: %d\n", *ptr);  // 解引用操作
    
    // 通过指针修改变量的值
    *ptr = 20;  // 通过指针修改num的值
    printf("\n通过指针修改后：\n");
    printf("变量num的值: %d\n", num);
    printf("通过指针访问num的值: %d\n", *ptr);
    
    // 指针的指针（二级指针）
    int **ptr_to_ptr = &ptr;  // 指向指针的指针
    printf("\n二级指针示例：\n");
    printf("指针ptr的地址: %p\n", &ptr);
    printf("二级指针ptr_to_ptr的值: %p\n", ptr_to_ptr);
    printf("通过二级指针访问num的值: %d\n", **ptr_to_ptr);
    
    // 指针算术运算
    int arr[] = {10, 20, 30, 40, 50};
    int *arr_ptr = arr;  // 数组名本身就是指向第一个元素的指针
    
    printf("\n指针算术运算示例：\n");
    printf("arr[0] = %d (通过数组访问)\n", arr[0]);
    printf("*arr_ptr = %d (通过指针访问第一个元素)\n", *arr_ptr);
    printf("*(arr_ptr + 1) = %d (通过指针算术访问第二个元素)\n", *(arr_ptr + 1));
    printf("arr_ptr[2] = %d (通过指针下标访问第三个元素)\n", arr_ptr[2]);
    
    // 指针遍历数组
    printf("\n使用指针遍历数组：\n");
    for (int i = 0; i < 5; i++) {
        printf("arr[%d] = %d\t", i, *(arr_ptr + i));
    }
    printf("\n");
    
    // 空指针和野指针
    printf("\n空指针示例：\n");
    int *null_ptr = NULL;  // 空指针
    printf("null_ptr的值: %p\n", null_ptr);
    
    // 注意：以下代码在实际运行中可能会导致未定义行为，仅作演示
    printf("\n野指针示例（仅作演示，实际运行可能异常）：\n");
    int *wild_ptr;  // 未初始化的指针（野指针）
    // printf("*wild_ptr = %d\n", *wild_ptr);  // 这行代码可能导致程序崩溃
    
    // 指针作为函数参数
    printf("\n指针作为函数参数示例：\n");
    
    // 演示通过指针交换两个变量的值
    int a = 5, b = 10;
    printf("交换前：a = %d, b = %d\n", a, b);
    
    // 使用指针交换变量（这里直接在main函数中实现，实际中通常写成单独的函数）
    int *ptr_a = &a;
    int *ptr_b = &b;
    int temp = *ptr_a;
    *ptr_a = *ptr_b;
    *ptr_b = temp;
    
    printf("交换后：a = %d, b = %d\n", a, b);
    
    // 字符指针和字符串
    printf("\n字符指针和字符串示例：\n");
    char str1[] = "Hello";  // 字符数组
    char *str2 = "World";   // 字符串常量的指针
    
    printf("str1 = %s\n", str1);
    printf("str2 = %s\n", str2);
    printf("str2[0] = %c\n", str2[0]);
    
    // 注意：不能修改字符串常量
    // str2[0] = 'A';  // 这行代码可能导致程序崩溃
    
    // 动态内存分配的指针（简单提及，后面动态内存章节会详细介绍）
    printf("\n指针和动态内存（简单示例）：\n");
    // int *dynamic_ptr = (int*)malloc(sizeof(int));  // 分配内存
    // *dynamic_ptr = 100;
    // printf("动态分配内存中的值: %d\n", *dynamic_ptr);
    // free(dynamic_ptr);  // 释放内存
    
    return 0;
}