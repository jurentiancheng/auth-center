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
use crate::{pojo::user_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for user-related database operations
#[async_trait::async_trait]
pub trait UserMapperTrait {
    async fn list(&self, condition: UserCondition) -> Result<Vec<UserVo>, DbErr>;
    async fn page(&self, condition: UserCondition) -> Result<PageData<UserVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserVo>, DbErr>;
    async fn save(&self, user_dto: UserDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, user_dto: UserDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, user_dto: UserDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, user_dto: UserDto) -> Result<u64, DbErr>;
}

/// Implementation of UserMapperTrait
pub struct UserMapper {
    state: Arc<AppState>,
}

impl UserMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static UserMapper {
        static INSTANCE: OnceCell<UserMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| UserMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &UserCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user::Column::Id.is_in(ids.clone()));
        };
        if let Some(user_name) = &condition.user_name {
            query_wrapper = query_wrapper.add(user::Column::UserName.eq(user_name));
        };
        if let Some(rean_name) = &condition.real_name {
            query_wrapper = query_wrapper.add(user::Column::RealName.eq(rean_name));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &UserCondition,
        list: Vec<UserVo>,
        total: u64,
    ) -> Result<PageData<UserVo>, DbErr> {
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
impl UserMapperTrait for UserMapper {
    async fn list(&self, condition: UserCondition) -> Result<Vec<UserVo>, DbErr> {
        let user = User::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(user)
    }

    async fn page(&self, condition: UserCondition) -> Result<PageData<UserVo>, DbErr> {
        let user = User::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = User::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, user, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserVo>, DbErr> {
        let user_opt = User::find_by_id(rec_id)
            .into_model::<UserVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(user_opt)
    }

    async fn save(&self, user_dto: UserDto) -> Result<i64, DbErr> {
        let user_dtoc = user_dto.into_json_with_snake_key();
        info!("user_json is {:?}", user_dtoc);
        let mut user_actmod = user::ActiveModel::from_json(user_dtoc)?;
        user_actmod.set(user::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        user_actmod.set(user::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = User::insert(user_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, user_dto: UserDto) -> Result<u64, DbErr> {
        let user_dto_json = user_dto.into_json_with_snake_key();
        info!("user_json is {:?}", user_dto);
        let user_actmod = user::ActiveModel::from_json(user_dto_json)?;
        let update_result = User::update_many()
            .set(user_actmod)
            .filter(user::Column::Id.eq(user_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, user_dto: UserDto) -> Result<u64, DbErr> {
        info!("user_json is {:?}", user_dto);
        let update_result = User::update_many()
            .col_expr(user::Column::IsDel, Expr::value(-1))
            .filter(user::Column::Id.is_in(user_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, user_dto: UserDto) -> Result<u64, DbErr> {
        info!("user_json is {:?}", user_dto);
        let update_result = User::delete_many()
            .filter(user::Column::Id.is_in(user_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
}
