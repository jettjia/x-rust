#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * 09_file_handling.c - C语言文件操作示例
 * 本示例介绍文件的打开、读写和关闭操作
 */

int main() {
    FILE *file;  // 文件指针
    char filename[] = "example.txt";
    char buffer[100];
    
    // 以写入模式打开文件
    printf("文件写入操作示例：\n");
    file = fopen(filename, "w");
    
    if (file == NULL) {
        perror("无法打开文件");
        return 1;
    }
    
    // 向文件写入内容
    fprintf(file, "Hello, C File Handling!\n");
    fprintf(file, "这是一个测试文件。\n");
    fprintf(file, "数字: %d, 浮点数: %.2f\n", 100, 3.14);
    
    // 关闭文件
    fclose(file);
    printf("成功写入文件 '%s'\n", filename);
    
    // 以读取模式打开文件
    printf("\n文件读取操作示例：\n");
    file = fopen(filename, "r");
    
    if (file == NULL) {
        perror("无法打开文件");
        return 1;
    }
    
    // 逐行读取文件内容
    printf("文件内容：\n");
    while (fgets(buffer, sizeof(buffer), file) != NULL) {
        printf("%s", buffer);
    }
    
    // 关闭文件
    fclose(file);
    
    // 以追加模式打开文件
    printf("\n文件追加操作示例：\n");
    file = fopen(filename, "a");
    
    if (file == NULL) {
        perror("无法打开文件");
        return 1;
    }
    
    // 追加内容到文件
    fprintf(file, "\n这是追加的内容。\n");
    fprintf(file, "追加第二行内容。\n");
    
    // 关闭文件
    fclose(file);
    printf("成功追加内容到文件 '%s'\n", filename);
    
    // 再次读取文件以查看追加的内容
    file = fopen(filename, "r");
    
    if (file == NULL) {
        perror("无法打开文件");
        return 1;
    }
    
    printf("\n追加后的文件内容：\n");
    while (fgets(buffer, sizeof(buffer), file) != NULL) {
        printf("%s", buffer);
    }
    
    // 关闭文件
    fclose(file);
    
    // 二进制文件操作示例
    printf("\n二进制文件操作示例：\n");
    
    // 定义一个结构体
    struct Person {
        char name[50];
        int age;
        float height;
    };
    
    struct Person person = {"张三", 25, 175.5};
    struct Person read_person;
    char bin_filename[] = "person.dat";
    
    // 以二进制写入模式打开文件
    FILE *bin_file = fopen(bin_filename, "wb");
    
    if (bin_file == NULL) {
        perror("无法打开二进制文件");
        return 1;
    }
    
    // 写入结构体到二进制文件
    fwrite(&person, sizeof(struct Person), 1, bin_file);
    fclose(bin_file);
    printf("成功写入结构体到二进制文件 '%s'\n", bin_filename);
    
    // 以二进制读取模式打开文件
    bin_file = fopen(bin_filename, "rb");
    
    if (bin_file == NULL) {
        perror("无法打开二进制文件");
        return 1;
    }
    
    // 从二进制文件读取结构体
    fread(&read_person, sizeof(struct Person), 1, bin_file);
    fclose(bin_file);
    
    // 显示读取的结构体内容
    printf("从二进制文件读取的内容：\n");
    printf("姓名: %s\n", read_person.name);
    printf("年龄: %d\n", read_person.age);
    printf("身高: %.1f\n", read_person.height);
    
    // 文件操作错误处理示例
    printf("\n文件操作错误处理示例：\n");
    
    FILE *error_file = fopen("non_existent_file.txt", "r");
    
    if (error_file == NULL) {
        printf("尝试打开不存在的文件时发生错误：");
        perror("");  // 打印系统错误信息
    } else {
        fclose(error_file);
    }
    
    // 文件位置指针操作示例
    printf("\n文件位置指针操作示例：\n");
    
    file = fopen(filename, "r");
    
    if (file == NULL) {
        perror("无法打开文件");
        return 1;
    }
    
    // 读取前10个字符
    fread(buffer, 1, 10, file);
    buffer[10] = '\0';  // 添加字符串结束符
    printf("文件前10个字符: %s\n", buffer);
    
    // 获取当前文件位置
    long current_pos = ftell(file);
    printf("当前文件位置: %ld\n", current_pos);
    
    // 将文件位置指针移动到文件开头
    rewind(file);
    printf("移动到文件开头后，读取整行: ");
    fgets(buffer, sizeof(buffer), file);
    printf("%s", buffer);
    
    // 关闭文件
    fclose(file);
    
    printf("\n文件操作示例完成！\n");
    
    return 0;
}