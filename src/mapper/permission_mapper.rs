use anyhow::Result;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Cond;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
    QueryTrait,
};
use std::sync::Arc;
use tracing::info;
use once_cell::sync::OnceCell;

use crate::entities::{prelude::*, *};
use crate::util::paged_struct::{PageData, PageInfo, Pageable};
use crate::util::IntoJsonValue;
use crate::{pojo::permission_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for permission-related database operations
#[async_trait::async_trait]
pub trait PermissionMapperTrait {
    async fn list(&self, condition: PermissionCondition) -> Result<Vec<PermissionVo>, DbErr>;
    async fn page(&self, condition: PermissionCondition) -> Result<PageData<PermissionVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<PermissionVo>, DbErr>;
    async fn save(&self, permission_dto: PermissionDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, permission_dto: PermissionDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, permission_dto: PermissionDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, permission_dto: PermissionDto) -> Result<u64, DbErr>;
}

/// Implementation of PermissionMapperTrait
pub struct PermissionMapper {
    state: Arc<AppState>,
}

impl PermissionMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static PermissionMapper {
        static INSTANCE: OnceCell<PermissionMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| PermissionMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &PermissionCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(permission::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(permission::Column::Id.is_in(ids.clone()));
        };
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(permission::Column::Code.eq(code));
        };
        if let Some(parent_uuid) = &condition.parent_uuid {
            query_wrapper = query_wrapper.add(permission::Column::ParentUuid.eq(parent_uuid));
        };
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(permission::Column::Name.eq(name));
        };
        if let Some(application) = &condition.application {
            query_wrapper = query_wrapper.add(permission::Column::Application.eq(application));
        };
        
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &PermissionCondition,
        list: Vec<PermissionVo>,
        total: u64,
    ) -> Result<PageData<PermissionVo>, DbErr> {
        let page_info = PageInfo::from(
            condition.get_page().unwrap(),
            condition.get_size().unwrap(),
            total,
        );
        let page_data = PageData::new(page_info, list);
        Ok(page_data)
    }
}

#[async_trait::async_trait]
impl PermissionMapperTrait for PermissionMapper {
    async fn list(&self, condition: PermissionCondition) -> Result<Vec<PermissionVo>, DbErr> {
        let permission = Permission::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<PermissionVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(permission)
    }

    async fn page(&self, condition: PermissionCondition) -> Result<PageData<PermissionVo>, DbErr> {
        let permission = Permission::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<PermissionVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = Permission::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, permission, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<PermissionVo>, DbErr> {
        let permission_opt = Permission::find_by_id(rec_id)
            .into_model::<PermissionVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(permission_opt)
    }

    async fn save(&self, permission_dto: PermissionDto) -> Result<i64, DbErr> {
        let permission_dtoc = permission_dto.into_json_with_snake_key();
        info!("permission_json is {:?}", permission_dtoc);
        let mut permission_actmod = permission::ActiveModel::from_json(permission_dtoc)?;
        permission_actmod.set(permission::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        permission_actmod.set(permission::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = Permission::insert(permission_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, permission_dto: PermissionDto) -> Result<u64, DbErr> {
        let permission_dto_json = permission_dto.into_json_with_snake_key();
        info!("permission_json is {:?}", permission_dto);
        let permission_actmod = permission::ActiveModel::from_json(permission_dto_json)?;
        let update_result = Permission::update_many()
            .set(permission_actmod)
            .filter(permission::Column::Id.eq(permission_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, permission_dto: PermissionDto) -> Result<u64, DbErr> {
        info!("permission_json is {:?}", permission_dto);
        let update_result = Permission::update_many()
            .col_expr(permission::Column::IsDel, Expr::value(-1))
            .filter(permission::Column::Id.is_in(permission_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, permission_dto: PermissionDto) -> Result<u64, DbErr> {
        info!("permission_json is {:?}", permission_dto);
        let update_result = Permission::delete_many()
            .filter(permission::Column::Id.is_in(permission_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 