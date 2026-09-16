/// 通用 Mapper Trait 宏
/// 自动生成 Mapper Trait 定义。
/// 加了 `Send + Sync` 约束，因为 Service 侧以 `Arc<dyn XxxMapperTrait>` 持有它（依赖注入）。
#[macro_export]
macro_rules! define_mapper_trait {
    ($trait_name:ident, $condition:ty, $vo:ty, $dto:ty) => {
        #[async_trait::async_trait]
        pub trait $trait_name: Send + Sync {
            async fn list(&self, condition: $condition) -> Result<Vec<$vo>, sea_orm::DbErr>;
            async fn page(&self, condition: $condition) -> Result<$crate::util::paged_struct::PageData<$vo>, sea_orm::DbErr>;
            async fn get_by_id(&self, rec_id: i64) -> Result<Option<$vo>, sea_orm::DbErr>;
            async fn save(&self, dto: $dto) -> Result<i64, sea_orm::DbErr>;
            async fn update_by_id(&self, dto: $dto) -> Result<u64, sea_orm::DbErr>;
            async fn delete_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr>;
            async fn remove_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr>;
        }
    };
}

/// 通用 Mapper 结构体和基础方法宏
///
/// 只持有连接池：mapper 不再依赖 `AppState`，因此可以脱离应用容器单独构造，
/// 测试里也能自己造一个（此前 `get_instance` 用 `OnceCell` 把实例焊成进程级单例，
/// 首个 state 获胜 —— 既换不掉也测不了）。
#[macro_export]
macro_rules! define_mapper_struct {
    ($mapper_name:ident) => {
        pub struct $mapper_name {
            pool: sea_orm::DatabaseConnection,
        }

        impl $mapper_name {
            pub fn new(pool: sea_orm::DatabaseConnection) -> Self {
                Self { pool }
            }
        }
    };
}

/// 通用 Mapper 实现宏
/// 自动生成 CRUD 方法实现
#[macro_export]
macro_rules! impl_mapper {
    (
        $mapper_name:ident,
        $mapper_trait:ident,
        $entity:ident,
        $entity_mod:ident,
        $condition:ty,
        $vo:ty,
        $dto:ty
    ) => {
        #[async_trait::async_trait]
        impl $mapper_trait for $mapper_name {
            /// 按条件查询，不分页，返回全部匹配记录。
            /// condition 里的 page/size 在此不生效。
            /// 单次最多 MAX_LIST_SIZE 条，超出会报错而不是静默截断，需要更多数据请用 page()。
            async fn list(&self, condition: $condition) -> Result<Vec<$vo>, sea_orm::DbErr> {
                use sea_orm::{EntityTrait, QueryFilter, QuerySelect};
                use $crate::util::paged_struct::MAX_LIST_SIZE;

                // 多取一行用于探测是否超限，避免额外一次 COUNT
                let result = $entity::find()
                    .filter(self.build_query_wrapper(&condition))
                    .limit(MAX_LIST_SIZE + 1)
                    .into_model::<$vo>()
                    .all(&self.pool)
                    .await?;
                if result.len() as u64 > MAX_LIST_SIZE {
                    return Err(sea_orm::DbErr::Custom(format!(
                        "list 查询结果超过上限 {} 条，请改用分页接口 page",
                        MAX_LIST_SIZE
                    )));
                }
                Ok(result)
            }

            /// 分页查询，condition 里的 page/size 生效，缺省为第 1 页、每页 DEFAULT_PAGE_SIZE 条。
            async fn page(&self, condition: $condition) -> Result<$crate::util::paged_struct::PageData<$vo>, sea_orm::DbErr> {
                use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryTrait};
                use $crate::util::paged_struct::{PageData, PageInfo, Pageable, DEFAULT_PAGE_SIZE};

                let list = $entity::find()
                    .filter(self.build_query_wrapper(&condition))
                    .apply_if(condition.get_size(), QuerySelect::limit)
                    .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
                    .into_model::<$vo>()
                    .all(&self.pool)
                    .await?;
                let total = $entity::find()
                    .filter(self.build_query_wrapper(&condition))
                    .count(&self.pool)
                    .await?;
                let page_info = PageInfo::from(
                    condition.get_page().unwrap_or(1),
                    condition.get_size().unwrap_or(DEFAULT_PAGE_SIZE),
                    total,
                );
                Ok(PageData::new(page_info, list))
            }

            async fn get_by_id(&self, rec_id: i64) -> Result<Option<$vo>, sea_orm::DbErr> {
                use sea_orm::EntityTrait;

                let result = $entity::find_by_id(rec_id)
                    .into_model::<$vo>()
                    .one(&self.pool)
                    .await?;
                Ok(result)
            }

            /// 只记实体名与主键，不再打印整个 DTO —— DTO 里可能有密码等敏感字段。
            async fn save(&self, dto: $dto) -> Result<i64, sea_orm::DbErr> {
                use sea_orm::{ActiveModelTrait, EntityTrait};
                use $crate::util::IntoJsonValue;
                use tracing::info;

                let dto_json = dto.into_json_with_snake_key();
                info!("{} 新增", stringify!($entity_mod));
                let mut active_model = $entity_mod::ActiveModel::from_json(dto_json)?;
                active_model.set($entity_mod::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
                active_model.set($entity_mod::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
                let result = $entity::insert(active_model).exec(&self.pool).await?;
                Ok(result.last_insert_id)
            }

            async fn update_by_id(&self, dto: $dto) -> Result<u64, sea_orm::DbErr> {
                use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
                use $crate::util::IntoJsonValue;
                use tracing::info;

                let dto_json = dto.into_json_with_snake_key();
                info!("{} 更新, rec_id={:?}", stringify!($entity_mod), &dto.rec_id);
                let active_model = $entity_mod::ActiveModel::from_json(dto_json)?;
                let result = $entity::update_many()
                    .set(active_model)
                    .filter($entity_mod::Column::Id.eq(dto.rec_id))
                    .exec(&self.pool)
                    .await?;
                Ok(result.rows_affected)
            }

            async fn delete_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr> {
                use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
                use sea_orm::prelude::Expr;
                use tracing::info;

                info!("{} 软删除, rec_ids={:?}", stringify!($entity_mod), &dto.rec_ids);
                let result = $entity::update_many()
                    .col_expr($entity_mod::Column::IsDel, Expr::value(-1))
                    .filter($entity_mod::Column::Id.is_in(dto.rec_ids.unwrap_or_default()))
                    .exec(&self.pool)
                    .await?;
                Ok(result.rows_affected)
            }

            async fn remove_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr> {
                use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
                use tracing::info;

                info!("{} 物理删除, rec_ids={:?}", stringify!($entity_mod), &dto.rec_ids);
                let result = $entity::delete_many()
                    .filter($entity_mod::Column::Id.is_in(dto.rec_ids.unwrap_or_default()))
                    .exec(&self.pool)
                    .await?;
                Ok(result.rows_affected)
            }
        }
    };
}

/// 通用 Service 宏
///
/// 生成 Service 结构体与 CRUD 方法，并在关键节点调用 `ServiceHooks`。
/// Service 以 `Arc<dyn XxxMapperTrait>` 持有依赖 —— 可注入、可替换、可单测。
///
/// 尾部可选接一个钩子块，用来承载业务逻辑；不写块则全部落到 `ServiceHooks` 的默认空实现：
///
/// ```ignore
/// // 无业务逻辑
/// crate::impl_service!(RoleSvc, RoleMapperTrait, RoleCondition, RoleVo, RoleDto);
///
/// // 有业务逻辑：覆盖需要的钩子即可，其余仍走默认实现
/// crate::impl_service!(UserSvc, UserMapperTrait, UserCondition, UserVo, UserDto, {
///     async fn before_save(&self, mut dto: UserDto) -> Result<UserDto, sea_orm::DbErr> {
///         dto.password = Some(crate::auth::password::hash_password(...)?);
///         Ok(dto)
///     }
/// });
/// ```
#[macro_export]
macro_rules! impl_service {
    // 带钩子块
    (
        $svc_name:ident,
        $mapper_trait:ident,
        $condition:ty,
        $vo:ty,
        $dto:ty,
        { $($hook:item)* }
    ) => {
        $crate::impl_service!(@gen $svc_name, $mapper_trait, $condition, $vo, $dto, { $($hook)* });
    };

    // 不带钩子块
    (
        $svc_name:ident,
        $mapper_trait:ident,
        $condition:ty,
        $vo:ty,
        $dto:ty
    ) => {
        $crate::impl_service!(@gen $svc_name, $mapper_trait, $condition, $vo, $dto, {});
    };

    (@gen $svc_name:ident, $mapper_trait:ident, $condition:ty, $vo:ty, $dto:ty, { $($hook:item)* }) => {
        use std::sync::Arc;
        use sea_orm::DbErr;
        use $crate::util::paged_struct::PageData;

        pub struct $svc_name {
            mapper: Arc<dyn $mapper_trait>,
        }

        impl $svc_name {
            pub fn new(mapper: Arc<dyn $mapper_trait>) -> Self {
                Self { mapper }
            }

            pub async fn list(&self, condition: $condition) -> Result<Vec<$vo>, DbErr> {
                self.mapper.list(condition).await
            }

            pub async fn page(&self, condition: $condition) -> Result<PageData<$vo>, DbErr> {
                self.mapper.page(condition).await
            }

            pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<$vo>, DbErr> {
                self.mapper.get_by_id(rec_id).await
            }

            pub async fn save(&self, dto: $dto) -> Result<i64, DbErr> {
                let dto = <Self as $crate::hooks::ServiceHooks<$dto>>::before_save(self, dto).await?;
                let rec_id = self.mapper.save(dto.clone()).await?;
                <Self as $crate::hooks::ServiceHooks<$dto>>::after_save(self, rec_id, &dto).await?;
                Ok(rec_id)
            }

            pub async fn update_by_id(&self, dto: $dto) -> Result<u64, DbErr> {
                let dto = <Self as $crate::hooks::ServiceHooks<$dto>>::before_update(self, dto).await?;
                self.mapper.update_by_id(dto).await
            }

            pub async fn delete_by_ids(&self, dto: $dto) -> Result<u64, DbErr> {
                <Self as $crate::hooks::ServiceHooks<$dto>>::before_delete(self, &dto).await?;
                self.mapper.delete_by_ids(dto).await
            }

            pub async fn remove_by_ids(&self, dto: $dto) -> Result<u64, DbErr> {
                <Self as $crate::hooks::ServiceHooks<$dto>>::before_delete(self, &dto).await?;
                self.mapper.remove_by_ids(dto).await
            }
        }

        #[async_trait::async_trait]
        impl $crate::hooks::ServiceHooks<$dto> for $svc_name {
            $($hook)*
        }
    };
}

/// 通用 Controller 宏
///
/// `$mapper_field` 是 `Mappers` 里该资源对应的字段名（如 `user`、`user_role_ref`）：
/// handler 从 `AppState` 里取出对应 mapper 再组装 Service —— 这就是注入点，
/// 换成假 mapper 即可脱离数据库测 handler。
#[macro_export]
macro_rules! impl_controller {
    (
        $ctl_name:ident,
        $svc_name:ident,
        $mapper_field:ident,
        $condition:ty,
        $vo:ty,
        $dto:ty
    ) => {
        use std::sync::Arc;
        use axum::{
            extract::{Path, Query, State},
            Json,
        };
        use $crate::{
            util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
            AppState, ResultJson,
        };

        pub struct $ctl_name;

        impl $ctl_name {
            pub async fn list(
                Query(condition): Query<$condition>,
                State(state): State<Arc<AppState>>,
            ) -> ResultJson<Vec<$vo>> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .list(condition)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn page(
                Query(condition): Query<$condition>,
                State(state): State<Arc<AppState>>,
            ) -> ResultJson<PageData<$vo>> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .page(condition)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn get_by_id(
                Path(rec_id): Path<i64>,
                State(state): State<Arc<AppState>>,
            ) -> ResultJson<Option<$vo>> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .get_by_id(rec_id)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn save(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<i64> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .save(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn update_by_id(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<u64> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .update_by_id(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn delete_by_ids(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<u64> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .delete_by_ids(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn remove_by_ids(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<u64> {
                let result = $svc_name::new(state.mappers.$mapper_field.clone())
                    .remove_by_ids(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }
        }
    };
}

// ==================== Pojo 辅助宏 ====================

/// DTO JSON 转换宏
/// 自动生成 IntoJsonValue 实现：把 camelCase 序列化结果的 key 转成 snake_case，
/// 供 sea_orm 的 ActiveModel::from_json 直接消费。
#[macro_export]
macro_rules! impl_into_json_value {
    ($dto:ty) => {
        impl $crate::util::IntoJsonValue for $dto {
            fn into_json_with_snake_key(&self) -> serde_json::Value {
                let mut json_object = serde_json::Map::new();
                let json_value = serde_json::json!(self);
                if let Some(obj_map) = json_value.as_object() {
                    for (k, v) in obj_map {
                        json_object.insert(
                            $crate::util::common_func::camel_case_to_under_score(k),
                            v.clone(),
                        );
                    }
                }
                serde_json::Value::Object(json_object)
            }
        }
    };
}

/// 分页条件宏
/// 自动生成 Pageable 实现，要求 Condition 上有 `page: Option<u64>` 与 `size: Option<u64>`。
#[macro_export]
macro_rules! impl_pageable {
    ($condition:ty) => {
        impl $crate::util::paged_struct::Pageable for $condition {
            fn get_page(&self) -> Option<u64> {
                self.page.or(Some(1))
            }
            fn get_size(&self) -> Option<u64> {
                self.size.or(Some($crate::util::paged_struct::DEFAULT_PAGE_SIZE))
            }
        }
    };
}
