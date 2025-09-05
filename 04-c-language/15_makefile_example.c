#include <stdio.h>
#include "helper.h"

/*
 * 15_makefile_example.c - C语言Makefile示例
 * 本示例展示如何使用Makefile来编译和管理C语言项目
 */

int main() {
    printf("Makefile项目构建示例：\n\n");
    
    // 调用主函数中的功能
    printf("1. 主程序功能：\n");
    printf("   这个程序演示了如何使用Makefile来组织和构建C语言项目。\n");
    printf("   Makefile可以自动处理依赖关系，只重新编译修改过的文件，\n");
    printf("   从而提高编译效率。\n\n");
    
    // 调用辅助函数（从helper.c中定义）
    printf("2. 调用辅助函数：\n");
    int a = 10, b = 5;
    printf("   add(%d, %d) = %d\n", a, b, add(a, b));
    printf("   subtract(%d, %d) = %d\n", a, b, subtract(a, b));
    printf("   multiply(%d, %d) = %d\n", a, b, multiply(a, b));
    
    // 计算并显示结果
    if (b != 0) {
        printf("   divide(%d, %d) = %d\n", a, b, divide(a, b));
    }
    
    // 显示Makefile使用说明
    printf("\n3. Makefile使用说明：\n");
    printf("   在项目目录下，可以使用以下命令：\n");
    printf("   - make          : 编译整个项目\n");
    printf("   - make clean    : 清理编译生成的文件\n");
    printf("   - make run      : 编译并运行程序\n");
    printf("   - make all      : 编译项目（同make）\n\n");
    
    // 显示项目结构
    printf("4. 项目结构：\n");
    printf("   15_makefile_example.c   : 主程序文件\n");
    printf("   helper.c                : 辅助函数实现\n");
    printf("   helper.h                : 辅助函数头文件\n");
    printf("   Makefile                : 构建配置文件\n\n");
    
    // 显示Makefile的好处
    printf("5. Makefile的好处：\n");
    printf("   - 自动处理依赖关系\n");
    printf("   - 只重新编译修改过的文件\n");
    printf("   - 集中管理编译选项\n");
    printf("   - 简化编译命令\n");
    printf("   - 支持多目标构建\n\n");
    
    // 提示用户如何扩展Makefile
    printf("6. 扩展Makefile：\n");
    printf("   - 添加更多源文件：在SRCS变量中添加新的.c文件\n");
    printf("   - 添加编译选项：在CFLAGS变量中添加\n");
    printf("   - 添加链接选项：在LDFLAGS变量中添加\n");
    printf("   - 添加新的构建目标：如测试目标\n\n");
    
    printf("Makefile示例完成！\n");
    
    return 0;
}