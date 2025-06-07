use std::sync::Arc;

use crate::{mapper::permission_mapper::{PermissionMapper, PermissionMapperTrait}, pojo::permission_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct PermissionSvc {
    mapper: &'static PermissionMapper,
}

impl PermissionSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: PermissionMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static PermissionSvc {
        static INSTANCE: OnceCell<PermissionSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| PermissionSvc::new(state))
    }

    pub async fn list(&self, condition: PermissionCondition) -> Result<Vec<PermissionVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: PermissionCondition) -> Result<PageData<PermissionVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<PermissionVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, permission_dto: PermissionDto) -> Result<i64, DbErr> {
        self.mapper.save(permission_dto).await
    }
    
    pub async fn update_by_id(&self, permission_dto: PermissionDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(permission_dto).await
    }
    
    pub async fn delete_by_ids(&self, permission_dto: PermissionDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(permission_dto).await
    }
    
    pub async fn remove_by_ids(&self, permission_dto: PermissionDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(permission_dto).await
    }
} 