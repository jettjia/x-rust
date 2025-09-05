#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <time.h>

/*
 * 16_multi_threading.c - C语言多线程编程示例
 * 本示例介绍使用POSIX线程库（pthread）进行多线程编程
 */

// 全局变量，所有线程共享
int global_counter = 0;

// 线程同步的互斥锁
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

// 线程同步的条件变量
pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
int ready = 0;  // 条件变量的谓词

// 线程函数的参数结构体
typedef struct {
    int thread_id;
    int iterations;
    char *message;
} ThreadArgs;

// 简单的线程函数，打印信息
void *simple_thread_function(void *arg) {
    int thread_id = *((int *)arg);
    
    printf("线程 %d: 开始执行\n", thread_id);
    
    // 模拟一些工作
    sleep(1);
    
    printf("线程 %d: 执行完毕\n", thread_id);
    
    pthread_exit(NULL);
}

// 带参数的线程函数
void *thread_with_args(void *arg) {
    ThreadArgs *args = (ThreadArgs *)arg;
    
    printf("线程 %d: %s\n", args->thread_id, args->message);
    
    // 模拟一些工作
    for (int i = 0; i < args->iterations; i++) {
        printf("线程 %d: 迭代 %d\n", args->thread_id, i+1);
        sleep(0.5);
    }
    
    // 释放参数内存
    free(arg);
    
    pthread_exit(NULL);
}

// 访问共享资源的线程函数
void *shared_resource_thread(void *arg) {
    int thread_id = *((int *)arg);
    
    for (int i = 0; i < 5; i++) {
        // 加锁，保护共享资源
        pthread_mutex_lock(&mutex);
        
        // 访问共享资源
        global_counter++;
        printf("线程 %d: 增加全局计数器到 %d\n", thread_id, global_counter);
        
        // 解锁
        pthread_mutex_unlock(&mutex);
        
        // 让出一些时间给其他线程
        usleep(100000);  // 100毫秒
    }
    
    pthread_exit(NULL);
}

// 生产者线程函数
void *producer_thread(void *arg) {
    printf("生产者线程: 开始执行\n");
    
    // 模拟生产过程
    sleep(2);
    
    // 加锁
    pthread_mutex_lock(&mutex);
    
    // 设置条件为真
    ready = 1;
    printf("生产者线程: 数据已准备好\n");
    
    // 通知等待的消费者线程
    pthread_cond_signal(&cond);
    
    // 解锁
    pthread_mutex_unlock(&mutex);
    
    pthread_exit(NULL);
}

// 消费者线程函数
void *consumer_thread(void *arg) {
    printf("消费者线程: 开始执行\n");
    
    // 加锁
    pthread_mutex_lock(&mutex);
    
    // 等待条件为真
    while (!ready) {
        printf("消费者线程: 等待数据准备...\n");
        pthread_cond_wait(&cond, &mutex);
    }
    
    // 此时条件为真，消费数据
    printf("消费者线程: 开始消费数据\n");
    
    // 解锁
    pthread_mutex_unlock(&mutex);
    
    pthread_exit(NULL);
}

// 线程取消示例函数
void *cancellable_thread(void *arg) {
    // 设置线程可以被取消
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, NULL);
    // 设置取消类型为延迟取消
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED, NULL);
    
    printf("可取消线程: 开始执行\n");
    
    // 模拟长时间运行的任务
    for (int i = 0; i < 10; i++) {
        printf("可取消线程: 工作中... %d\n", i+1);
        
        // 取消点
        pthread_testcancel();
        
        sleep(1);
    }
    
    printf("可取消线程: 执行完毕\n");
    
    pthread_exit(NULL);
}

// 线程分离示例函数
void *detached_thread(void *arg) {
    printf("分离线程: 开始执行\n");
    
    // 模拟一些工作
    sleep(2);
    
    printf("分离线程: 执行完毕\n");
    
    // 不需要pthread_join，线程结束后自动释放资源
    pthread_exit(NULL);
}

// 线程局部存储示例
__thread int thread_local_var = 0;  // 线程局部变量

void *thread_local_storage(void *arg) {
    int thread_id = *((int *)arg);
    
    // 每个线程都有自己的thread_local_var副本
    thread_local_var = thread_id * 10;
    printf("线程 %d: 线程局部变量初始值 = %d\n", thread_id, thread_local_var);
    
    // 修改线程局部变量
    thread_local_var++;
    printf("线程 %d: 线程局部变量修改后 = %d\n", thread_id, thread_local_var);
    
    pthread_exit(NULL);
}

int main() {
    pthread_t threads[10];  // 线程ID数组
    int thread_ids[10];     // 线程ID参数
    void *status;           // 线程返回状态
    
    printf("C语言多线程编程示例：\n\n");
    
    // 1. 简单的多线程创建和等待
    printf("1. 简单的多线程创建和等待：\n");
    
    // 创建两个线程
    for (int i = 0; i < 2; i++) {
        thread_ids[i] = i+1;
        int rc = pthread_create(&threads[i], NULL, simple_thread_function, &thread_ids[i]);
        
        if (rc) {
            printf("错误：无法创建线程，返回代码 %d\n", rc);
            exit(-1);
        }
    }
    
    // 等待线程完成
    for (int i = 0; i < 2; i++) {
        pthread_join(threads[i], &status);
    }
    
    // 2. 向线程传递参数
    printf("\n2. 向线程传递参数：\n");
    
    // 创建两个带参数的线程
    for (int i = 0; i < 2; i++) {
        // 为每个线程分配参数内存
        ThreadArgs *args = (ThreadArgs *)malloc(sizeof(ThreadArgs));
        if (args == NULL) {
            perror("内存分配失败");
            exit(-1);
        }
        
        args->thread_id = i+1;
        args->iterations = 3;
        args->message = (i == 0) ? "这是第一个带参数的线程" : "这是第二个带参数的线程";
        
        int rc = pthread_create(&threads[i], NULL, thread_with_args, args);
        
        if (rc) {
            printf("错误：无法创建线程，返回代码 %d\n", rc);
            exit(-1);
        }
    }
    
    // 等待线程完成
    for (int i = 0; i < 2; i++) {
        pthread_join(threads[i], &status);
    }
    
    // 3. 线程同步（互斥锁）
    printf("\n3. 线程同步（互斥锁）：\n");
    printf("初始全局计数器: %d\n", global_counter);
    
    // 创建两个访问共享资源的线程
    for (int i = 0; i < 2; i++) {
        thread_ids[i] = i+1;
        int rc = pthread_create(&threads[i], NULL, shared_resource_thread, &thread_ids[i]);
        
        if (rc) {
            printf("错误：无法创建线程，返回代码 %d\n", rc);
            exit(-1);
        }
    }
    
    // 等待线程完成
    for (int i = 0; i < 2; i++) {
        pthread_join(threads[i], &status);
    }
    
    printf("最终全局计数器: %d\n", global_counter);
    
    // 4. 条件变量
    printf("\n4. 条件变量：\n");
    
    // 创建生产者和消费者线程
    pthread_create(&threads[0], NULL, producer_thread, NULL);
    pthread_create(&threads[1], NULL, consumer_thread, NULL);
    
    // 等待线程完成
    pthread_join(threads[0], &status);
    pthread_join(threads[1], &status);
    
    // 5. 线程取消
    printf("\n5. 线程取消：\n");
    
    // 创建可取消的线程
    pthread_create(&threads[0], NULL, cancellable_thread, NULL);
    
    // 等待一段时间后取消线程
    sleep(3);
    printf("主线程: 取消工作线程\n");
    pthread_cancel(threads[0]);
    
    // 等待线程结束
    pthread_join(threads[0], &status);
    if (status == PTHREAD_CANCELED) {
        printf("主线程: 线程已被取消\n");
    }
    
    // 6. 线程分离
    printf("\n6. 线程分离：\n");
    
    // 创建线程属性
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    
    // 创建分离线程
    pthread_create(&threads[0], &attr, detached_thread, NULL);
    
    // 销毁线程属性
    pthread_attr_destroy(&attr);
    
    // 等待一段时间，确保分离线程有足够的时间执行
    sleep(3);
    printf("主线程: 确认分离线程已执行完毕\n");
    
    // 7. 线程局部存储
    printf("\n7. 线程局部存储：\n");
    
    // 创建两个使用线程局部存储的线程
    for (int i = 0; i < 2; i++) {
        thread_ids[i] = i+1;
        int rc = pthread_create(&threads[i], NULL, thread_local_storage, &thread_ids[i]);
        
        if (rc) {
            printf("错误：无法创建线程，返回代码 %d\n", rc);
            exit(-1);
        }
    }
    
    // 等待线程完成
    for (int i = 0; i < 2; i++) {
        pthread_join(threads[i], &status);
    }
    
    // 8. 多线程编程最佳实践
    printf("\n8. 多线程编程最佳实践：\n");
    printf("- 尽量减少共享数据的使用\n");
    printf("- 使用适当的同步机制保护共享数据\n");
    printf("- 避免死锁（确保锁的获取顺序一致）\n");
    printf("- 避免长时间持有锁\n");
    printf("- 正确处理线程的创建和销毁\n");
    printf("- 为长时间运行的线程提供取消机制\n");
    printf("- 使用线程局部存储存储线程特定的数据\n");
    printf("- 考虑使用线程池管理线程资源\n");
    
    // 清理资源
    pthread_mutex_destroy(&mutex);
    pthread_cond_destroy(&cond);
    
    printf("\n多线程编程示例完成！\n");
    
    return 0;
}