use std::sync::Arc;

use crate::{mapper::role_mapper::{RoleMapper, RoleMapperTrait}, pojo::role_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct RoleSvc {
    mapper: &'static RoleMapper,
}

impl RoleSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: RoleMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static RoleSvc {
        static INSTANCE: OnceCell<RoleSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| RoleSvc::new(state))
    }

    pub async fn list(&self, condition: RoleCondition) -> Result<Vec<RoleVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: RoleCondition) -> Result<PageData<RoleVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<RoleVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, role_dto: RoleDto) -> Result<i64, DbErr> {
        self.mapper.save(role_dto).await
    }
    
    pub async fn update_by_id(&self, role_dto: RoleDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(role_dto).await
    }
    
    pub async fn delete_by_ids(&self, role_dto: RoleDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(role_dto).await
    }
    
    pub async fn remove_by_ids(&self, role_dto: RoleDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(role_dto).await
    }
} 