# Auth Center

一个基于 Rust 和 Axum 框架开发的认证中心服务。

## 项目结构

```
auth-center/
├── src/
│   ├── main.rs          # 应用程序入口点
│   ├── lib.rs           # 库文件，包含核心功能
│   ├── util.rs          # 工具函数和辅助方法
│   ├── ctl/             # 控制器层，处理 HTTP 请求
│   ├── entities/        # 数据库实体定义
│   ├── mapper/          # 数据映射层
│   ├── pojo/            # 数据传输对象（DTO）
│   └── svc/             # 服务层，包含业务逻辑
├── Cargo.toml           # 项目依赖和配置
├── Cargo.lock           # 依赖版本锁定文件
└── Dockerfile           # Docker 构建文件
```

## 模块说明

### 核心模块
- `main.rs`: 应用程序的入口点，负责启动服务器和初始化配置
- `lib.rs`: 核心库文件，包含主要的业务逻辑和功能实现
- `util.rs`: 通用工具函数集合，提供各种辅助功能

### 分层架构
- `ctl/`: 控制器层，处理 HTTP 请求和响应，实现 API 接口
- `entities/`: 数据库实体定义，使用 Sea-ORM 框架
- `mapper/`: 数据映射层，处理数据库操作和对象映射
- `pojo/`: 数据传输对象，定义 API 请求和响应的数据结构
- `svc/`: 服务层，实现核心业务逻辑

## 技术栈

- **Web 框架**: Axum
- **数据库 ORM**: Sea-ORM
- **异步运行时**: Tokio
- **序列化**: Serde
- **日志**: Tracing
- **加密**: OpenSSL
- **时间处理**: Chrono
- **错误处理**: Anyhow

## 构建和运行

1. 确保已安装 Rust 工具链
2. 克隆项目
3. 运行 `cargo build` 构建项目
4. 运行 `cargo run` 启动服务

## Docker 支持

项目包含 Dockerfile，支持容器化部署：

```bash
docker build -t auth-center .
docker run -p 18080:18080 auth-center
``` 