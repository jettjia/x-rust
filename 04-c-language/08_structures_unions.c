#include <stdio.h>
#include <string.h>

/*
 * 08_structures_unions.c - C语言结构体和联合体示例
 * 本示例介绍结构体、联合体的定义、初始化和使用
 */

// 结构体定义：表示一个学生
struct Student {
    int id;              // 学号
    char name[50];       // 姓名
    float score;         // 分数
    char gender;         // 性别
};

// 结构体定义：表示一个点
struct Point {
    int x;              // x坐标
    int y;              // y坐标
};

// 嵌套结构体：表示一个矩形
struct Rectangle {
    struct Point top_left;     // 左上角点
    struct Point bottom_right; // 右下角点
};

// 联合体定义：可以存储不同类型的数据，但同一时间只能存储一种类型
union Data {
    int i;            // 整型
    float f;          // 浮点型
    char str[20];     // 字符串
};

// 枚举类型定义：表示一周的天数
enum Weekday {
    MONDAY,    // 0
    TUESDAY,   // 1
    WEDNESDAY, // 2
    THURSDAY,  // 3
    FRIDAY,    // 4
    SATURDAY,  // 5
    SUNDAY     // 6
};

// 带typedef的结构体定义
typedef struct {
    char brand[30];
    char model[30];
    int year;
    float price;
} Car;

int main() {
    // 结构体变量的声明和初始化
    struct Student s1 = {101, "张三", 85.5, 'M'};
    
    printf("结构体基础示例：\n");
    printf("学生ID: %d\n", s1.id);
    printf("学生姓名: %s\n", s1.name);
    printf("学生分数: %.1f\n", s1.score);
    printf("学生性别: %c\n", s1.gender);
    
    // 修改结构体成员的值
    s1.score = 90.0;
    strcpy(s1.name, "李四");  // 使用strcpy修改字符串
    
    printf("\n修改后：\n");
    printf("学生姓名: %s\n", s1.name);
    printf("学生分数: %.1f\n", s1.score);
    
    // 结构体数组
    struct Student class[3] = {
        {101, "张三", 85.5, 'M'},
        {102, "李四", 90.0, 'F'},
        {103, "王五", 78.5, 'M'}
    };
    
    printf("\n结构体数组示例：\n");
    for (int i = 0; i < 3; i++) {
        printf("学生%d: ID=%d, 姓名=%s, 分数=%.1f\n", 
               i+1, class[i].id, class[i].name, class[i].score);
    }
    
    // 嵌套结构体
    struct Rectangle rect = { {10, 20}, {30, 40} };
    
    printf("\n嵌套结构体示例：\n");
    printf("矩形左上角坐标: (%d, %d)\n", rect.top_left.x, rect.top_left.y);
    printf("矩形右下角坐标: (%d, %d)\n", rect.bottom_right.x, rect.bottom_right.y);
    
    // 计算矩形面积
    int width = rect.bottom_right.x - rect.top_left.x;
    int height = rect.bottom_right.y - rect.top_left.y;
    int area = width * height;
    printf("矩形面积: %d\n", area);
    
    // 联合体的使用
    union Data data;
    
    printf("\n联合体示例：\n");
    
    // 存储整型
    data.i = 100;
    printf("存储整型后：\n");
    printf("data.i = %d\n", data.i);
    printf("data.f = %f (注意：这里的值是未定义的，因为当前存储的是整型)\n", data.f);
    
    // 存储浮点型（会覆盖之前存储的整型值）
    data.f = 3.14f;
    printf("\n存储浮点型后：\n");
    printf("data.f = %f\n", data.f);
    printf("data.i = %d (注意：这里的值是未定义的，因为当前存储的是浮点型)\n", data.i);
    
    // 存储字符串（会覆盖之前存储的浮点型值）
    strcpy(data.str, "Hello");
    printf("\n存储字符串后：\n");
    printf("data.str = %s\n", data.str);
    printf("data.i = %d (注意：这里的值是未定义的，因为当前存储的是字符串)\n", data.i);
    
    // 枚举类型的使用
    enum Weekday today = WEDNESDAY;
    
    printf("\n枚举类型示例：\n");
    printf("MONDAY的值: %d\n", MONDAY);
    printf("TUESDAY的值: %d\n", TUESDAY);
    printf("WEDNESDAY的值: %d\n", WEDNESDAY);
    printf("今天是星期%d\n", today + 1);  // 加1是因为枚举从0开始
    
    // typedef结构体的使用
    Car my_car = {"Toyota", "Corolla", 2022, 150000.0};
    
    printf("\ntypedef结构体示例：\n");
    printf("汽车品牌: %s\n", my_car.brand);
    printf("汽车型号: %s\n", my_car.model);
    printf("汽车年份: %d\n", my_car.year);
    printf("汽车价格: %.2f\n", my_car.price);
    
    return 0;
}