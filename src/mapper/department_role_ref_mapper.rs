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
use crate::{pojo::department_role_ref_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for department role ref-related database operations
#[async_trait::async_trait]
pub trait DepartmentRoleRefMapperTrait {
    async fn list(&self, condition: DepartmentRoleRefCondition) -> Result<Vec<DepartmentRoleRefVo>, DbErr>;
    async fn page(&self, condition: DepartmentRoleRefCondition) -> Result<PageData<DepartmentRoleRefVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<DepartmentRoleRefVo>, DbErr>;
    async fn save(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr>;
}

/// Implementation of DepartmentRoleRefMapperTrait
pub struct DepartmentRoleRefMapper {
    state: Arc<AppState>,
}

impl DepartmentRoleRefMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static DepartmentRoleRefMapper {
        static INSTANCE: OnceCell<DepartmentRoleRefMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| DepartmentRoleRefMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &DepartmentRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(department_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(department_role_ref::Column::Id.is_in(ids.clone()));
        };
        if let Some(department_code) = &condition.department_code {
            query_wrapper = query_wrapper.add(department_role_ref::Column::DepartmentCode.eq(department_code));
        };
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(department_role_ref::Column::RoleCode.eq(role_code));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(department_role_ref::Column::OrgCode.eq(org_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &DepartmentRoleRefCondition,
        list: Vec<DepartmentRoleRefVo>,
        total: u64,
    ) -> Result<PageData<DepartmentRoleRefVo>, DbErr> {
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
impl DepartmentRoleRefMapperTrait for DepartmentRoleRefMapper {
    async fn list(&self, condition: DepartmentRoleRefCondition) -> Result<Vec<DepartmentRoleRefVo>, DbErr> {
        let department_role_ref = DepartmentRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<DepartmentRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(department_role_ref)
    }

    async fn page(&self, condition: DepartmentRoleRefCondition) -> Result<PageData<DepartmentRoleRefVo>, DbErr> {
        let department_role_ref = DepartmentRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<DepartmentRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = DepartmentRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, department_role_ref, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<DepartmentRoleRefVo>, DbErr> {
        let department_role_ref_opt = DepartmentRoleRef::find_by_id(rec_id)
            .into_model::<DepartmentRoleRefVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(department_role_ref_opt)
    }

    async fn save(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<i64, DbErr> {
        let department_role_ref_dtoc = department_role_ref_dto.into_json_with_snake_key();
        info!("department_role_ref_json is {:?}", department_role_ref_dtoc);
        let mut department_role_ref_actmod = department_role_ref::ActiveModel::from_json(department_role_ref_dtoc)?;
        department_role_ref_actmod.set(department_role_ref::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        department_role_ref_actmod.set(department_role_ref::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = DepartmentRoleRef::insert(department_role_ref_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr> {
        let department_role_ref_dto_json = department_role_ref_dto.into_json_with_snake_key();
        info!("department_role_ref_json is {:?}", department_role_ref_dto);
        let department_role_ref_actmod = department_role_ref::ActiveModel::from_json(department_role_ref_dto_json)?;
        let update_result = DepartmentRoleRef::update_many()
            .set(department_role_ref_actmod)
            .filter(department_role_ref::Column::Id.eq(department_role_ref_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr> {
        info!("department_role_ref_json is {:?}", department_role_ref_dto);
        let update_result = DepartmentRoleRef::update_many()
            .col_expr(department_role_ref::Column::IsDel, Expr::value(-1))
            .filter(department_role_ref::Column::Id.is_in(department_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr> {
        info!("department_role_ref_json is {:?}", department_role_ref_dto);
        let update_result = DepartmentRoleRef::delete_many()
            .filter(department_role_ref::Column::Id.is_in(department_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 