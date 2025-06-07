use std::sync::Arc;

use crate::{mapper::user_role_ref_mapper::{UserRoleRefMapper, UserRoleRefMapperTrait}, pojo::user_role_ref_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct UserRoleRefSvc {
    mapper: &'static UserRoleRefMapper,
}

impl UserRoleRefSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: UserRoleRefMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static UserRoleRefSvc {
        static INSTANCE: OnceCell<UserRoleRefSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| UserRoleRefSvc::new(state))
    }

    pub async fn list(&self, condition: UserRoleRefCondition) -> Result<Vec<UserRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: UserRoleRefCondition) -> Result<PageData<UserRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, user_role_ref_dto: UserRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(user_role_ref_dto).await
    }
    
    pub async fn update_by_id(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_role_ref_dto).await
    }
    
    pub async fn delete_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_role_ref_dto).await
    }
    
    pub async fn remove_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_role_ref_dto).await
    }
} 