/// 通用 Mapper Trait 宏
/// 自动生成 Mapper Trait 定义
#[macro_export]
macro_rules! define_mapper_trait {
    ($trait_name:ident, $condition:ty, $vo:ty, $dto:ty) => {
        #[async_trait::async_trait]
        pub trait $trait_name {
            async fn list(&self, condition: $condition) -> Result<Vec<$vo>, sea_orm::DbErr>;
            async fn page(&self, condition: $condition) -> Result<crate::util::paged_struct::PageData<$vo>, sea_orm::DbErr>;
            async fn get_by_id(&self, rec_id: i64) -> Result<Option<$vo>, sea_orm::DbErr>;
            async fn save(&self, dto: $dto) -> Result<i64, sea_orm::DbErr>;
            async fn update_by_id(&self, dto: $dto) -> Result<u64, sea_orm::DbErr>;
            async fn delete_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr>;
            async fn remove_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr>;
        }
    };
}

/// 通用 Mapper 结构体和基础方法宏
#[macro_export]
macro_rules! define_mapper_struct {
    ($mapper_name:ident) => {
        pub struct $mapper_name {
            state: std::sync::Arc<crate::AppState>,
        }

        impl $mapper_name {
            pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
                Self { state }
            }

            pub fn get_instance(state: std::sync::Arc<crate::AppState>) -> &'static $mapper_name {
                static INSTANCE: once_cell::sync::OnceCell<$mapper_name> = once_cell::sync::OnceCell::new();
                INSTANCE.get_or_init(|| $mapper_name::new(state))
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
            async fn list(&self, condition: $condition) -> Result<Vec<$vo>, sea_orm::DbErr> {
                use sea_orm::{EntityTrait, QueryFilter, QuerySelect, QueryTrait};
                use crate::util::paged_struct::Pageable;

                let result = $entity::find()
                    .filter(self.build_query_wrapper(&condition))
                    .apply_if(condition.get_size(), QuerySelect::limit)
                    .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
                    .into_model::<$vo>()
                    .all(&self.state.mysql_pool)
                    .await?;
                Ok(result)
            }

            async fn page(&self, condition: $condition) -> Result<crate::util::paged_struct::PageData<$vo>, sea_orm::DbErr> {
                use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryTrait};
                use crate::util::paged_struct::{PageData, PageInfo, Pageable};

                let list = $entity::find()
                    .filter(self.build_query_wrapper(&condition))
                    .apply_if(condition.get_size(), QuerySelect::limit)
                    .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
                    .into_model::<$vo>()
                    .all(&self.state.mysql_pool)
                    .await?;
                let total = $entity::find()
                    .filter(self.build_query_wrapper(&condition))
                    .count(&self.state.mysql_pool)
                    .await?;
                let page_info = PageInfo::from(
                    condition.get_page().unwrap_or(1),
                    condition.get_size().unwrap_or(20),
                    total,
                );
                Ok(PageData::new(page_info, list))
            }

            async fn get_by_id(&self, rec_id: i64) -> Result<Option<$vo>, sea_orm::DbErr> {
                use sea_orm::EntityTrait;

                let result = $entity::find_by_id(rec_id)
                    .into_model::<$vo>()
                    .one(&self.state.mysql_pool)
                    .await?;
                Ok(result)
            }

            async fn save(&self, dto: $dto) -> Result<i64, sea_orm::DbErr> {
                use sea_orm::{ActiveModelTrait, EntityTrait};
                use crate::util::IntoJsonValue;
                use tracing::info;

                let dto_json = dto.into_json_with_snake_key();
                info!("{}_json is {:?}", stringify!($entity_mod), dto_json);
                let mut active_model = $entity_mod::ActiveModel::from_json(dto_json)?;
                active_model.set($entity_mod::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
                active_model.set($entity_mod::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
                let result = $entity::insert(active_model).exec(&self.state.mysql_pool).await?;
                Ok(result.last_insert_id)
            }

            async fn update_by_id(&self, dto: $dto) -> Result<u64, sea_orm::DbErr> {
                use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
                use crate::util::IntoJsonValue;
                use tracing::info;

                let dto_json = dto.into_json_with_snake_key();
                info!("{}_json is {:?}", stringify!($entity_mod), dto);
                let active_model = $entity_mod::ActiveModel::from_json(dto_json)?;
                let result = $entity::update_many()
                    .set(active_model)
                    .filter($entity_mod::Column::Id.eq(dto.rec_id))
                    .exec(&self.state.mysql_pool)
                    .await?;
                Ok(result.rows_affected)
            }

            async fn delete_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr> {
                use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
                use sea_orm::prelude::Expr;
                use tracing::info;

                info!("{}_json is {:?}", stringify!($entity_mod), dto);
                let result = $entity::update_many()
                    .col_expr($entity_mod::Column::IsDel, Expr::value(-1))
                    .filter($entity_mod::Column::Id.is_in(dto.rec_ids.unwrap_or_default()))
                    .exec(&self.state.mysql_pool)
                    .await?;
                Ok(result.rows_affected)
            }

            async fn remove_by_ids(&self, dto: $dto) -> Result<u64, sea_orm::DbErr> {
                use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
                use tracing::info;

                info!("{}_json is {:?}", stringify!($entity_mod), dto);
                let result = $entity::delete_many()
                    .filter($entity_mod::Column::Id.is_in(dto.rec_ids.unwrap_or_default()))
                    .exec(&self.state.mysql_pool)
                    .await?;
                Ok(result.rows_affected)
            }
        }
    };
}

/// 通用 Service 宏
/// 自动生成 Service 结构体和 CRUD 方法
#[macro_export]
macro_rules! impl_service {
    (
        $svc_name:ident,
        $mapper_name:ident,
        $mapper_trait:ident,
        $condition:ty,
        $vo:ty,
        $dto:ty
    ) => {
        use std::sync::Arc;
        use once_cell::sync::OnceCell;
        use sea_orm::DbErr;
        use crate::util::paged_struct::PageData;
        use crate::AppState;

        pub struct $svc_name {
            mapper: &'static $mapper_name,
        }

        impl $svc_name {
            pub fn new(state: &AppState) -> Self {
                Self {
                    mapper: $mapper_name::get_instance(Arc::new(state.clone())),
                }
            }

            pub fn get_instance(state: &AppState) -> &'static $svc_name {
                static INSTANCE: OnceCell<$svc_name> = OnceCell::new();
                INSTANCE.get_or_init(|| $svc_name::new(state))
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
                self.mapper.save(dto).await
            }

            pub async fn update_by_id(&self, dto: $dto) -> Result<u64, DbErr> {
                self.mapper.update_by_id(dto).await
            }

            pub async fn delete_by_ids(&self, dto: $dto) -> Result<u64, DbErr> {
                self.mapper.delete_by_ids(dto).await
            }

            pub async fn remove_by_ids(&self, dto: $dto) -> Result<u64, DbErr> {
                self.mapper.remove_by_ids(dto).await
            }
        }
    };
}

/// 通用 Controller 宏
/// 自动生成 Controller 结构体和 HTTP 处理方法
#[macro_export]
macro_rules! impl_controller {
    (
        $ctl_name:ident,
        $svc_name:ident,
        $condition:ty,
        $vo:ty,
        $dto:ty
    ) => {
        use std::sync::Arc;
        use axum::{
            extract::{Path, Query, State},
            Json,
        };
        use crate::{
            util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
            AppState, ResultJson,
        };

        pub struct $ctl_name;

        impl $ctl_name {
            pub async fn list(
                Query(condition): Query<$condition>,
                State(state): State<Arc<AppState>>,
            ) -> ResultJson<Vec<$vo>> {
                let result = $svc_name::get_instance(&state)
                    .list(condition)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn page(
                Query(condition): Query<$condition>,
                State(state): State<Arc<AppState>>,
            ) -> ResultJson<PageData<$vo>> {
                let result = $svc_name::get_instance(&state)
                    .page(condition)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn get_by_id(
                Path(rec_id): Path<i64>,
                State(state): State<Arc<AppState>>,
            ) -> ResultJson<Option<$vo>> {
                let result = $svc_name::get_instance(&state)
                    .get_by_id(rec_id)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn save(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<i64> {
                let result = $svc_name::get_instance(&state)
                    .save(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn update_by_id(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<u64> {
                let result = $svc_name::get_instance(&state)
                    .update_by_id(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn delete_by_ids(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<u64> {
                let result = $svc_name::get_instance(&state)
                    .delete_by_ids(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }

            pub async fn remove_by_ids(
                State(state): State<Arc<AppState>>,
                Json(dto): Json<$dto>,
            ) -> ResultJson<u64> {
                let result = $svc_name::get_instance(&state)
                    .remove_by_ids(dto)
                    .await
                    .map_err(internal_err)?;
                Ok(Json(RespResult::ok(result)))
            }
        }
    };
}
