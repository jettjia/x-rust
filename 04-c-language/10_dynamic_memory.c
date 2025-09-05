#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * 10_dynamic_memory.c - C语言动态内存分配示例
 * 本示例介绍malloc、calloc、realloc和free函数的使用
 */

int main() {
    // malloc函数：分配指定大小的内存块
    printf("malloc函数示例：\n");
    
    // 分配一个整型变量的内存
    int *ptr = (int*)malloc(sizeof(int));
    
    if (ptr == NULL) {
        perror("内存分配失败");
        return 1;
    }
    
    // 使用分配的内存
    *ptr = 100;
    printf("通过malloc分配的内存中存储的值: %d\n", *ptr);
    
    // 释放内存
    free(ptr);
    printf("内存已释放\n");
    
    // 注意：释放后不要再访问这块内存
    // printf("%d\n", *ptr);  // 这行代码可能导致未定义行为
    
    // calloc函数：分配内存并初始化为0
    printf("\ncalloc函数示例：\n");
    
    // 分配5个整型变量的内存并初始化为0
    int *numbers = (int*)calloc(5, sizeof(int));
    
    if (numbers == NULL) {
        perror("内存分配失败");
        return 1;
    }
    
    // 显示calloc初始化的内存内容（应该都是0）
    printf("calloc分配的内存初始值：\n");
    for (int i = 0; i < 5; i++) {
        printf("numbers[%d] = %d\n", i, numbers[i]);
    }
    
    // 为分配的内存赋值
    for (int i = 0; i < 5; i++) {
        numbers[i] = (i + 1) * 10;
    }
    
    // 显示赋值后的内容
    printf("\n为calloc分配的内存赋值后：\n");
    for (int i = 0; i < 5; i++) {
        printf("numbers[%d] = %d\n", i, numbers[i]);
    }
    
    // realloc函数：调整已分配内存的大小
    printf("\nrealloc函数示例：\n");
    
    // 将之前分配的5个整型变量的内存调整为8个
    int *resized_numbers = (int*)realloc(numbers, 8 * sizeof(int));
    
    if (resized_numbers == NULL) {
        perror("内存重分配失败");
        free(numbers);  // 失败时释放原来的内存
        return 1;
    }
    
    // 注意：realloc可能返回新的内存地址，所以更新指针
    numbers = resized_numbers;
    
    // 为新增的内存赋值
    for (int i = 5; i < 8; i++) {
        numbers[i] = (i + 1) * 10;
    }
    
    // 显示重分配后的所有内容
    printf("realloc调整大小后的所有值：\n");
    for (int i = 0; i < 8; i++) {
        printf("numbers[%d] = %d\n", i, numbers[i]);
    }
    
    // 动态分配二维数组
    printf("\n动态分配二维数组示例：\n");
    
    int rows = 3, cols = 4;
    
    // 分配行指针的内存
    int **matrix = (int**)malloc(rows * sizeof(int*));
    
    if (matrix == NULL) {
        perror("内存分配失败");
        free(numbers);
        return 1;
    }
    
    // 为每行分配列的内存
    for (int i = 0; i < rows; i++) {
        matrix[i] = (int*)malloc(cols * sizeof(int));
        
        if (matrix[i] == NULL) {
            perror("内存分配失败");
            // 释放已分配的内存
            for (int j = 0; j < i; j++) {
                free(matrix[j]);
            }
            free(matrix);
            free(numbers);
            return 1;
        }
    }
    
    // 为二维数组赋值
    int count = 1;
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix[i][j] = count++;
        }
    }
    
    // 显示二维数组内容
    printf("动态分配的%d×%d矩阵：\n", rows, cols);
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            printf("%d\t", matrix[i][j]);
        }
        printf("\n");
    }
    
    // 动态内存分配用于字符串
    printf("\n动态内存分配用于字符串示例：\n");
    
    char *str = (char*)malloc(20 * sizeof(char));  // 分配足够存储20个字符的内存
    
    if (str == NULL) {
        perror("内存分配失败");
        // 释放其他已分配的内存
        for (int i = 0; i < rows; i++) {
            free(matrix[i]);
        }
        free(matrix);
        free(numbers);
        return 1;
    }
    
    // 复制字符串到动态分配的内存
    strcpy(str, "Hello, Dynamic Memory!");
    printf("动态分配的字符串: %s\n", str);
    
    // 重新分配更大的内存以存储更长的字符串
    char *resized_str = (char*)realloc(str, 50 * sizeof(char));
    
    if (resized_str == NULL) {
        perror("内存重分配失败");
        free(str);
        // 释放其他已分配的内存
        for (int i = 0; i < rows; i++) {
            free(matrix[i]);
        }
        free(matrix);
        free(numbers);
        return 1;
    }
    
    // 更新指针
    str = resized_str;
    
    // 追加内容到字符串
    strcat(str, " 这是追加的内容。");
    printf("重分配并追加内容后的字符串: %s\n", str);
    
    // 内存泄漏和悬垂指针示例（仅作演示）
    printf("\n内存管理注意事项：\n");
    printf("1. 始终检查内存分配是否成功\n");
    printf("2. 不再使用的内存要及时释放\n");
    printf("3. 释放后的内存不要再访问（避免悬垂指针）\n");
    printf("4. 避免内存泄漏（分配的内存未释放）\n");
    
    // 释放所有动态分配的内存
    free(str);
    
    for (int i = 0; i < rows; i++) {
        free(matrix[i]);
    }
    free(matrix);
    
    free(numbers);
    
    printf("\n所有动态分配的内存已成功释放！\n");
    
    return 0;
}