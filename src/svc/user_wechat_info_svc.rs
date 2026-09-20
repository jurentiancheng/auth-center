use std::sync::Arc;

use crate::{
    mapper::UserWechatInfoMapperTrait,
    pojo::user_wechat_info_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct UserWechatInfoSvc {
    mapper: Arc<dyn UserWechatInfoMapperTrait>,
}

impl UserWechatInfoSvc {
    pub fn new(mapper: Arc<dyn UserWechatInfoMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(
        &self,
        condition: UserWechatInfoCondition,
    ) -> Result<Vec<UserWechatInfoVo>, DbErr> {
        self.mapper.list(condition).await
    }

    pub async fn page(
        &self,
        condition: UserWechatInfoCondition,
    ) -> Result<PageData<UserWechatInfoVo>, DbErr> {
        self.mapper.page(condition).await
    }

    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserWechatInfoVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }

    pub async fn save(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<i64, DbErr> {
        self.mapper.save(user_wechat_info_dto).await
    }

    pub async fn update_by_id(
        &self,
        user_wechat_info_dto: UserWechatInfoDto,
    ) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_wechat_info_dto).await
    }

    pub async fn delete_by_ids(
        &self,
        user_wechat_info_dto: UserWechatInfoDto,
    ) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_wechat_info_dto).await
    }

    pub async fn remove_by_ids(
        &self,
        user_wechat_info_dto: UserWechatInfoDto,
    ) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_wechat_info_dto).await
    }
}
