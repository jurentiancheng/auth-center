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
use crate::{pojo::user_role_ref_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for user role ref-related database operations
#[async_trait::async_trait]
pub trait UserRoleRefMapperTrait {
    async fn list(&self, condition: UserRoleRefCondition) -> Result<Vec<UserRoleRefVo>, DbErr>;
    async fn page(&self, condition: UserRoleRefCondition) -> Result<PageData<UserRoleRefVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserRoleRefVo>, DbErr>;
    async fn save(&self, user_role_ref_dto: UserRoleRefDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr>;
}

/// Implementation of UserRoleRefMapperTrait
pub struct UserRoleRefMapper {
    state: Arc<AppState>,
}

impl UserRoleRefMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static UserRoleRefMapper {
        static INSTANCE: OnceCell<UserRoleRefMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| UserRoleRefMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &UserRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_role_ref::Column::Id.is_in(ids.clone()));
        };
        if let Some(user_code) = &condition.user_code {
            query_wrapper = query_wrapper.add(user_role_ref::Column::UserCode.eq(user_code));
        };
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(user_role_ref::Column::RoleCode.eq(role_code));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(user_role_ref::Column::OrgCode.eq(org_code));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &UserRoleRefCondition,
        list: Vec<UserRoleRefVo>,
        total: u64,
    ) -> Result<PageData<UserRoleRefVo>, DbErr> {
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
impl UserRoleRefMapperTrait for UserRoleRefMapper {
    async fn list(&self, condition: UserRoleRefCondition) -> Result<Vec<UserRoleRefVo>, DbErr> {
        let user_role_ref = UserRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(user_role_ref)
    }

    async fn page(&self, condition: UserRoleRefCondition) -> Result<PageData<UserRoleRefVo>, DbErr> {
        let user_role_ref = UserRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserRoleRefVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = UserRoleRef::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, user_role_ref, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserRoleRefVo>, DbErr> {
        let user_role_ref_opt = UserRoleRef::find_by_id(rec_id)
            .into_model::<UserRoleRefVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(user_role_ref_opt)
    }

    async fn save(&self, user_role_ref_dto: UserRoleRefDto) -> Result<i64, DbErr> {
        let user_role_ref_dtoc = user_role_ref_dto.into_json_with_snake_key();
        info!("user_role_ref_json is {:?}", user_role_ref_dtoc);
        let mut user_role_ref_actmod = user_role_ref::ActiveModel::from_json(user_role_ref_dtoc)?;
        user_role_ref_actmod.set(user_role_ref::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        user_role_ref_actmod.set(user_role_ref::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = UserRoleRef::insert(user_role_ref_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        let user_role_ref_dto_json = user_role_ref_dto.into_json_with_snake_key();
        info!("user_role_ref_json is {:?}", user_role_ref_dto);
        let user_role_ref_actmod = user_role_ref::ActiveModel::from_json(user_role_ref_dto_json)?;
        let update_result = UserRoleRef::update_many()
            .set(user_role_ref_actmod)
            .filter(user_role_ref::Column::Id.eq(user_role_ref_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        info!("user_role_ref_json is {:?}", user_role_ref_dto);
        let update_result = UserRoleRef::update_many()
            .col_expr(user_role_ref::Column::IsDel, Expr::value(-1))
            .filter(user_role_ref::Column::Id.is_in(user_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        info!("user_role_ref_json is {:?}", user_role_ref_dto);
        let update_result = UserRoleRef::delete_many()
            .filter(user_role_ref::Column::Id.is_in(user_role_ref_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 