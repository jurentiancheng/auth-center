//! Service 层业务钩子。
//!
//! 宏生成的 Service 方法原本是「一展开就定死」的纯转发，任何资源要加业务逻辑
//! 就只能删掉宏调用、手写整个 impl。这里给宏留出扩展点：`impl_service!` 会在生成的
//! `impl ServiceHooks<Dto> for XxxSvc` 块里把调用方写的钩子拼进去。
//!
//! 默认实现放在 trait 上（而不是让宏生成方法体），所以：
//! - 不写钩子块的资源，全部落到这里的默认空实现，行为与从前一致；
//! - 需要的资源可以只覆盖其中一两个，不必写全套。
//!
//! 用法见 `src/svc/user_svc.rs`。

use sea_orm::DbErr;

/// `Dto` 需要 `Send + 'static`：钩子是 async 的，参数会被移进 `#[async_trait]` 装箱的
/// future，而该 future 必须能跨线程调度。项目里 16 个 DTO 都满足。
#[async_trait::async_trait]
pub trait ServiceHooks<Dto>: Send + Sync
where
    Dto: Send + 'static,
{
    /// 入库前改写 DTO。返回的 DTO 才是真正写库的那个 —— 例如在这里把明文密码换成哈希。
    async fn before_save(&self, dto: Dto) -> Result<Dto, DbErr> {
        Ok(dto)
    }

    /// 入库后拿到自增主键。用于写关联表、发消息这类需要主键的副作用。
    async fn after_save(&self, _rec_id: i64, _dto: &Dto) -> Result<(), DbErr> {
        Ok(())
    }

    /// 更新前改写 DTO。
    async fn before_update(&self, dto: Dto) -> Result<Dto, DbErr> {
        Ok(dto)
    }

    /// 软删除 / 物理删除前。用于校验可否删除、级联清理。
    async fn before_delete(&self, _dto: &Dto) -> Result<(), DbErr> {
        Ok(())
    }
}
