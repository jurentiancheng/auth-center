use std::sync::Arc;

use crate::{mapper::user_group_ref_mapper::{UserGroupRefMapper, UserGroupRefMapperTrait}, pojo::user_group_ref_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct UserGroupRefSvc {
    mapper: &'static UserGroupRefMapper,
}

impl UserGroupRefSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: UserGroupRefMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static UserGroupRefSvc {
        static INSTANCE: OnceCell<UserGroupRefSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| UserGroupRefSvc::new(state))
    }

    pub async fn list(&self, condition: UserGroupRefCondition) -> Result<Vec<UserGroupRefVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: UserGroupRefCondition) -> Result<PageData<UserGroupRefVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserGroupRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, user_group_ref_dto: UserGroupRefDto) -> Result<i64, DbErr> {
        self.mapper.save(user_group_ref_dto).await
    }
    
    pub async fn update_by_id(&self, user_group_ref_dto: UserGroupRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_group_ref_dto).await
    }
    
    pub async fn delete_by_ids(&self, user_group_ref_dto: UserGroupRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_group_ref_dto).await
    }
    
    pub async fn remove_by_ids(&self, user_group_ref_dto: UserGroupRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_group_ref_dto).await
    }
} 