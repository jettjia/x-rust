#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>
#include <time.h>
#include <limits.h>
#include <float.h>

/*
 * 14_standard_library.c - C语言标准库函数示例
 * 本示例介绍C语言标准库中的常用函数
 */

int main() {
    printf("C语言标准库函数示例：\n\n");
    
    // 1. stdio.h - 输入输出函数
    printf("1. stdio.h - 输入输出函数：\n");
    
    // printf - 格式化输出
    int num = 100;
    float fnum = 3.14f;
    char ch = 'A';
    char str[] = "Hello, C!";
    
    printf("格式化输出示例：\n");
    printf("整数: %d\n", num);
    printf("浮点数: %.2f\n", fnum);
    printf("字符: %c (ASCII: %d)\n", ch, ch);
    printf("字符串: %s\n", str);
    printf("十六进制: %x\n", num);
    printf("八进制: %o\n", num);
    printf("指针地址: %p\n", &num);
    
    // scanf - 格式化输入（在实际运行时需要用户输入）
    printf("\nscanf示例（演示目的，实际运行时需要用户输入）：\n");
    printf("提示：使用scanf可以读取用户输入，例如：\n");
    printf("  int user_input;\n");
    printf("  printf(\"请输入一个整数：\");\n");
    printf("  scanf(\"%d\", &user_input);\n");
    
    // 2. stdlib.h - 常用工具函数
    printf("\n2. stdlib.h - 常用工具函数：\n");
    
    // 字符串转换函数
    const char *str_int = "12345";
    const char *str_float = "3.14159";
    const char *str_hex = "FF";
    
    int i = atoi(str_int);
    float f = atof(str_float);
    int h = strtol(str_hex, NULL, 16);
    
    printf("字符串转换函数：\n");
    printf("atoi(\"%s\") = %d\n", str_int, i);
    printf("atof(\"%s\") = %f\n", str_float, f);
    printf("strtol(\"%s\", NULL, 16) = %d (十进制)\n", str_hex, h);
    
    // 随机数生成
    printf("\n随机数生成：\n");
    srand(time(NULL));  // 设置随机数种子
    printf("生成5个随机数：\n");
    for (int j = 0; j < 5; j++) {
        printf("%d\t", rand());
    }
    printf("\n");
    
    // 生成指定范围的随机数
    int min = 1, max = 100;
    printf("生成5个%d到%d之间的随机数：\n", min, max);
    for (int j = 0; j < 5; j++) {
        int rand_num = min + rand() % (max - min + 1);
        printf("%d\t", rand_num);
    }
    printf("\n");
    
    // 内存分配函数（在动态内存章节已经详细介绍）
    printf("\n内存分配函数：\n");
    printf("malloc(), calloc(), realloc(), free()\n");
    
    // 3. string.h - 字符串处理函数
    printf("\n3. string.h - 字符串处理函数：\n");
    
    char str1[50] = "Hello";
    char str2[50] = "World";
    char str3[50];
    
    // 字符串复制
    strcpy(str3, str1);
    printf("strcpy(str3, str1) -> str3 = %s\n", str3);
    
    // 安全的字符串复制（指定最大长度）
    strncpy(str3, str2, sizeof(str3) - 1);
    str3[sizeof(str3) - 1] = '\0';  // 确保字符串以空字符结尾
    printf("strncpy(str3, str2, sizeof(str3)-1) -> str3 = %s\n", str3);
    
    // 字符串连接
    strcpy(str3, str1);  // 先恢复str3
    strcat(str3, " ");
    strcat(str3, str2);
    printf("strcat(str3, \" \"); strcat(str3, str2) -> str3 = %s\n", str3);
    
    // 安全的字符串连接
    strcpy(str3, str1);  // 先恢复str3
    strncat(str3, " ", sizeof(str3) - strlen(str3) - 1);
    strncat(str3, str2, sizeof(str3) - strlen(str3) - 1);
    printf("strncat安全连接 -> str3 = %s\n", str3);
    
    // 字符串比较
    printf("strcmp(\"abc\", \"abd\") = %d\n", strcmp("abc", "abd"));
    printf("strcmp(\"abc\", \"abc\") = %d\n", strcmp("abc", "abc"));
    printf("strcmp(\"abd\", \"abc\") = %d\n", strcmp("abd", "abc"));
    
    // 字符串长度
    printf("strlen(\"%s\") = %lu\n", str3, strlen(str3));
    
    // 字符串查找
    char *pos = strstr(str3, "World");
    if (pos != NULL) {
        printf("strstr(str3, \"World\") 找到，位置: %ld\n", pos - str3);
    }
    
    // 4. ctype.h - 字符处理函数
    printf("\n4. ctype.h - 字符处理函数：\n");
    
    char test_chars[] = "aZ9! \t";
    printf("字符测试：\n");
    
    for (int j = 0; test_chars[j] != '\0'; j++) {
        printf("'%c':\t", test_chars[j]);
        printf("isalpha: %d\t", isalpha((unsigned char)test_chars[j]));
        printf("isdigit: %d\t", isdigit((unsigned char)test_chars[j]));
        printf("islower: %d\t", islower((unsigned char)test_chars[j]));
        printf("isupper: %d\t", isupper((unsigned char)test_chars[j]));
        printf("isspace: %d\t", isspace((unsigned char)test_chars[j]));
        printf("ispunct: %d\n", ispunct((unsigned char)test_chars[j]));
    }
    
    // 字符转换
    printf("\n字符转换：\n");
    printf("tolower('A') = %c\n", tolower((unsigned char)'A'));
    printf("toupper('a') = %c\n", toupper((unsigned char)'a'));
    
    // 5. math.h - 数学函数
    printf("\n5. math.h - 数学函数：\n");
    
    double x = 2.0, y = 3.0;
    
    printf("基本数学函数：\n");
    printf("sqrt(%.1f) = %.2f\n", x, sqrt(x));
    printf("pow(%.1f, %.1f) = %.2f\n", x, y, pow(x, y));
    printf("exp(%.1f) = %.2f\n", x, exp(x));
    printf("log(%.1f) = %.2f\n", x, log(x));
    printf("log10(%.1f) = %.2f\n", x*5, log10(x*5));
    printf("sin(%.1f) = %.2f\n", x, sin(x));
    printf("cos(%.1f) = %.2f\n", x, cos(x));
    printf("tan(%.1f) = %.2f\n", x, tan(x));
    printf("abs(%d) = %d\n", -10, abs(-10));
    printf("fabs(-%.1f) = %.2f\n", x, fabs(-x));
    printf("ceil(%.1f) = %.1f\n", 2.3, ceil(2.3));
    printf("floor(%.1f) = %.1f\n", 2.7, floor(2.7));
    printf("round(%.1f) = %.1f\n", 2.5, round(2.5));
    
    // 6. time.h - 时间函数
    printf("\n6. time.h - 时间函数：\n");
    
    // 获取当前时间
    time_t now = time(NULL);
    printf("当前时间戳: %ld\n", now);
    
    // 转换为本地时间
    struct tm *local_time = localtime(&now);
    
    // 格式化时间字符串
    char time_str[100];
    strftime(time_str, sizeof(time_str), "%Y-%m-%d %H:%M:%S", local_time);
    printf("格式化时间: %s\n", time_str);
    
    // 7. limits.h 和 float.h - 数据类型限制
    printf("\n7. 数据类型限制：\n");
    
    printf("整型限制（limits.h）：\n");
    printf("INT_MIN: %d\n", INT_MIN);
    printf("INT_MAX: %d\n", INT_MAX);
    printf("LONG_MIN: %ld\n", LONG_MIN);
    printf("LONG_MAX: %ld\n", LONG_MAX);
    printf("UINT_MAX: %u\n", UINT_MAX);
    printf("CHAR_BIT: %d\n", CHAR_BIT);
    
    printf("\n浮点型限制（float.h）：\n");
    printf("FLT_MIN: %e\n", FLT_MIN);
    printf("FLT_MAX: %e\n", FLT_MAX);
    printf("DBL_MIN: %e\n", DBL_MIN);
    printf("DBL_MAX: %e\n", DBL_MAX);
    printf("FLT_DIG: %d\n", FLT_DIG);
    printf("DBL_DIG: %d\n", DBL_DIG);
    
    // 8. 其他有用的标准库函数
    printf("\n8. 其他有用的标准库函数：\n");
    
    // exit() 和 atexit()
    printf("exit() - 终止程序执行\n");
    printf("atexit() - 注册程序退出时执行的函数\n");
    
    // qsort() - 快速排序
    int arr[] = {5, 2, 9, 1, 5, 6};
    int n = sizeof(arr) / sizeof(arr[0]);
    
    printf("排序前数组：");
    for (int j = 0; j < n; j++) {
        printf("%d ", arr[j]);
    }
    printf("\n");
    
    // 比较函数
    int compare(const void *a, const void *b) {
        return (*(int*)a - *(int*)b);
    }
    
    qsort(arr, n, sizeof(int), compare);
    
    printf("qsort排序后数组：");
    for (int j = 0; j < n; j++) {
        printf("%d ", arr[j]);
    }
    printf("\n");
    
    // bsearch() - 二分查找（要求数组已排序）
    int key = 5;
    int *found = (int*)bsearch(&key, arr, n, sizeof(int), compare);
    if (found != NULL) {
        printf("bsearch查找 %d: 找到，值为 %d\n", key, *found);
    } else {
        printf("bsearch查找 %d: 未找到\n", key);
    }
    
    // 9. 标准库使用最佳实践
    printf("\n9. 标准库使用最佳实践：\n");
    printf("- 熟悉常用的标准库函数，可以提高编程效率\n");
    printf("- 使用安全的字符串函数（如strncpy、strncat等）避免缓冲区溢出\n");
    printf("- 注意处理标准库函数的返回值，尤其是可能失败的函数\n");
    printf("- 了解数据类型的限制，避免溢出\n");
    printf("- 对于跨平台程序，注意标准库函数的平台差异\n");
    printf("- 使用头文件保护防止头文件重复包含\n");
    printf("- 合理使用静态库和动态库\n");
    
    printf("\n标准库函数示例完成！\n");
    
    return 0;
}