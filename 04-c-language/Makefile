# Makefile - 用于构建C语言项目

# 编译器设置
CC = gcc
CFLAGS = -Wall -Wextra -g  # 编译选项：开启警告，生成调试信息
LDFLAGS =                   # 链接选项

# 目标文件名
TARGET = makefile_example

# 源文件列表
SRCS = 15_makefile_example.c helper.c

# 目标文件列表（将源文件的.c扩展名替换为.o）
OBJS = $(SRCS:.c=.o)

# 头文件列表
HDRS = helper.h

# 默认目标（第一个目标，直接运行make时执行）
all: $(TARGET)

# 链接目标文件生成可执行文件
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS) $(LDFLAGS)

# 编译源文件生成目标文件
%.o: %.c $(HDRS)
	$(CC) $(CFLAGS) -c $< -o $@

# 运行程序
run: $(TARGET)
	./$(TARGET)

# 清理编译生成的文件
clean:
	rm -f $(OBJS) $(TARGET)

# 伪目标（不生成实际文件）
.PHONY: all clean run

# 依赖关系
15_makefile_example.o: 15_makefile_example.c helper.h
helper.o: helper.c helper.h

# 打印变量（用于调试Makefile）
print-vars:
	@echo "CC = $(CC)"
	@echo "CFLAGS = $(CFLAGS)"
	@echo "LDFLAGS = $(LDFLAGS)"
	@echo "TARGET = $(TARGET)"
	@echo "SRCS = $(SRCS)"
	@echo "OBJS = $(OBJS)"
	@echo "HDRS = $(HDRS)"

# 安装目标（示例）
install:
	@echo "安装程序到系统..."
	@echo "实际安装时，这里会包含复制文件到系统目录的命令"

# 卸载目标（示例）
uninstall:
	@echo "从系统卸载程序..."
	@echo "实际卸载时，这里会包含删除系统文件的命令"