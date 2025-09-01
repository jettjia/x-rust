# Axum + SQLx + PostgreSQL 用户CRUD案例

一个使用Rust、Axum、SQLx和PostgreSQL实现的完整用户增删改查(CRUD)Web应用。

## 项目结构

```
src/
├── main.rs          # 应用入口点
├── db.rs            # 数据库连接和迁移
├── model.rs         # 数据模型和存储实现
├── handler.rs       # HTTP请求处理函数
└── router.rs        # 路由配置
.env                 # 环境变量配置
Cargo.toml           # 项目依赖配置
.gitignore           # Git忽略规则
```

## 技术栈

- **框架**: Axum
- **ORM**: SQLx
- **数据库**: PostgreSQL
- **运行时**: Tokio
- **序列化**: Serde
- **错误处理**: thiserror
- **日志**: tracing

## 功能特性

- 用户创建
- 用户列表查询
- 用户详情查询
- 用户信息更新
- 用户删除
- 数据库迁移自动执行
- 优雅关闭

## 准备工作

1. 确保已安装Rust和Cargo
2. 确保已安装PostgreSQL
3. 创建一个名为`user_crud`的数据库
4. 根据需要修改`.env`文件中的数据库连接配置

## 运行项目

```bash
# 安装依赖
cargo install --path .

# 运行项目
cargo run
```

服务器将在 http://127.0.0.1:3000 启动

## API接口

### 用户接口

- **创建用户**: POST /users
- **获取所有用户**: GET /users
- **获取单个用户**: GET /users/:id
- **更新用户**: PUT /users/:id
- **删除用户**: DELETE /users/:id

## 示例请求

### 创建用户

```bash
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "张三", "email": "zhangsan@example.com", "password": "password123"}'
```

### 获取所有用户

```bash
curl http://127.0.0.1:3000/users
```

### 获取单个用户

```bash
curl http://127.0.0.1:3000/users/{user_id}
```

### 更新用户

```bash
curl -X PUT http://127.0.0.1:3000/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{"name": "李四", "email": "lisi@example.com"}'
```

### 删除用户

```bash
curl -X DELETE http://127.0.0.1:3000/users/{user_id}
```