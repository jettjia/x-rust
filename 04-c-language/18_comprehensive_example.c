#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <time.h>

/*
 * 17_comprehensive_example.c - C语言综合示例
 * 本示例展示如何将C语言的各个知识点结合起来构建一个简单但完整的应用程序
 * 功能：一个简单的学生管理系统
 */

// 定义常量
#define MAX_STUDENTS 50
#define MAX_NAME_LENGTH 50
#define MAX_ID_LENGTH 20
#define COURSE_COUNT 3

// 定义学生结构体
typedef struct {
    char id[MAX_ID_LENGTH];     // 学号
    char name[MAX_NAME_LENGTH]; // 姓名
    int age;                    // 年龄
    float scores[COURSE_COUNT]; // 成绩数组
    float average;              // 平均分
} Student;

// 定义学生管理系统结构体
typedef struct {
    Student students[MAX_STUDENTS]; // 学生数组
    int count;                      // 当前学生数量
} StudentManager;

// 函数声明
void init_manager(StudentManager *manager);
int add_student(StudentManager *manager, const Student *student);
Student* find_student_by_id(StudentManager *manager, const char *id);
int delete_student_by_id(StudentManager *manager, const char *id);
void update_student(StudentManager *manager, const char *id, const Student *new_student);
void calculate_averages(StudentManager *manager);
void sort_students_by_average(StudentManager *manager);
void print_all_students(const StudentManager *manager);
void print_student(const Student *student);
void save_to_file(const StudentManager *manager, const char *filename);
int load_from_file(StudentManager *manager, const char *filename);
void clear_manager(StudentManager *manager);
void display_menu();
void handle_user_input(StudentManager *manager);
int validate_id(const char *id);
int validate_name(const char *name);
int validate_age(int age);
int validate_score(float score);
void generate_sample_data(StudentManager *manager);

// 主函数
int main() {
    StudentManager manager;
    
    // 初始化学生管理系统
    init_manager(&manager);
    
    // 生成一些示例数据
    generate_sample_data(&manager);
    
    printf("C语言综合示例：学生管理系统\n");
    printf("================================\n\n");
    
    // 处理用户输入
    handle_user_input(&manager);
    
    // 清理资源
    clear_manager(&manager);
    
    printf("\n感谢使用学生管理系统！\n");
    
    return 0;
}

// 初始化学生管理系统
void init_manager(StudentManager *manager) {
    if (manager == NULL) {
        return;
    }
    manager->count = 0;
    memset(manager->students, 0, sizeof(manager->students));
}

// 添加学生
int add_student(StudentManager *manager, const Student *student) {
    if (manager == NULL || student == NULL) {
        return 0;  // 失败
    }
    
    // 检查是否已达到最大学生数量
    if (manager->count >= MAX_STUDENTS) {
        printf("错误：已达到最大学生数量（%d）\n", MAX_STUDENTS);
        return 0;  // 失败
    }
    
    // 检查学生ID是否已存在
    if (find_student_by_id(manager, student->id) != NULL) {
        printf("错误：学号 '%s' 已存在\n", student->id);
        return 0;  // 失败
    }
    
    // 复制学生信息到管理系统
    memcpy(&manager->students[manager->count], student, sizeof(Student));
    manager->count++;
    
    // 计算平均分
    calculate_averages(manager);
    
    return 1;  // 成功
}

// 根据ID查找学生
Student* find_student_by_id(StudentManager *manager, const char *id) {
    if (manager == NULL || id == NULL) {
        return NULL;
    }
    
    for (int i = 0; i < manager->count; i++) {
        if (strcmp(manager->students[i].id, id) == 0) {
            return &manager->students[i];
        }
    }
    
    return NULL;  // 未找到
}

// 根据ID删除学生
int delete_student_by_id(StudentManager *manager, const char *id) {
    if (manager == NULL || id == NULL) {
        return 0;  // 失败
    }
    
    // 查找学生
    Student *student = find_student_by_id(manager, id);
    
    if (student == NULL) {
        printf("错误：未找到学号为 '%s' 的学生\n", id);
        return 0;  // 失败
    }
    
    // 计算要删除的学生在数组中的索引
    int index = student - manager->students;
    
    // 将后面的学生向前移动，覆盖要删除的学生
    for (int i = index; i < manager->count - 1; i++) {
        memcpy(&manager->students[i], &manager->students[i+1], sizeof(Student));
    }
    
    // 减少学生计数
    manager->count--;
    
    // 清空最后一个位置（可选）
    memset(&manager->students[manager->count], 0, sizeof(Student));
    
    return 1;  // 成功
}

// 更新学生信息
void update_student(StudentManager *manager, const char *id, const Student *new_student) {
    if (manager == NULL || id == NULL || new_student == NULL) {
        return;
    }
    
    // 查找学生
    Student *student = find_student_by_id(manager, id);
    
    if (student == NULL) {
        printf("错误：未找到学号为 '%s' 的学生\n", id);
        return;
    }
    
    // 更新学生信息（注意：不更新学号）
    strcpy(student->name, new_student->name);
    student->age = new_student->age;
    memcpy(student->scores, new_student->scores, sizeof(student->scores));
    
    // 重新计算平均分
    calculate_averages(manager);
}

// 计算所有学生的平均分
void calculate_averages(StudentManager *manager) {
    if (manager == NULL) {
        return;
    }
    
    for (int i = 0; i < manager->count; i++) {
        float sum = 0;
        for (int j = 0; j < COURSE_COUNT; j++) {
            sum += manager->students[i].scores[j];
        }
        manager->students[i].average = sum / COURSE_COUNT;
    }
}

// 按平均分对学生进行排序（从高到低）
void sort_students_by_average(StudentManager *manager) {
    if (manager == NULL) {
        return;
    }
    
    for (int i = 0; i < manager->count - 1; i++) {
        for (int j = 0; j < manager->count - i - 1; j++) {
            if (manager->students[j].average < manager->students[j+1].average) {
                // 交换学生信息
                Student temp = manager->students[j];
                manager->students[j] = manager->students[j+1];
                manager->students[j+1] = temp;
            }
        }
    }
}

// 打印所有学生信息
void print_all_students(const StudentManager *manager) {
    if (manager == NULL || manager->count == 0) {
        printf("当前没有学生记录\n");
        return;
    }
    
    printf("\n学生列表（共%d名学生）：\n", manager->count);
    printf("------------------------------------------------------------\n");
    printf("学号\t\t姓名\t\t年龄\t成绩1\t成绩2\t成绩3\t平均分\n");
    printf("------------------------------------------------------------\n");
    
    for (int i = 0; i < manager->count; i++) {
        const Student *student = &manager->students[i];
        printf("%s\t%s\t\t%d\t", student->id, student->name, student->age);
        
        for (int j = 0; j < COURSE_COUNT; j++) {
            printf("%.1f\t", student->scores[j]);
        }
        
        printf("%.2f\n", student->average);
    }
    
    printf("------------------------------------------------------------\n");
}

// 打印单个学生信息
void print_student(const Student *student) {
    if (student == NULL) {
        return;
    }
    
    printf("\n学生详情：\n");
    printf("学号：%s\n", student->id);
    printf("姓名：%s\n", student->name);
    printf("年龄：%d\n", student->age);
    printf("成绩：\n");
    
    for (int j = 0; j < COURSE_COUNT; j++) {
        printf("  课程%d：%.1f\n", j+1, student->scores[j]);
    }
    
    printf("平均分：%.2f\n", student->average);
}

// 将学生数据保存到文件
void save_to_file(const StudentManager *manager, const char *filename) {
    if (manager == NULL || filename == NULL) {
        return;
    }
    
    FILE *file = fopen(filename, "wb");
    
    if (file == NULL) {
        perror("无法打开文件进行写入");
        return;
    }
    
    // 写入学生数量
    fwrite(&manager->count, sizeof(int), 1, file);
    
    // 写入每个学生的信息
    for (int i = 0; i < manager->count; i++) {
        fwrite(&manager->students[i], sizeof(Student), 1, file);
    }
    
    fclose(file);
    printf("成功将%d名学生的数据保存到文件 '%s'\n", manager->count, filename);
}

// 从文件加载学生数据
int load_from_file(StudentManager *manager, const char *filename) {
    if (manager == NULL || filename == NULL) {
        return 0;  // 失败
    }
    
    FILE *file = fopen(filename, "rb");
    
    if (file == NULL) {
        perror("无法打开文件进行读取");
        return 0;  // 失败
    }
    
    // 读取学生数量
    int count;
    if (fread(&count, sizeof(int), 1, file) != 1) {
        printf("文件格式错误或为空\n");
        fclose(file);
        return 0;  // 失败
    }
    
    // 检查学生数量是否合法
    if (count < 0 || count > MAX_STUDENTS) {
        printf("文件中的学生数量无效\n");
        fclose(file);
        return 0;  // 失败
    }
    
    // 清空当前的学生数据
    clear_manager(manager);
    
    // 读取学生信息
    for (int i = 0; i < count; i++) {
        if (fread(&manager->students[i], sizeof(Student), 1, file) != 1) {
            printf("读取学生数据时出错\n");
            fclose(file);
            return 0;  // 失败
        }
        manager->count++;
    }
    
    fclose(file);
    printf("成功从文件 '%s' 加载%d名学生的数据\n", filename, manager->count);
    
    return 1;  // 成功
}

// 清空学生管理系统
void clear_manager(StudentManager *manager) {
    if (manager == NULL) {
        return;
    }
    
    manager->count = 0;
    memset(manager->students, 0, sizeof(manager->students));
}

// 显示菜单
void display_menu() {
    printf("\n学生管理系统菜单：\n");
    printf("1. 添加学生\n");
    printf("2. 查找学生\n");
    printf("3. 更新学生\n");
    printf("4. 删除学生\n");
    printf("5. 显示所有学生\n");
    printf("6. 按平均分排序\n");
    printf("7. 保存数据到文件\n");
    printf("8. 从文件加载数据\n");
    printf("9. 清空所有数据\n");
    printf("0. 退出系统\n");
    printf("请选择操作（0-9）：");
}

// 处理用户输入
void handle_user_input(StudentManager *manager) {
    if (manager == NULL) {
        return;
    }
    
    int choice;
    char id[MAX_ID_LENGTH];
    char filename[100];
    Student student;
    
    while (1) {
        display_menu();
        
        // 读取用户选择
        if (scanf("%d", &choice) != 1) {
            // 清除输入缓冲区
            while (getchar() != '\n');
            printf("无效的输入，请输入一个数字\n");
            continue;
        }
        
        // 清除输入缓冲区中的换行符
        while (getchar() != '\n');
        
        switch (choice) {
            case 0:
                // 退出系统
                return;
                
            case 1:
                // 添加学生
                printf("\n添加新学生：\n");
                
                // 输入并验证学号
                do {
                    printf("请输入学号：");
                    fgets(id, sizeof(id), stdin);
                    id[strcspn(id, "\n")] = '\0';  // 移除换行符
                } while (!validate_id(id));
                
                strcpy(student.id, id);
                
                // 输入并验证姓名
                do {
                    printf("请输入姓名：");
                    fgets(student.name, sizeof(student.name), stdin);
                    student.name[strcspn(student.name, "\n")] = '\0';  // 移除换行符
                } while (!validate_name(student.name));
                
                // 输入并验证年龄
                do {
                    printf("请输入年龄：");
                    scanf("%d", &student.age);
                    while (getchar() != '\n');  // 清除输入缓冲区
                } while (!validate_age(student.age));
                
                // 输入并验证成绩
                for (int j = 0; j < COURSE_COUNT; j++) {
                    do {
                        printf("请输入课程%d成绩：", j+1);
                        scanf("%f", &student.scores[j]);
                        while (getchar() != '\n');  // 清除输入缓冲区
                    } while (!validate_score(student.scores[j]));
                }
                
                // 添加学生
                if (add_student(manager, &student)) {
                    printf("成功添加学生 '%s'\n", student.name);
                }
                break;
                
            case 2:
                // 查找学生
                printf("\n查找学生：\n");
                printf("请输入学号：");
                fgets(id, sizeof(id), stdin);
                id[strcspn(id, "\n")] = '\0';  // 移除换行符
                
                Student *found = find_student_by_id(manager, id);
                if (found != NULL) {
                    print_student(found);
                } else {
                    printf("未找到学号为 '%s' 的学生\n", id);
                }
                break;
                
            case 3:
                // 更新学生
                printf("\n更新学生：\n");
                printf("请输入要更新的学生学号：");
                fgets(id, sizeof(id), stdin);
                id[strcspn(id, "\n")] = '\0';  // 移除换行符
                
                if (find_student_by_id(manager, id) == NULL) {
                    printf("未找到学号为 '%s' 的学生\n", id);
                    break;
                }
                
                // 输入新的学生信息
                printf("请输入新的学生信息：\n");
                
                // 注意：不更新学号
                do {
                    printf("请输入姓名：");
                    fgets(student.name, sizeof(student.name), stdin);
                    student.name[strcspn(student.name, "\n")] = '\0';  // 移除换行符
                } while (!validate_name(student.name));
                
                do {
                    printf("请输入年龄：");
                    scanf("%d", &student.age);
                    while (getchar() != '\n');  // 清除输入缓冲区
                } while (!validate_age(student.age));
                
                for (int j = 0; j < COURSE_COUNT; j++) {
                    do {
                        printf("请输入课程%d成绩：", j+1);
                        scanf("%f", &student.scores[j]);
                        while (getchar() != '\n');  // 清除输入缓冲区
                    } while (!validate_score(student.scores[j]));
                }
                
                // 更新学生信息
                update_student(manager, id, &student);
                printf("成功更新学号为 '%s' 的学生信息\n", id);
                break;
                
            case 4:
                // 删除学生
                printf("\n删除学生：\n");
                printf("请输入要删除的学生学号：");
                fgets(id, sizeof(id), stdin);
                id[strcspn(id, "\n")] = '\0';  // 移除换行符
                
                if (delete_student_by_id(manager, id)) {
                    printf("成功删除学号为 '%s' 的学生\n", id);
                }
                break;
                
            case 5:
                // 显示所有学生
                print_all_students(manager);
                break;
                
            case 6:
                // 按平均分排序
                sort_students_by_average(manager);
                printf("学生已按平均分从高到低排序\n");
                print_all_students(manager);
                break;
                
            case 7:
                // 保存数据到文件
                printf("\n保存数据到文件：\n");
                printf("请输入文件名：");
                fgets(filename, sizeof(filename), stdin);
                filename[strcspn(filename, "\n")] = '\0';  // 移除换行符
                
                save_to_file(manager, filename);
                break;
                
            case 8:
                // 从文件加载数据
                printf("\n从文件加载数据：\n");
                printf("请输入文件名：");
                fgets(filename, sizeof(filename), stdin);
                filename[strcspn(filename, "\n")] = '\0';  // 移除换行符
                
                load_from_file(manager, filename);
                break;
                
            case 9:
                // 清空所有数据
                printf("\n确认要清空所有数据吗？(y/n)：");
                char confirm;
                scanf(" %c", &confirm);
                while (getchar() != '\n');  // 清除输入缓冲区
                
                if (confirm == 'y' || confirm == 'Y') {
                    clear_manager(manager);
                    printf("所有学生数据已清空\n");
                }
                break;
                
            default:
                printf("无效的选择，请重新输入\n");
                break;
        }
    }
}

// 验证学号
int validate_id(const char *id) {
    if (id == NULL || strlen(id) == 0) {
        printf("错误：学号不能为空\n");
        return 0;
    }
    
    // 简单验证：只允许字母和数字
    for (int i = 0; id[i] != '\0'; i++) {
        if (!isalnum((unsigned char)id[i])) {
            printf("错误：学号只能包含字母和数字\n");
            return 0;
        }
    }
    
    return 1;
}

// 验证姓名
int validate_name(const char *name) {
    if (name == NULL || strlen(name) == 0) {
        printf("错误：姓名不能为空\n");
        return 0;
    }
    
    return 1;
}

// 验证年龄
int validate_age(int age) {
    if (age < 0 || age > 150) {
        printf("错误：年龄必须在0到150之间\n");
        return 0;
    }
    
    return 1;
}

// 验证成绩
int validate_score(float score) {
    if (score < 0 || score > 100) {
        printf("错误：成绩必须在0到100之间\n");
        return 0;
    }
    
    return 1;
}

// 生成示例数据
void generate_sample_data(StudentManager *manager) {
    if (manager == NULL) {
        return;
    }
    
    // 预定义的示例学生数据
    const char *sample_ids[] = {"S1001", "S1002", "S1003", "S1004", "S1005"};
    const char *sample_names[] = {"张三", "李四", "王五", "赵六", "钱七"};
    const int sample_ages[] = {20, 21, 19, 22, 20};
    const float sample_scores[][COURSE_COUNT] = {
        {85.5, 90.0, 88.0},
        {78.0, 82.5, 80.0},
        {92.0, 88.5, 90.5},
        {65.5, 70.0, 68.0},
        {88.0, 92.0, 94.0}
    };
    
    // 添加示例学生
    int count = sizeof(sample_ids) / sizeof(sample_ids[0]);
    
    for (int i = 0; i < count && manager->count < MAX_STUDENTS; i++) {
        Student student;
        strcpy(student.id, sample_ids[i]);
        strcpy(student.name, sample_names[i]);
        student.age = sample_ages[i];
        memcpy(student.scores, sample_scores[i], sizeof(student.scores));
        
        add_student(manager, &student);
    }
    
    printf("已生成%d条示例学生数据\n", manager->count);
}