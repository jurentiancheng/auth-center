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
use crate::{pojo::group_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for group-related database operations
#[async_trait::async_trait]
pub trait GroupMapperTrait {
    async fn list(&self, condition: GroupCondition) -> Result<Vec<GroupVo>, DbErr>;
    async fn page(&self, condition: GroupCondition) -> Result<PageData<GroupVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<GroupVo>, DbErr>;
    async fn save(&self, group_dto: GroupDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, group_dto: GroupDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, group_dto: GroupDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, group_dto: GroupDto) -> Result<u64, DbErr>;
}

/// Implementation of GroupMapperTrait
pub struct GroupMapper {
    state: Arc<AppState>,
}

impl GroupMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static GroupMapper {
        static INSTANCE: OnceCell<GroupMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| GroupMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &GroupCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(group::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(group::Column::Id.is_in(ids.clone()));
        };
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(group::Column::Code.eq(code));
        };
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(group::Column::Name.eq(name));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(group::Column::OrgCode.eq(org_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &GroupCondition,
        list: Vec<GroupVo>,
        total: u64,
    ) -> Result<PageData<GroupVo>, DbErr> {
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
impl GroupMapperTrait for GroupMapper {
    async fn list(&self, condition: GroupCondition) -> Result<Vec<GroupVo>, DbErr> {
        let group = Group::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<GroupVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(group)
    }

    async fn page(&self, condition: GroupCondition) -> Result<PageData<GroupVo>, DbErr> {
        let group = Group::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<GroupVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = Group::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, group, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<GroupVo>, DbErr> {
        let group_opt = Group::find_by_id(rec_id)
            .into_model::<GroupVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(group_opt)
    }

    async fn save(&self, group_dto: GroupDto) -> Result<i64, DbErr> {
        let group_dtoc = group_dto.into_json_with_snake_key();
        info!("group_json is {:?}", group_dtoc);
        let mut group_actmod = group::ActiveModel::from_json(group_dtoc)?;
        group_actmod.set(group::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        group_actmod.set(group::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = Group::insert(group_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, group_dto: GroupDto) -> Result<u64, DbErr> {
        let group_dto_json = group_dto.into_json_with_snake_key();
        info!("group_json is {:?}", group_dto);
        let group_actmod = group::ActiveModel::from_json(group_dto_json)?;
        let update_result = Group::update_many()
            .set(group_actmod)
            .filter(group::Column::Id.eq(group_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, group_dto: GroupDto) -> Result<u64, DbErr> {
        info!("group_json is {:?}", group_dto);
        let update_result = Group::update_many()
            .col_expr(group::Column::IsDel, Expr::value(-1))
            .filter(group::Column::Id.is_in(group_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, group_dto: GroupDto) -> Result<u64, DbErr> {
        info!("group_json is {:?}", group_dto);
        let update_result = Group::delete_many()
            .filter(group::Column::Id.is_in(group_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 