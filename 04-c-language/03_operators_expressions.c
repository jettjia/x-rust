#include <stdio.h>

/*
 * 03_operators_expressions.c - C语言运算符和表达式示例
 * 本示例介绍C语言的算术、关系、逻辑、位运算和表达式求值
 */

int main() {
    // 算术运算符示例
    int a = 10, b = 3;
    printf("算术运算符示例：\n");
    printf("a = %d, b = %d\n", a, b);
    printf("a + b = %d\n", a + b);
    printf("a - b = %d\n", a - b);
    printf("a * b = %d\n", a * b);
    printf("a / b = %d (整数除法)\n", a / b);
    printf("a %% b = %d (取模运算)\n", a %% b);
    printf("a++ = %d (先使用后自增)\n", a++);
    printf("++a = %d (先自增后使用)\n", ++a);
    
    // 重置a的值
    a = 10;
    
    // 关系运算符示例
    printf("\n关系运算符示例：\n");
    printf("a > b: %d\n", a > b);  // 真为1，假为0
    printf("a < b: %d\n", a < b);
    printf("a == b: %d\n", a == b);
    printf("a != b: %d\n", a != b);
    printf("a >= b: %d\n", a >= b);
    printf("a <= b: %d\n", a <= b);
    
    // 逻辑运算符示例
    int x = 1, y = 0;
    printf("\n逻辑运算符示例：\n");
    printf("x = %d, y = %d\n", x, y);
    printf("x && y: %d\n", x && y);  // 逻辑与
    printf("x || y: %d\n", x || y);  // 逻辑或
    printf("!x: %d\n", !x);          // 逻辑非
    
    // 位运算符示例
    unsigned int m = 12;  // 二进制: 1100
    unsigned int n = 5;   // 二进制: 0101
    printf("\n位运算符示例：\n");
    printf("m = %u (二进制: 1100), n = %u (二进制: 0101)\n", m, n);
    printf("m & n: %u (按位与)\n", m & n);
    printf("m | n: %u (按位或)\n", m | n);
    printf("m ^ n: %u (按位异或)\n", m ^ n);
    printf("~m: %u (按位非)\n", ~m);
    printf("m << 2: %u (左移2位)\n", m << 2);
    printf("m >> 2: %u (右移2位)\n", m >> 2);
    
    // 赋值运算符示例
    int z = 10;
    printf("\n赋值运算符示例：\n");
    printf("初始 z = %d\n", z);
    z += 5;  // 等价于 z = z + 5
    printf("z += 5 后 z = %d\n", z);
    z -= 3;  // 等价于 z = z - 3
    printf("z -= 3 后 z = %d\n", z);
    z *= 2;  // 等价于 z = z * 2
    printf("z *= 2 后 z = %d\n", z);
    z /= 4;  // 等价于 z = z / 4
    printf("z /= 4 后 z = %d\n", z);
    
    // 条件运算符（三目运算符）示例
    int max = (a > b) ? a : b;
    printf("\n条件运算符示例：\n");
    printf("max(%d, %d) = %d\n", a, b, max);
    
    // 运算符优先级示例
    int result = a + b * 2 / (a - b);
    printf("\n运算符优先级示例：\n");
    printf("a + b * 2 / (a - b) = %d\n", result);
    printf("等同于：a + (b * 2) / (a - b)\n");
    
    return 0;
}