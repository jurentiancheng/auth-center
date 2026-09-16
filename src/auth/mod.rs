//! 认证：登录、JWT 签发校验、鉴权中间件。
//!
//! 这里是**手写**模块，不走 CRUD 宏 —— 认证是领域服务而非通用增删改查；
//! 同时也示范了宏生成模块与手写模块如何共存（宏范式必须留出逃生舱）。

pub mod ctl;
pub mod extractor;
pub mod jwt;
pub mod mapper;
pub mod middleware;
pub mod password;
pub mod svc;
