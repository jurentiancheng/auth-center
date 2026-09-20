use anyhow::Result;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Cond;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, QueryTrait,
};
use tracing::info;

use crate::entities::{prelude::*, *};
use crate::util::paged_struct::{PageData, PageInfo, Pageable};
use crate::util::IntoJsonValue;
use crate::pojo::user_wechat_info_pojo::*;
use sea_orm::Condition;
/// Trait defining the interface for user wechat info-related database operations
#[async_trait::async_trait]
pub trait UserWechatInfoMapperTrait: Send + Sync {
    async fn list(
        &self,
        condition: UserWechatInfoCondition,
    ) -> Result<Vec<UserWechatInfoVo>, DbErr>;
    async fn page(
        &self,
        condition: UserWechatInfoCondition,
    ) -> Result<PageData<UserWechatInfoVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserWechatInfoVo>, DbErr>;
    async fn save(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr>;
}

/// Implementation of UserWechatInfoMapperTrait
pub struct UserWechatInfoMapper {
    db: DatabaseConnection,
}

impl UserWechatInfoMapper {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn build_query_wrapper(&self, condition: &UserWechatInfoCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_wechat_info::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::Id.is_in(ids.clone()));
        };
        if let Some(union_id) = &condition.union_id {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::UnionId.eq(union_id));
        };
        if let Some(open_id) = &condition.mini_open_id {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::MiniOpenId.eq(open_id));
        };
        if let Some(union_id) = &condition.union_id {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::UnionId.eq(union_id));
        };
        if let Some(nickname) = &condition.nickname {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::Nickname.like(nickname));
        };

        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &UserWechatInfoCondition,
        list: Vec<UserWechatInfoVo>,
        total: u64,
    ) -> Result<PageData<UserWechatInfoVo>, DbErr> {
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
impl UserWechatInfoMapperTrait for UserWechatInfoMapper {
    async fn list(
        &self,
        condition: UserWechatInfoCondition,
    ) -> Result<Vec<UserWechatInfoVo>, DbErr> {
        let user_wechat_info = UserWechatInfo::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserWechatInfoVo>()
            .all(&self.db)
            .await?;

        Ok(user_wechat_info)
    }

    async fn page(
        &self,
        condition: UserWechatInfoCondition,
    ) -> Result<PageData<UserWechatInfoVo>, DbErr> {
        let user_wechat_info = UserWechatInfo::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserWechatInfoVo>()
            .all(&self.db)
            .await?;
        let total = UserWechatInfo::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.db)
            .await?;
        self.convert_page_data(&condition, user_wechat_info, total)
            .await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserWechatInfoVo>, DbErr> {
        let user_wechat_info_opt = UserWechatInfo::find_by_id(rec_id)
            .into_model::<UserWechatInfoVo>()
            .one(&self.db)
            .await?;
        Ok(user_wechat_info_opt)
    }

    async fn save(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<i64, DbErr> {
        let user_wechat_info_dtoc = user_wechat_info_dto.into_json_with_snake_key();
        info!("user_wechat_info_json is {:?}", user_wechat_info_dtoc);
        let mut user_wechat_info_actmod =
            user_wechat_info::ActiveModel::from_json(user_wechat_info_dtoc)?;
        user_wechat_info_actmod.set(
            user_wechat_info::Column::CreateBy,
            sea_orm::Value::BigInt(Some(0)),
        );
        user_wechat_info_actmod.set(
            user_wechat_info::Column::UpdateBy,
            sea_orm::Value::BigInt(Some(0)),
        );
        let inserted_result = UserWechatInfo::insert(user_wechat_info_actmod)
            .exec(&self.db)
            .await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr> {
        let user_wechat_info_dto_json = user_wechat_info_dto.into_json_with_snake_key();
        info!("user_wechat_info_json is {:?}", user_wechat_info_dto);
        let user_wechat_info_actmod =
            user_wechat_info::ActiveModel::from_json(user_wechat_info_dto_json)?;
        let update_result = UserWechatInfo::update_many()
            .set(user_wechat_info_actmod)
            .filter(user_wechat_info::Column::Id.eq(user_wechat_info_dto.rec_id))
            .exec(&self.db)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr> {
        info!("user_wechat_info_json is {:?}", user_wechat_info_dto);
        let update_result = UserWechatInfo::update_many()
            .col_expr(user_wechat_info::Column::IsDel, Expr::value(-1))
            .filter(user_wechat_info::Column::Id.is_in(user_wechat_info_dto.rec_ids.unwrap()))
            .exec(&self.db)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr> {
        info!("user_wechat_info_json is {:?}", user_wechat_info_dto);
        let update_result = UserWechatInfo::delete_many()
            .filter(user_wechat_info::Column::Id.is_in(user_wechat_info_dto.rec_ids.unwrap()))
            .exec(&self.db)
            .await?;
        Ok(update_result.rows_affected)
    }
}
