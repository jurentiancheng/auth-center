use std::sync::Arc;

use crate::{mapper::user_wechat_info_mapper::{UserWechatInfoMapper, UserWechatInfoMapperTrait}, pojo::user_wechat_info_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct UserWechatInfoSvc {
    mapper: &'static UserWechatInfoMapper,
}

impl UserWechatInfoSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: UserWechatInfoMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static UserWechatInfoSvc {
        static INSTANCE: OnceCell<UserWechatInfoSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| UserWechatInfoSvc::new(state))
    }

    pub async fn list(&self, condition: UserWechatInfoCondition) -> Result<Vec<UserWechatInfoVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: UserWechatInfoCondition) -> Result<PageData<UserWechatInfoVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserWechatInfoVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<i64, DbErr> {
        self.mapper.save(user_wechat_info_dto).await
    }
    
    pub async fn update_by_id(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_wechat_info_dto).await
    }
    
    pub async fn delete_by_ids(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_wechat_info_dto).await
    }
    
    pub async fn remove_by_ids(&self, user_wechat_info_dto: UserWechatInfoDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_wechat_info_dto).await
    }
} 