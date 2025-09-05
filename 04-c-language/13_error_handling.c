#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <ctype.h>

/*
 * 13_error_handling.c - C语言错误处理示例
 * 本示例介绍C语言中的错误处理机制
 */

// 定义错误码常量
enum ErrorCode {
    SUCCESS = 0,
    ERROR_INVALID_INPUT = 1,
    ERROR_FILE_NOT_FOUND = 2,
    ERROR_MEMORY_ALLOCATION = 3,
    ERROR_DIVISION_BY_ZERO = 4,
    ERROR_OUT_OF_RANGE = 5
};

// 使用返回值进行错误处理的函数
enum ErrorCode divide(int a, int b, int *result) {
    if (b == 0) {
        return ERROR_DIVISION_BY_ZERO;  // 返回错误码
    }
    
    *result = a / b;
    return SUCCESS;  // 返回成功码
}

// 使用全局变量errno的文件操作示例
void read_file_example(const char *filename) {
    FILE *file = fopen(filename, "r");
    
    if (file == NULL) {
        // 使用perror打印与errno相关的错误信息
        perror("fopen failed");
        printf("错误码: %d, 错误信息: %s\n", errno, strerror(errno));
        return;
    }
    
    // 读取文件内容的示例代码（这里仅作演示）
    char buffer[100];
    if (fgets(buffer, sizeof(buffer), file) != NULL) {
        printf("从文件读取的内容: %s\n", buffer);
    } else {
        printf("读取文件内容失败或文件为空\n");
    }
    
    fclose(file);
}

// 验证用户输入的函数
enum ErrorCode validate_input(const char *input) {
    if (input == NULL) {
        return ERROR_INVALID_INPUT;
    }
    
    // 检查输入是否为空字符串
    if (strlen(input) == 0) {
        return ERROR_INVALID_INPUT;
    }
    
    return SUCCESS;
}

// 动态内存分配并处理错误
void *safe_malloc(size_t size) {
    void *ptr = malloc(size);
    
    if (ptr == NULL) {
        fprintf(stderr, "内存分配失败: %s\n", strerror(errno));
        exit(ERROR_MEMORY_ALLOCATION);
    }
    
    return ptr;
}

// 带错误处理的字符串转换函数
enum ErrorCode string_to_int(const char *str, int *result) {
    if (str == NULL || result == NULL) {
        return ERROR_INVALID_INPUT;
    }
    
    // 跳过前导空白字符
    while (isspace((unsigned char)*str)) {
        str++;
    }
    
    // 检查是否为空字符串
    if (*str == '\0') {
        return ERROR_INVALID_INPUT;
    }
    
    // 处理正负号
    int sign = 1;
    if (*str == '+' || *str == '-') {
        sign = (*str == '-') ? -1 : 1;
        str++;
    }
    
    // 转换数字
    long long value = 0;
    while (isdigit((unsigned char)*str)) {
        value = value * 10 + (*str - '0');
        
        // 检查是否溢出
        if ((sign == 1 && value > INT_MAX) || 
            (sign == -1 && -value < INT_MIN)) {
            return ERROR_OUT_OF_RANGE;
        }
        
        str++;
    }
    
    // 检查是否还有非数字字符
    if (*str != '\0') {
        return ERROR_INVALID_INPUT;
    }
    
    *result = (int)(value * sign);
    return SUCCESS;
}

// 错误处理演示函数
void demonstrate_error_propagation() {
    printf("\n错误传播演示：\n");
    
    // 模拟一个可能失败的操作
    int a = 10, b = 0, result;
    enum ErrorCode error = divide(a, b, &result);
    
    // 处理错误
    if (error != SUCCESS) {
        // 在实际应用中，这里可能会记录日志、清理资源等
        printf("操作失败，错误码: %d\n", error);
        
        // 决定是否继续执行或返回错误给上层
        return;  // 这里选择返回，将错误传播给调用者
    }
    
    // 只有操作成功时才执行后续代码
    printf("操作成功，结果: %d\n", result);
}

// 清理资源的函数示例
void cleanup_resources_example() {
    printf("\n资源清理演示：\n");
    
    FILE *file = NULL;
    char *buffer = NULL;
    
    // 分配资源
    file = fopen("nonexistent_file.txt", "r");
    if (file == NULL) {
        fprintf(stderr, "打开文件失败: %s\n", strerror(errno));
        // 即使失败，也要确保清理了所有已分配的资源
        goto cleanup;  // 使用goto进行统一的资源清理
    }
    
    buffer = (char*)malloc(100);
    if (buffer == NULL) {
        fprintf(stderr, "内存分配失败\n");
        goto cleanup;  // 使用goto进行统一的资源清理
    }
    
    // 如果所有资源都成功分配，则继续处理...
    printf("所有资源分配成功（演示目的）\n");
    
cleanup:
    // 统一清理资源
    if (file != NULL) {
        fclose(file);
        printf("文件已关闭\n");
    }
    if (buffer != NULL) {
        free(buffer);
        printf("内存已释放\n");
    }
}

int main() {
    printf("C语言错误处理示例：\n\n");
    
    // 1. 使用返回值进行错误处理
    printf("1. 使用返回值进行错误处理：\n");
    int a = 10, b = 2, result;
    enum ErrorCode error = divide(a, b, &result);
    
    if (error == SUCCESS) {
        printf("%d / %d = %d\n", a, b, result);
    } else {
        printf("除法运算失败，错误码: %d\n", error);
    }
    
    // 测试除零错误
    b = 0;
    error = divide(a, b, &result);
    if (error == SUCCESS) {
        printf("%d / %d = %d\n", a, b, result);
    } else {
        printf("除法运算失败，错误码: %d\n", error);
    }
    
    // 2. 使用errno和perror进行错误处理
    printf("\n2. 使用errno和perror进行错误处理：\n");
    read_file_example("nonexistent_file.txt");
    
    // 3. 验证用户输入
    printf("\n3. 验证用户输入：\n");
    const char *valid_input = "valid input";
    const char *invalid_input = "";
    
    error = validate_input(valid_input);
    printf("验证 '%s': %s\n", valid_input, 
           (error == SUCCESS) ? "成功" : "失败");
    
    error = validate_input(invalid_input);
    printf("验证 '%s': %s\n", invalid_input, 
           (error == SUCCESS) ? "成功" : "失败");
    
    // 4. 安全的内存分配
    printf("\n4. 安全的内存分配：\n");
    // 以下代码在实际运行中不会导致程序退出，因为分配的内存很小
    char *large_buffer = (char*)safe_malloc(100);
    printf("成功分配100字节内存\n");
    free(large_buffer);
    
    // 5. 字符串转换与错误处理
    printf("\n5. 字符串转换与错误处理：\n");
    const char *str_num1 = "12345";
    const char *str_num2 = "123abc";
    const char *str_num3 = "2147483648";  // 超出int范围
    int num;
    
    error = string_to_int(str_num1, &num);
    printf("转换 '%s': %s, 结果: %d\n", str_num1, 
           (error == SUCCESS) ? "成功" : "失败", num);
    
    error = string_to_int(str_num2, &num);
    printf("转换 '%s': %s\n", str_num2, 
           (error == SUCCESS) ? "成功" : "失败 (包含非数字字符)");
    
    error = string_to_int(str_num3, &num);
    printf("转换 '%s': %s\n", str_num3, 
           (error == SUCCESS) ? "成功" : "失败 (超出范围)");
    
    // 6. 错误传播
    demonstrate_error_propagation();
    
    // 7. 资源清理
    cleanup_resources_example();
    
    // 8. 错误处理最佳实践
    printf("\n8. 错误处理最佳实践：\n");
    printf("- 始终检查函数返回值\n");
    printf("- 使用错误码或errno标识错误类型\n");
    printf("- 提供清晰的错误信息\n");
    printf("- 在错误发生时适当清理资源\n");
    printf("- 考虑使用goto进行统一的资源清理\n");
    printf("- 避免在错误处理中使用exit()，除非是致命错误\n");
    printf("- 对于库函数，将错误信息返回给调用者而不是直接输出\n");
    printf("- 对于可能失败的操作，提供重试机制\n");
    
    printf("\n错误处理示例完成！\n");
    
    return 0;
}