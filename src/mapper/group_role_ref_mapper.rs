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
use crate::{pojo::group_role_ref_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for group role ref-related database operations
#[async_trait::async_trait]
pub trait GroupRoleRefMapperTrait {
    async fn list(&self, condition: GroupRoleRefCondition) -> Result<Vec<GroupRoleRefVo>, DbErr>;
    async fn page(&self, condition: GroupRoleRefCondition) -> Result<PageData<GroupRoleRefVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<GroupRoleRefVo>, DbErr>;
    async fn save(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr>;
}

/// Implementation of GroupRoleRefMapperTrait
pub struct GroupRoleRefMapper {
    state: Arc<AppState>,
}

impl GroupRoleRefMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static GroupRoleRefMapper {
        static INSTANCE: OnceCell<GroupRoleRefMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| GroupRoleRefMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &GroupRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(group_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(group_role_ref::Column::Id.is_in(ids.clone()));
        };
        if let Some(group_code) = &condition.group_code {
            query_wrapper = query_wrapper.add(group_role_ref::Column::GroupCode.eq(group_code));
        };
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(group_role_ref::Column::RoleCode.eq(role_code));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(group_role_ref::Column::OrgCode.eq(org_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &GroupRoleRefCondition,
        list: Vec<GroupRoleRefVo>,
        total: u64,
    ) -> Result<PageData<GroupRoleRefVo>, DbErr> {
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
impl GroupRoleRefMapperTrait for GroupRoleRefMapper {
    async fn list(&self, condition: GroupRoleRefCondition) -> Result<Vec<GroupRoleRefVo>, DbErr> {
        let group_role_ref = GroupRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<GroupRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(group_role_ref)
    }

    async fn page(&self, condition: GroupRoleRefCondition) -> Result<PageData<GroupRoleRefVo>, DbErr> {
        let group_role_ref = GroupRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<GroupRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = GroupRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, group_role_ref, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<GroupRoleRefVo>, DbErr> {
        let group_role_ref_opt = GroupRoleRef::find_by_id(rec_id)
            .into_model::<GroupRoleRefVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(group_role_ref_opt)
    }

    async fn save(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<i64, DbErr> {
        let group_role_ref_dtoc = group_role_ref_dto.into_json_with_snake_key();
        info!("group_role_ref_json is {:?}", group_role_ref_dtoc);
        let mut group_role_ref_actmod = group_role_ref::ActiveModel::from_json(group_role_ref_dtoc)?;
        group_role_ref_actmod.set(group_role_ref::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        group_role_ref_actmod.set(group_role_ref::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = GroupRoleRef::insert(group_role_ref_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr> {
        let group_role_ref_dto_json = group_role_ref_dto.into_json_with_snake_key();
        info!("group_role_ref_json is {:?}", group_role_ref_dto);
        let group_role_ref_actmod = group_role_ref::ActiveModel::from_json(group_role_ref_dto_json)?;
        let update_result = GroupRoleRef::update_many()
            .set(group_role_ref_actmod)
            .filter(group_role_ref::Column::Id.eq(group_role_ref_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr> {
        info!("group_role_ref_json is {:?}", group_role_ref_dto);
        let update_result = GroupRoleRef::update_many()
            .col_expr(group_role_ref::Column::IsDel, Expr::value(-1))
            .filter(group_role_ref::Column::Id.is_in(group_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr> {
        info!("group_role_ref_json is {:?}", group_role_ref_dto);
        let update_result = GroupRoleRef::delete_many()
            .filter(group_role_ref::Column::Id.is_in(group_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 