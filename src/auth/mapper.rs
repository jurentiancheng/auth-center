//! 认证相关的数据访问：手写，不走 CRUD 宏。
//!
//! 宏生成的是通用增删改查；这里是领域查询（按用户名取用户、写最后登录时间），
//! 也是「宏范式」必须留出的逃生舱。

use async_trait::async_trait;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use crate::entities::{prelude::*, user};
use crate::pojo::user_pojo::UserVo;

#[async_trait]
pub trait AuthMapperTrait: Send + Sync {
    /// 按用户名取用户（不含已软删除的）。
    ///
    /// 命中多条时报错而不是随便返回一条 —— 用户名不唯一意味着同名用户可以互相登录，
    /// 这是认证缺陷而不是数据瑕疵（`init.sql` 已给 `user_name` 加唯一索引）。
    async fn find_by_user_name(&self, user_name: &str) -> Result<Option<UserVo>, sea_orm::DbErr>;

    /// 写最后登录时间。该字段此前从未被任何代码写入过。
    async fn touch_last_login(&self, user_id: i64) -> Result<u64, sea_orm::DbErr>;
}

pub struct AuthMapper {
    pool: DatabaseConnection,
}

impl AuthMapper {
    pub fn new(pool: DatabaseConnection) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthMapperTrait for AuthMapper {
    async fn find_by_user_name(&self, user_name: &str) -> Result<Option<UserVo>, sea_orm::DbErr> {
        // 多取一行用于探测重名，不必额外一次 COUNT
        let mut rows = User::find()
            .filter(user::Column::UserName.eq(user_name))
            .filter(user::Column::IsDel.eq(0))
            .limit(2)
            .into_model::<UserVo>()
            .all(&self.pool)
            .await?;

        match rows.len() {
            0 => Ok(None),
            1 => Ok(Some(rows.remove(0))),
            _ => Err(sea_orm::DbErr::Custom(format!(
                "用户名 {user_name} 命中多条 user 记录，请先给 user.user_name 加唯一索引并清理数据"
            ))),
        }
    }

    async fn touch_last_login(&self, user_id: i64) -> Result<u64, sea_orm::DbErr> {
        let now = chrono::Local::now().fixed_offset();
        let result = User::update_many()
            .col_expr(user::Column::LastLoginTime, Expr::value(now))
            .filter(user::Column::Id.eq(user_id))
            .exec(&self.pool)
            .await?;
        Ok(result.rows_affected)
    }
}
