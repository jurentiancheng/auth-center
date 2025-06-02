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
use crate::{pojo::position_role_ref_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for position role ref-related database operations
#[async_trait::async_trait]
pub trait PositionRoleRefMapperTrait {
    async fn list(&self, condition: PositionRoleRefCondition) -> Result<Vec<PositionRoleRefVo>, DbErr>;
    async fn page(&self, condition: PositionRoleRefCondition) -> Result<PageData<PositionRoleRefVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionRoleRefVo>, DbErr>;
    async fn save(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr>;
}

/// Implementation of PositionRoleRefMapperTrait
pub struct PositionRoleRefMapper {
    state: Arc<AppState>,
}

impl PositionRoleRefMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static PositionRoleRefMapper {
        static INSTANCE: OnceCell<PositionRoleRefMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| PositionRoleRefMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &PositionRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(position_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(position_role_ref::Column::Id.is_in(ids.clone()));
        };
        if let Some(position_code) = &condition.position_code {
            query_wrapper = query_wrapper.add(position_role_ref::Column::PositionCode.eq(position_code));
        };
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(position_role_ref::Column::RoleCode.eq(role_code));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(position_role_ref::Column::OrgCode.eq(org_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &PositionRoleRefCondition,
        list: Vec<PositionRoleRefVo>,
        total: u64,
    ) -> Result<PageData<PositionRoleRefVo>, DbErr> {
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
impl PositionRoleRefMapperTrait for PositionRoleRefMapper {
    async fn list(&self, condition: PositionRoleRefCondition) -> Result<Vec<PositionRoleRefVo>, DbErr> {
        let position_role_ref = PositionRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<PositionRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(position_role_ref)
    }

    async fn page(&self, condition: PositionRoleRefCondition) -> Result<PageData<PositionRoleRefVo>, DbErr> {
        let position_role_ref = PositionRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<PositionRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = PositionRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, position_role_ref, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionRoleRefVo>, DbErr> {
        let position_role_ref_opt = PositionRoleRef::find_by_id(rec_id)
            .into_model::<PositionRoleRefVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(position_role_ref_opt)
    }

    async fn save(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<i64, DbErr> {
        let position_role_ref_dtoc = position_role_ref_dto.into_json_with_snake_key();
        info!("position_role_ref_json is {:?}", position_role_ref_dtoc);
        let mut position_role_ref_actmod = position_role_ref::ActiveModel::from_json(position_role_ref_dtoc)?;
        position_role_ref_actmod.set(position_role_ref::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        position_role_ref_actmod.set(position_role_ref::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = PositionRoleRef::insert(position_role_ref_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr> {
        let position_role_ref_dto_json = position_role_ref_dto.into_json_with_snake_key();
        info!("position_role_ref_json is {:?}", position_role_ref_dto);
        let position_role_ref_actmod = position_role_ref::ActiveModel::from_json(position_role_ref_dto_json)?;
        let update_result = PositionRoleRef::update_many()
            .set(position_role_ref_actmod)
            .filter(position_role_ref::Column::Id.eq(position_role_ref_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr> {
        info!("position_role_ref_json is {:?}", position_role_ref_dto);
        let update_result = PositionRoleRef::update_many()
            .col_expr(position_role_ref::Column::IsDel, Expr::value(-1))
            .filter(position_role_ref::Column::Id.is_in(position_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr> {
        info!("position_role_ref_json is {:?}", position_role_ref_dto);
        let update_result = PositionRoleRef::delete_many()
            .filter(position_role_ref::Column::Id.is_in(position_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 