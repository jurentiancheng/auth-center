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
use crate::{pojo::position_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for position-related database operations
#[async_trait::async_trait]
pub trait PositionMapperTrait {
    async fn list(&self, condition: PositionCondition) -> Result<Vec<PositionVo>, DbErr>;
    async fn page(&self, condition: PositionCondition) -> Result<PageData<PositionVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionVo>, DbErr>;
    async fn save(&self, position_dto: PositionDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, position_dto: PositionDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, position_dto: PositionDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, position_dto: PositionDto) -> Result<u64, DbErr>;
}

/// Implementation of PositionMapperTrait
pub struct PositionMapper {
    state: Arc<AppState>,
}

impl PositionMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static PositionMapper {
        static INSTANCE: OnceCell<PositionMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| PositionMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &PositionCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(position::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(position::Column::Id.is_in(ids.clone()));
        };
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(position::Column::Code.eq(code));
        };
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(position::Column::Name.eq(name));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(position::Column::OrgCode.eq(org_code));
        };

        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &PositionCondition,
        list: Vec<PositionVo>,
        total: u64,
    ) -> Result<PageData<PositionVo>, DbErr> {
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
impl PositionMapperTrait for PositionMapper {
    async fn list(&self, condition: PositionCondition) -> Result<Vec<PositionVo>, DbErr> {
        let position = Position::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<PositionVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(position)
    }

    async fn page(&self, condition: PositionCondition) -> Result<PageData<PositionVo>, DbErr> {
        let position = Position::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<PositionVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = Position::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, position, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionVo>, DbErr> {
        let position_opt = Position::find_by_id(rec_id)
            .into_model::<PositionVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(position_opt)
    }

    async fn save(&self, position_dto: PositionDto) -> Result<i64, DbErr> {
        let position_dtoc = position_dto.into_json_with_snake_key();
        info!("position_json is {:?}", position_dtoc);
        let mut position_actmod = position::ActiveModel::from_json(position_dtoc)?;
        position_actmod.set(position::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        position_actmod.set(position::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = Position::insert(position_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, position_dto: PositionDto) -> Result<u64, DbErr> {
        let position_dto_json = position_dto.into_json_with_snake_key();
        info!("position_json is {:?}", position_dto);
        let position_actmod = position::ActiveModel::from_json(position_dto_json)?;
        let update_result = Position::update_many()
            .set(position_actmod)
            .filter(position::Column::Id.eq(position_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, position_dto: PositionDto) -> Result<u64, DbErr> {
        info!("position_json is {:?}", position_dto);
        let update_result = Position::update_many()
            .col_expr(position::Column::IsDel, Expr::value(-1))
            .filter(position::Column::Id.is_in(position_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, position_dto: PositionDto) -> Result<u64, DbErr> {
        info!("position_json is {:?}", position_dto);
        let update_result = Position::delete_many()
            .filter(position::Column::Id.is_in(position_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 