#include <stdio.h>

/*
 * 01_basic_syntax.c - C语言基本语法示例
 * 本示例介绍C语言的基本程序结构、编译和运行方法
 */

// 主函数 - 程序的入口点
int main() {
    // 输出Hello, World!到控制台
    printf("Hello, World!\n");
    
    // 输出基本C语言程序结构说明
    printf("\nC语言程序基本结构：\n");
    printf("1. 预处理器指令（如#include）\n");
    printf("2. 全局变量和函数声明\n");
    printf("3. 主函数main() - 程序的入口点\n");
    printf("4. 其他函数定义\n");
    
    printf("\n编译和运行方法：\n");
    printf("1. 编译: gcc 01_basic_syntax.c -o 01_basic_syntax\n");
    printf("2. 运行: ./01_basic_syntax\n");
    
    // main函数返回0表示程序正常结束
    return 0;
}