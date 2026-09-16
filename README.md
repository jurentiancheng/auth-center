# Auth Center

一个基于 Rust 和 Axum 框架开发的认证中心服务，提供用户管理、权限控制和组织架构管理等核心功能。

## 项目结构说明

本项目采用清晰的模块化结构，主要包含以下目录：

### 核心目录结构
```
auth-center/
├── src/
│   ├── main.rs          # 应用程序入口点
│   ├── lib.rs           # 核心库文件，AppState 与启动初始化
│   ├── macros.rs        # 四层代码生成宏（含业务钩子扩展点）
│   ├── config.rs        # 配置收口：环境变量读取与校验
│   ├── hooks.rs         # Service 层业务钩子 trait
│   ├── route.rs         # 路由配置，含鉴权保护层与白名单
│   ├── util.rs          # 工具类，包含通用的辅助函数和工具方法
│   ├── auth/            # 认证：登录、JWT、密码哈希、鉴权中间件、CurrentUser 提取器
│   ├── ctl/             # 控制器层，处理HTTP请求和响应
│   ├── entities/        # 数据库实体层，定义数据模型
│   ├── mapper/          # 数据访问层，实现数据库操作 + Mappers 依赖装配
│   ├── pojo/            # 数据传输对象，定义API请求/响应结构
│   └── svc/             # 服务层，实现核心业务逻辑
├── tests/               # 集成测试（不需要数据库）
├── Cargo.toml           # 项目依赖和配置
├── Cargo.lock           # 依赖版本锁定文件
├── .env.example         # 环境变量清单
├── init.sql             # 建库建表脚本
└── Dockerfile           # Docker构建配置文件
```

### 分层架构说明

#### 控制器层 (`src/ctl`)
- 处理HTTP请求和响应
- 实现RESTful API接口
- 验证请求参数并返回适当的响应格式
- 调用服务层处理业务逻辑

#### 数据库实体层 (`src/entities`)
- 定义与数据库表结构对应的实体类
- 使用SeaORM框架进行数据库映射
- 包含表结构定义和关系配置

#### 数据访问层 (`src/mapper`)
- 包含数据库操作的具体实现
- 提供CRUD操作和其他自定义查询方法
- 将数据库记录映射为实体对象

#### 数据传输对象 (`src/pojo`)
- 定义API请求和响应的数据结构
- 包含数据验证规则
- 用于不同层之间安全地传递数据

#### 服务层 (`src/svc`)
- 实现核心业务逻辑
- 调用mapper层进行数据持久化操作
- 处理跨多个实体的复杂业务规则
- 向控制器层提供业务功能接口

#### 核心模块
- `main.rs`: 应用程序入口，负责初始化日志与启动服务器
- `lib.rs`: `AppState`（配置 + mapper 容器 + JWT 服务）与启动初始化
- `config.rs`: 所有环境变量的唯一读取点，启动时校验
- `macros.rs`: 生成四层样板代码的宏，`impl_service!` 尾部可接业务钩子块
- `hooks.rs`: `ServiceHooks` trait，宏生成的 Service 在各关键节点调用它
- `route.rs`: 定义所有API路由，并把需要登录的接口挂到鉴权保护层下
- `auth/`: 认证模块（手写，不走宏）：登录、JWT 签发校验、argon2 密码哈希、鉴权中间件、`CurrentUser` 提取器
- `util.rs`: 通用工具函数集合（驼峰/下划线转换、统一响应体、分页结构、日期序列化）

> 注意：加密与 JWT 相关逻辑在 `src/auth/`，不在 `util.rs`。

## 主要功能模块

### 用户管理
- 用户注册与登录
- 用户信息管理（创建、读取、更新、删除）
- 角色分配与管理
- 组管理及成员维护

### 权限管理
- 角色定义与管理
- 权限分配与继承
- 基于角色的访问控制(RBAC)
- 操作审计与日志记录

### 组织架构
- 部门结构管理
- 职位定义与分配
- 组织单元层级管理
- 组织与用户关联

### 系统配置
- 全局参数配置
- 认证策略配置
- 安全策略设置
- 第三方集成配置

## 环境变量

全部由 `src/config.rs` 收口读取并在启动时校验，清单见 `.env.example`。

| 变量 | 必填 | 默认 | 说明 |
|---|---|---|---|
| `JWT_SECRET` | ✅ | 无 | HS256 签名密钥，至少 32 字节。生成：`openssl rand -base64 48` |
| `DATABASE_URL` | | `mysql://root:root@127.0.0.1:18306/auth_center` | |
| `LISTEN_ADDR` | | `0.0.0.0:18080` | |
| `JWT_EXPIRE_SECONDS` | | `7200` | 令牌有效期（秒） |
| `DB_MAX_CONNECTIONS` | | `100` | 连接池上限 |
| `DB_MIN_CONNECTIONS` | | `5` | 连接池下限 |
| `DB_CONNECT_TIMEOUT_SECS` | | `20` | 连接超时 |
| `RUST_LOG` | | `info` | 日志级别，如 `RUST_LOG=debug` |

**`JWT_SECRET` 故意不给默认值** —— 认证服务带着一个默认签名密钥跑起来，比启动失败危险得多，所以缺失时直接启动报错并给出生成命令。

```bash
cp .env.example .env
# 填好 JWT_SECRET 与 DATABASE_URL
cargo run
```

## 认证

```bash
# 登录（白名单，无需令牌）
curl -X POST localhost:18080/auth/login \
     -H 'Content-Type: application/json' \
     -d '{"userName":"admin","password":"..."}'
# => {"code":0,"data":{"token":"...","tokenType":"Bearer","expiresIn":7200,...}}

# 带令牌访问（除 / 与 /auth/login 外，所有接口都需要）
curl localhost:18080/auth/me -H 'Authorization: Bearer <token>'
curl localhost:18080/user/list -H 'Authorization: Bearer <token>'
```

- 未带或带无效令牌 → **401**，且 HTTP 状态码与响应体 `code` 一致（都是 401）。
- 业务 handler 直接把 `CurrentUser` 写进参数即可拿到当前登录用户，见 `src/auth/extractor.rs`。
- 密码以 **argon2id** 哈希入库（PHC 字符串自带随机盐），`UserSvc` 的钩子在落库前完成哈希。
- 登录失败统一返回「用户名或密码错误」，不区分用户不存在 / 密码错 / 账号禁用，避免登录接口变成用户名枚举器。
- token 载荷只含鉴权必需字段（`sub`/`userName`/`isAdmin`/`iat`/`exp`）—— token 是客户端可读的，不放敏感信息。

> 目前只做**认证**（是否已登录），未做**授权**（这个用户能否访问这个接口）。RBAC 相关表已齐备，但 `user` 与 `user_info.user_code` 之间没有可用的关联约定，授权链路还建不起来。

## 宏与业务钩子

`src/macros.rs` 把四层样板代码压到每个资源 2~7 行。**宏生成的代码不是死的**：`impl_service!` 尾部可接一个钩子块承载业务逻辑，覆盖 `ServiceHooks`（`src/hooks.rs`）中的钩子，没覆盖的落到默认空实现 —— 所以不需要逻辑的资源一行都不用多写。

```rust
// 无业务逻辑：与从前完全一致
crate::impl_service!(RoleSvc, RoleMapperTrait, RoleCondition, RoleVo, RoleDto);

// 有业务逻辑：尾部追加钩子块（真实用例见 src/svc/user_svc.rs 的密码哈希）
crate::impl_service!(UserSvc, UserMapperTrait, UserCondition, UserVo, UserDto, {
    async fn before_save(&self, mut dto: UserDto) -> Result<UserDto, sea_orm::DbErr> {
        dto.password = Some(hash_password(&dto.password)?);
        Ok(dto)
    }
});
```

依赖统一由 `AppState.mappers`（`src/mapper/mod.rs` 的 `Mappers` 容器）注入，各层只依赖 trait，因此可替换、可单测。认证这类领域服务不走宏，直接手写在 `src/auth/` —— 宏与手写两种模块共存。

## 测试

```bash
cargo test
```

全部测试**不需要数据库**：JWT 签发/验签、篡改与过期令牌被拒、argon2 哈希与校验、`Authorization` 头解析、401 响应一致性，以及用假 mapper 注入 Service 验证钩子确实生效（`tests/service_di.rs`）。

## 技术栈

- **编程语言**: Rust (1.60+)
- **Web框架**: Axum - 异步、模块化的Web框架
- **数据库 ORM**: Sea-ORM - 强类型的ORM框架
- **异步运行时**: Tokio - 高性能异步执行环境
- **序列化/反序列化**: Serde - 高效的数据转换库
- **日志系统**: Tracing - 结构化事件跟踪框架
- **密码哈希**: Argon2 (argon2id) - PHC 字符串格式，自带随机盐
- **令牌**: jsonwebtoken - JWT 签发与校验（HS256，`rust_crypto` 纯 Rust 后端）
- **加密库**: OpenSSL - 交叉编译时提供系统 TLS
- **时间处理**: Chrono - 日期时间操作库
- **错误处理**: Anyhow - 简化的错误类型封装
- **数据库**: MySQL
- **容器化**: Docker/Docker Compose - 环境隔离和部署

## 开发环境要求

- Rust 1.60+ 已安装Rust工具链
- Cargo 包管理器
- Docker 和 Docker Compose（用于容器化部署）
- PostgreSQL 或 MySQL 开发环境
- Git 版本控制系统

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

## 开发规范

- 所有代码遵循Rust官方编码规范
- 使用Tracing进行结构化日志记录
- 所有外部API使用Serde进行数据序列化/反序列化
- 数据库操作全部通过Sea-ORM进行
- 错误处理统一使用Anyhow包装
- 安全相关操作使用加密库保证安全性