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
use crate::{pojo::department_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for department-related database operations
#[async_trait::async_trait]
pub trait DepartmentMapperTrait {
    async fn list(&self, condition: DepartmentCondition) -> Result<Vec<DepartmentVo>, DbErr>;
    async fn page(&self, condition: DepartmentCondition) -> Result<PageData<DepartmentVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<DepartmentVo>, DbErr>;
    async fn save(&self, department_dto: DepartmentDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, department_dto: DepartmentDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, department_dto: DepartmentDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, department_dto: DepartmentDto) -> Result<u64, DbErr>;
}

/// Implementation of DepartmentMapperTrait
pub struct DepartmentMapper {
    state: Arc<AppState>,
}

impl DepartmentMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static DepartmentMapper {
        static INSTANCE: OnceCell<DepartmentMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| DepartmentMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &DepartmentCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(department::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(department::Column::Id.is_in(ids.clone()));
        };
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(department::Column::Code.eq(code));
        };
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(department::Column::ParentCode.eq(parent_code));
        };
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(department::Column::Name.like(format!("%{}%", name)));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(department::Column::OrgCode.eq(org_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &DepartmentCondition,
        list: Vec<DepartmentVo>,
        total: u64,
    ) -> Result<PageData<DepartmentVo>, DbErr> {
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
impl DepartmentMapperTrait for DepartmentMapper {
    async fn list(&self, condition: DepartmentCondition) -> Result<Vec<DepartmentVo>, DbErr> {
        let department = Department::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<DepartmentVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(department)
    }

    async fn page(&self, condition: DepartmentCondition) -> Result<PageData<DepartmentVo>, DbErr> {
        let department = Department::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<DepartmentVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = Department::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, department, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<DepartmentVo>, DbErr> {
        let department_opt = Department::find_by_id(rec_id)
            .into_model::<DepartmentVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(department_opt)
    }

    async fn save(&self, department_dto: DepartmentDto) -> Result<i64, DbErr> {
        let department_dtoc = department_dto.into_json_with_snake_key();
        info!("department_json is {:?}", department_dtoc);
        let mut department_actmod = department::ActiveModel::from_json(department_dtoc)?;
        department_actmod.set(department::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        department_actmod.set(department::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = Department::insert(department_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, department_dto: DepartmentDto) -> Result<u64, DbErr> {
        let department_dto_json = department_dto.into_json_with_snake_key();
        info!("department_json is {:?}", department_dto);
        let department_actmod = department::ActiveModel::from_json(department_dto_json)?;
        let update_result = Department::update_many()
            .set(department_actmod)
            .filter(department::Column::Id.eq(department_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, department_dto: DepartmentDto) -> Result<u64, DbErr> {
        info!("department_json is {:?}", department_dto);
        let update_result = Department::update_many()
            .col_expr(department::Column::IsDel, Expr::value(-1))
            .filter(department::Column::Id.is_in(department_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, department_dto: DepartmentDto) -> Result<u64, DbErr> {
        info!("department_json is {:?}", department_dto);
        let update_result = Department::delete_many()
            .filter(department::Column::Id.is_in(department_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 