#include <stdio.h>

/*
 * 06_arrays.c - C语言数组示例
 * 本示例介绍一维数组、多维数组、数组操作和常见的数组算法
 */

int main() {
    // 一维数组的声明和初始化
    int numbers[5] = {10, 20, 30, 40, 50};  // 显式初始化
    int primes[] = {2, 3, 5, 7, 11, 13};     // 隐式确定大小
    char message[20] = "Hello, C!";          // 字符数组（字符串）
    
    // 访问数组元素
    printf("一维数组示例：\n");
    printf("numbers[0] = %d\n", numbers[0]);
    printf("numbers[2] = %d\n", numbers[2]);
    printf("primes数组长度: %lu\n", sizeof(primes) / sizeof(primes[0]));
    printf("message = %s\n", message);
    
    // 遍历数组
    printf("\n遍历numbers数组：\n");
    for (int i = 0; i < 5; i++) {
        printf("numbers[%d] = %d\n", i, numbers[i]);
    }
    
    // 修改数组元素
    numbers[1] = 25;  // 修改第二个元素
    printf("\n修改后numbers[1] = %d\n", numbers[1]);
    
    // 数组作为函数参数
    printf("\n数组作为函数参数示例：\n");
    
    // 计算数组元素和
    int sum = 0;
    for (int i = 0; i < 5; i++) {
        sum += numbers[i];
    }
    printf("numbers数组元素和: %d\n", sum);
    
    // 查找数组中的最大值
    int max = numbers[0];
    for (int i = 1; i < 5; i++) {
        if (numbers[i] > max) {
            max = numbers[i];
        }
    }
    printf("numbers数组最大值: %d\n", max);
    
    // 二维数组的声明和初始化
    int matrix[3][4] = {
        {1, 2, 3, 4},
        {5, 6, 7, 8},
        {9, 10, 11, 12}
    };
    
    // 访问二维数组元素
    printf("\n二维数组示例：\n");
    printf("matrix[1][2] = %d\n", matrix[1][2]);
    
    // 遍历二维数组
    printf("遍历3x4矩阵：\n");
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 4; j++) {
            printf("%d\t", matrix[i][j]);
        }
        printf("\n");
    }
    
    // 字符数组（字符串）操作
    printf("\n字符数组（字符串）操作：\n");
    
    // 字符串长度计算
    int length = 0;
    while (message[length] != '\0') {
        length++;
    }
    printf("message字符串长度: %d\n", length);
    
    // 字符串拼接示例
    char greeting[30] = "Hello, ";
    char name[] = "C Programmer";
    
    // 手动拼接字符串
    int i = 0;
    while (greeting[i] != '\0') {
        i++;
    }
    
    int j = 0;
    while (name[j] != '\0') {
        greeting[i] = name[j];
        i++;
        j++;
    }
    greeting[i] = '\0';  // 添加字符串结束符
    
    printf("拼接后的字符串: %s\n", greeting);
    
    // 数组的一些常见错误演示
    // 注意：以下代码在实际运行中可能会导致未定义行为，仅作演示
    printf("\n数组的常见错误（仅作演示，实际运行可能异常）：\n");
    printf("numbers[10] = %d (数组越界访问)\n", numbers[10]);
    
    return 0;
}