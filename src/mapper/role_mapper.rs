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
use crate::{pojo::role_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for role-related database operations
#[async_trait::async_trait]
pub trait RoleMapperTrait {
    async fn list(&self, condition: RoleCondition) -> Result<Vec<RoleVo>, DbErr>;
    async fn page(&self, condition: RoleCondition) -> Result<PageData<RoleVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<RoleVo>, DbErr>;
    async fn save(&self, role_dto: RoleDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, role_dto: RoleDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, role_dto: RoleDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, role_dto: RoleDto) -> Result<u64, DbErr>;
}

/// Implementation of RoleMapperTrait
pub struct RoleMapper {
    state: Arc<AppState>,
}

impl RoleMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static RoleMapper {
        static INSTANCE: OnceCell<RoleMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| RoleMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &RoleCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(role::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(role::Column::Id.is_in(ids.clone()));
        };
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(role::Column::Code.eq(code));
        };
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(role::Column::ParentCode.eq(parent_code));
        };
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(role::Column::Name.eq(name));
        };
        if let Some(application) = &condition.application {
            query_wrapper = query_wrapper.add(role::Column::Application.eq(application));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(role::Column::OrgCode.eq(org_code));
        };
        if let Some(role_type) = &condition.role_type {
            query_wrapper = query_wrapper.add(role::Column::RoleType.eq(*role_type));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &RoleCondition,
        list: Vec<RoleVo>,
        total: u64,
    ) -> Result<PageData<RoleVo>, DbErr> {
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
impl RoleMapperTrait for RoleMapper {
    async fn list(&self, condition: RoleCondition) -> Result<Vec<RoleVo>, DbErr> {
        let role = Role::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<RoleVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(role)
    }

    async fn page(&self, condition: RoleCondition) -> Result<PageData<RoleVo>, DbErr> {
        let role = Role::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<RoleVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = Role::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, role, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<RoleVo>, DbErr> {
        let role_opt = Role::find_by_id(rec_id)
            .into_model::<RoleVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(role_opt)
    }

    async fn save(&self, role_dto: RoleDto) -> Result<i64, DbErr> {
        let role_dtoc = role_dto.into_json_with_snake_key();
        info!("role_json is {:?}", role_dtoc);
        let mut role_actmod = role::ActiveModel::from_json(role_dtoc)?;
        role_actmod.set(role::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        role_actmod.set(role::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = Role::insert(role_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, role_dto: RoleDto) -> Result<u64, DbErr> {
        let role_dto_json = role_dto.into_json_with_snake_key();
        info!("role_json is {:?}", role_dto);
        let role_actmod = role::ActiveModel::from_json(role_dto_json)?;
        let update_result = Role::update_many()
            .set(role_actmod)
            .filter(role::Column::Id.eq(role_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, role_dto: RoleDto) -> Result<u64, DbErr> {
        info!("role_json is {:?}", role_dto);
        let update_result = Role::update_many()
            .col_expr(role::Column::IsDel, Expr::value(-1))
            .filter(role::Column::Id.is_in(role_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, role_dto: RoleDto) -> Result<u64, DbErr> {
        info!("role_json is {:?}", role_dto);
        let update_result = Role::delete_many()
            .filter(role::Column::Id.is_in(role_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 