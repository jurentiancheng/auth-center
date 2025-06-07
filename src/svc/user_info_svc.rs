use std::sync::Arc;

use crate::{mapper::user_info_mapper::{UserInfoMapper, UserInfoMapperTrait}, pojo::user_info_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct UserInfoSvc {
    mapper: &'static UserInfoMapper,
}

impl UserInfoSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: UserInfoMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static UserInfoSvc {
        static INSTANCE: OnceCell<UserInfoSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| UserInfoSvc::new(state))
    }

    pub async fn list(&self, condition: UserInfoCondition) -> Result<Vec<UserInfoVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: UserInfoCondition) -> Result<PageData<UserInfoVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserInfoVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, user_info_dto: UserInfoDto) -> Result<i64, DbErr> {
        self.mapper.save(user_info_dto).await
    }
    
    pub async fn update_by_id(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_info_dto).await
    }
    
    pub async fn delete_by_ids(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_info_dto).await
    }
    
    pub async fn remove_by_ids(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_info_dto).await
    }
} 