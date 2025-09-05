#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * 12_command_line_args.c - C语言命令行参数示例
 * 本示例介绍如何处理命令行参数和使用环境变量
 */

int main(int argc, char *argv[]) {
    // 显示命令行参数的数量
    printf("命令行参数处理示例：\n\n");
    printf("1. 命令行参数的基本信息：\n");
    printf("参数数量 (argc): %d\n\n", argc);
    
    // 显示所有命令行参数
    printf("2. 所有命令行参数：\n");
    for (int i = 0; i < argc; i++) {
        printf("argv[%d] = %s\n", i, argv[i]);
    }
    
    // 检查是否提供了足够的参数
    if (argc < 2) {
        printf("\n用法: %s <参数1> [参数2] [参数3] ...\n", argv[0]);
        printf("例如: %s hello world\n\n", argv[0]);
    }
    
    // 示例：简单的计算器
    if (argc >= 4) {
        printf("\n3. 简单计算器示例：\n");
        
        // 将字符串参数转换为数值
        double num1 = atof(argv[2]);
        double num2 = atof(argv[3]);
        double result = 0;
        
        // 根据操作符执行相应的计算
        if (strcmp(argv[1], "add") == 0) {
            result = num1 + num2;
            printf("%.2f + %.2f = %.2f\n", num1, num2, result);
        } else if (strcmp(argv[1], "sub") == 0) {
            result = num1 - num2;
            printf("%.2f - %.2f = %.2f\n", num1, num2, result);
        } else if (strcmp(argv[1], "mul") == 0) {
            result = num1 * num2;
            printf("%.2f * %.2f = %.2f\n", num1, num2, result);
        } else if (strcmp(argv[1], "div") == 0) {
            if (num2 != 0) {
                result = num1 / num2;
                printf("%.2f / %.2f = %.2f\n", num1, num2, result);
            } else {
                printf("错误：除数不能为零！\n");
            }
        } else {
            printf("不支持的操作符: %s\n", argv[1]);
            printf("支持的操作符: add, sub, mul, div\n");
        }
    }
    
    // 选项解析示例
    printf("\n4. 选项解析示例：\n");
    int verbose_flag = 0;
    char *name = NULL;
    int number = 0;
    
    // 简单的选项解析（不使用getopt）
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-v") == 0 || strcmp(argv[i], "--verbose") == 0) {
            verbose_flag = 1;
        } else if (strcmp(argv[i], "-n") == 0 || strcmp(argv[i], "--name") == 0) {
            if (i + 1 < argc) {
                name = argv[i + 1];
                i++;  // 跳过参数值
            }
        } else if (strcmp(argv[i], "-num") == 0) {
            if (i + 1 < argc) {
                number = atoi(argv[i + 1]);
                i++;  // 跳过参数值
            }
        }
    }
    
    // 显示解析的选项
    if (verbose_flag) {
        printf("详细模式已启用\n");
    }
    if (name != NULL) {
        printf("名称: %s\n", name);
    }
    if (number > 0) {
        printf("数字: %d\n", number);
    }
    
    // 环境变量示例
    printf("\n5. 环境变量示例：\n");
    
    // 尝试获取一些常见的环境变量
    char *home_dir = getenv("HOME");
    char *user_name = getenv("USER");
    char *path = getenv("PATH");
    char *shell = getenv("SHELL");
    
    if (home_dir != NULL) {
        printf("HOME: %s\n", home_dir);
    }
    if (user_name != NULL) {
        printf("USER: %s\n", user_name);
    }
    if (shell != NULL) {
        printf("SHELL: %s\n", shell);
    }
    if (path != NULL) {
        printf("PATH: %s\n", path);
        // 可以选择是否显示PATH的详细分解
        // printf("\nPATH分解：\n");
        // char *token = strtok(path, ":");
        // while (token != NULL) {
        //     printf("  %s\n", token);
        //     token = strtok(NULL, ":");
        // }
    }
    
    // 设置环境变量
    printf("\n6. 设置环境变量示例：\n");
    
    // 设置一个新的环境变量
    if (setenv("MY_CUSTOM_VAR", "Hello from C program!", 1) == 0) {
        printf("成功设置环境变量 MY_CUSTOM_VAR\n");
        
        // 验证是否设置成功
        char *custom_var = getenv("MY_CUSTOM_VAR");
        if (custom_var != NULL) {
            printf("MY_CUSTOM_VAR: %s\n", custom_var);
        }
    } else {
        printf("设置环境变量失败\n");
    }
    
    // 取消环境变量
    if (unsetenv("MY_CUSTOM_VAR") == 0) {
        printf("成功取消环境变量 MY_CUSTOM_VAR\n");
        
        // 验证是否取消成功
        char *custom_var = getenv("MY_CUSTOM_VAR");
        if (custom_var == NULL) {
            printf("确认：MY_CUSTOM_VAR 已被取消\n");
        }
    } else {
        printf("取消环境变量失败\n");
    }
    
    // 命令行参数处理最佳实践
    printf("\n7. 命令行参数处理最佳实践：\n");
    printf("- 使用argc检查参数数量\n");
    printf("- 使用strcmp比较字符串参数\n");
    printf("- 使用atoi/atof等函数转换数值参数\n");
    printf("- 为程序提供清晰的用法说明\n");
    printf("- 处理可能的错误情况\n");
    printf("- 对于复杂的命令行参数，可以使用getopt/getopt_long函数\n");
    printf("- 环境变量可以用来存储程序配置\n");
    
    printf("\n命令行参数和环境变量示例完成！\n");
    printf("提示：尝试使用不同的参数运行此程序，例如：\n");
    printf("  %s add 10 5\n", argv[0]);
    printf("  %s -v --name Alice -num 42\n", argv[0]);
    
    return 0;
}