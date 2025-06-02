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
use crate::{pojo::organization_role_ref_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for organization role ref-related database operations
#[async_trait::async_trait]
pub trait OrganizationRoleRefMapperTrait {
    async fn list(&self, condition: OrganizationRoleRefCondition) -> Result<Vec<OrganizationRoleRefVo>, DbErr>;
    async fn page(&self, condition: OrganizationRoleRefCondition) -> Result<PageData<OrganizationRoleRefVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<OrganizationRoleRefVo>, DbErr>;
    async fn save(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr>;
}

/// Implementation of OrganizationRoleRefMapperTrait
pub struct OrganizationRoleRefMapper {
    state: Arc<AppState>,
}

impl OrganizationRoleRefMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static OrganizationRoleRefMapper {
        static INSTANCE: OnceCell<OrganizationRoleRefMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| OrganizationRoleRefMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &OrganizationRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(organization_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(organization_role_ref::Column::Id.is_in(ids.clone()));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(organization_role_ref::Column::OrgCode.eq(org_code));
        };
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(organization_role_ref::Column::RoleCode.eq(role_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &OrganizationRoleRefCondition,
        list: Vec<OrganizationRoleRefVo>,
        total: u64,
    ) -> Result<PageData<OrganizationRoleRefVo>, DbErr> {
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
impl OrganizationRoleRefMapperTrait for OrganizationRoleRefMapper {
    async fn list(&self, condition: OrganizationRoleRefCondition) -> Result<Vec<OrganizationRoleRefVo>, DbErr> {
        let organization_role_ref = OrganizationRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<OrganizationRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(organization_role_ref)
    }

    async fn page(&self, condition: OrganizationRoleRefCondition) -> Result<PageData<OrganizationRoleRefVo>, DbErr> {
        let organization_role_ref = OrganizationRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<OrganizationRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = OrganizationRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, organization_role_ref, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<OrganizationRoleRefVo>, DbErr> {
        let organization_role_ref_opt = OrganizationRoleRef::find_by_id(rec_id)
            .into_model::<OrganizationRoleRefVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(organization_role_ref_opt)
    }

    async fn save(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<i64, DbErr> {
        let organization_role_ref_dtoc = organization_role_ref_dto.into_json_with_snake_key();
        info!("organization_role_ref_json is {:?}", organization_role_ref_dtoc);
        let mut organization_role_ref_actmod = organization_role_ref::ActiveModel::from_json(organization_role_ref_dtoc)?;
        organization_role_ref_actmod.set(organization_role_ref::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        organization_role_ref_actmod.set(organization_role_ref::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = OrganizationRoleRef::insert(organization_role_ref_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr> {
        let organization_role_ref_dto_json = organization_role_ref_dto.into_json_with_snake_key();
        info!("organization_role_ref_json is {:?}", organization_role_ref_dto);
        let organization_role_ref_actmod = organization_role_ref::ActiveModel::from_json(organization_role_ref_dto_json)?;
        let update_result = OrganizationRoleRef::update_many()
            .set(organization_role_ref_actmod)
            .filter(organization_role_ref::Column::Id.eq(organization_role_ref_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr> {
        info!("organization_role_ref_json is {:?}", organization_role_ref_dto);
        let update_result = OrganizationRoleRef::update_many()
            .col_expr(organization_role_ref::Column::IsDel, Expr::value(-1))
            .filter(organization_role_ref::Column::Id.is_in(organization_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr> {
        info!("organization_role_ref_json is {:?}", organization_role_ref_dto);
        let update_result = OrganizationRoleRef::delete_many()
            .filter(organization_role_ref::Column::Id.is_in(organization_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 