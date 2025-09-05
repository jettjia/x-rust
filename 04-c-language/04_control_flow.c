#include <stdio.h>

/*
 * 04_control_flow.c - C语言控制流语句示例
 * 本示例介绍if-else、switch-case、循环语句(while、for、do-while)
 */

int main() {
    // if-else语句示例
    int number = 75;
    printf("if-else语句示例：\n");
    printf("分数: %d\n", number);
    
    if (number >= 90) {
        printf("成绩: A\n");
    } else if (number >= 80) {
        printf("成绩: B\n");
    } else if (number >= 70) {
        printf("成绩: C\n");
    } else if (number >= 60) {
        printf("成绩: D\n");
    } else {
        printf("成绩: F (不及格)\n");
    }
    
    // switch-case语句示例
    char grade = 'B';
    printf("\nswitch-case语句示例：\n");
    printf("等级: %c\n", grade);
    
    switch (grade) {
        case 'A':
            printf("优秀\n");
            break;
        case 'B':
            printf("良好\n");
            break;
        case 'C':
            printf("中等\n");
            break;
        case 'D':
            printf("及格\n");
            break;
        case 'F':
            printf("不及格\n");
            break;
        default:
            printf("无效等级\n");
    }
    
    // while循环示例 - 计算1到10的和
    int sum = 0;
    int i = 1;
    printf("\nwhile循环示例 (计算1到10的和):\n");
    
    while (i <= 10) {
        sum += i;
        i++;
    }
    printf("1 + 2 + ... + 10 = %d\n", sum);
    
    // do-while循环示例 - 至少执行一次
    int j = 1;
    sum = 0;
    printf("\ndo-while循环示例 (计算1到5的和):\n");
    
    do {
        sum += j;
        j++;
    } while (j <= 5);
    printf("1 + 2 + ... + 5 = %d\n", sum);
    
    // for循环示例 - 打印乘法表
    printf("\nfor循环示例 (打印乘法表):\n");
    
    for (int m = 1; m <= 5; m++) {
        for (int n = 1; n <= m; n++) {
            printf("%d*%d=%d\t", n, m, n*m);
        }
        printf("\n");
    }
    
    // break和continue语句示例
    printf("\nbreak和continue语句示例:\n");
    
    for (int k = 1; k <= 10; k++) {
        if (k == 5) {
            printf("遇到k=5，跳过\n");
            continue;  // 跳过当前循环的剩余部分，继续下一次循环
        }
        if (k == 8) {
            printf("遇到k=8，终止循环\n");
            break;  // 立即终止循环
        }
        printf("k = %d\n", k);
    }
    
    // 嵌套条件和循环示例
    printf("\n嵌套条件和循环示例 (打印100以内的偶数):\n");
    
    int count = 0;
    for (int num = 1; num <= 100; num++) {
        if (num % 2 == 0) {
            printf("%d\t", num);
            count++;
            if (count % 10 == 0) {
                printf("\n");  // 每10个数换行
            }
        }
    }
    printf("\n");
    
    return 0;
}