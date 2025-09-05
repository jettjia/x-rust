#include "helper.h"

/*
 * helper.c - 辅助函数实现
 * 包含辅助函数的具体实现
 */

// 计算两个数的和
int add(int a, int b) {
    return a + b;
}

// 计算两个数的差
int subtract(int a, int b) {
    return a - b;
}

// 计算两个数的积
int multiply(int a, int b) {
    return a * b;
}

// 计算两个数的商（整数除法）
int divide(int a, int b) {
    if (b == 0) {
        // 在实际应用中，应该有更好的错误处理
        return 0;  // 这里简单返回0表示错误
    }
    return a / b;
}