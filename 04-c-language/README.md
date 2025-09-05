# C语言速通学习材料

这个仓库包含了一系列C语言的示例代码，按照序号排列，覆盖了C语言的各种核心概念和语法特性。这些文件旨在帮助初学者快速学习和掌握C编程语言，类似于Rust速通学习材料。

## 项目结构

每个文件都按照主题进行了分类，并使用数字序号命名，以便按照建议的学习顺序进行阅读和实践。

| 文件名                       | 主题               | 内容简介                                                   |
| ---------------------------- | ------------------ | ---------------------------------------------------------- |
| `01_basic_syntax.c`          | 基础语法           | 介绍C语言的基本语法、程序结构、编译和运行                  |
| `02_variables_datatypes.c`   | 变量和数据类型     | 介绍C语言的变量声明、初始化、基本数据类型和常量            |
| `03_operators_expressions.c` | 运算符和表达式     | 介绍C语言的算术、关系、逻辑、位运算和表达式求值            |
| `04_control_flow.c`          | 控制流             | 介绍if-else、switch-case、循环语句(while、for、do-while)   |
| `05_functions.c`             | 函数               | 介绍函数定义、参数传递、返回值、递归和函数声明             |
| `06_arrays.c`                | 数组               | 介绍一维数组、多维数组、数组初始化和数组操作               |
| `07_pointers.c`              | 指针               | 介绍指针概念、指针运算、指针与数组的关系                   |
| `08_strings.c`               | 字符串             | 介绍字符数组、字符串操作函数、字符串处理技巧               |
| `09_structures_unions.c`     | 结构体和共用体     | 介绍结构体定义、嵌套结构体、共用体和枚举类型               |
| `10_memory_management.c`     | 内存管理           | 介绍动态内存分配(malloc、calloc、realloc、free)和内存泄漏  |
| `11_file_handling.c`         | 文件处理           | 介绍文件打开、读写、关闭和文件操作函数                     |
| `12_preprocessor.c`          | 预处理器           | 介绍预处理指令(#include、#define、条件编译等)              |
| `13_error_handling.c`        | 错误处理           | 介绍错误码、errno、perror和exit函数                        |
| `14_standard_library.c`      | 标准库函数         | 介绍常用的标准库函数(数学、字符串、时间等)                 |
| `15_multithreading.c`        | 多线程编程         | 介绍POSIX线程(pthread)的创建、同步和通信                   |
| `16_system_calls.c`          | 系统调用           | 介绍基本的UNIX/Linux系统调用                               |
| `17_c_vs_cpp.c`              | C与C++比较         | 介绍C和C++的主要区别和兼容性                               |

## 如何运行示例代码

要运行这些示例代码，您需要在系统上安装C编译器（如GCC）。如果您还没有安装，可以按照以下步骤进行安装：

### 在Linux系统上安装GCC
```bash
# Ubuntu/Debian
sudo apt-get update && sudo apt-get install gcc

# CentOS/RHEL
sudo yum install gcc
```

### 在macOS上安装GCC
```bash
# 使用Homebrew
brew install gcc
```

### 在Windows上安装GCC
可以安装MinGW或Cygwin来获取GCC编译器。

运行单个示例文件的步骤：

1. 打开终端，导航到项目目录
2. 使用GCC编译特定的C文件，例如：
   ```bash
   gcc 01_basic_syntax.c -o 01_basic_syntax
   ```
3. 编译成功后，会生成一个可执行文件，使用以下命令运行：
   ```bash
   # Linux/macOS
   ./01_basic_syntax
   
   # Windows
   01_basic_syntax.exe
   ```

## 资源推荐

- [C语言编程(第2版)](https://book.douban.com/subject/1139336/)
- [C Primer Plus(第6版)](https://book.douban.com/subject/25708318/)
- [The C Programming Language(第2版)](https://book.douban.com/subject/1139334/)
- [C语言参考手册](https://zh.cppreference.com/w/c)
- [GCC手册](https://gcc.gnu.org/onlinedocs/)