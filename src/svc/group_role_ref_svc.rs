use std::sync::Arc;

use crate::{mapper::group_role_ref_mapper::{GroupRoleRefMapper, GroupRoleRefMapperTrait}, pojo::group_role_ref_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct GroupRoleRefSvc {
    mapper: &'static GroupRoleRefMapper,
}

impl GroupRoleRefSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: GroupRoleRefMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static GroupRoleRefSvc {
        static INSTANCE: OnceCell<GroupRoleRefSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| GroupRoleRefSvc::new(state))
    }

    pub async fn list(&self, condition: GroupRoleRefCondition) -> Result<Vec<GroupRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: GroupRoleRefCondition) -> Result<PageData<GroupRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<GroupRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(group_role_ref_dto).await
    }
    
    pub async fn update_by_id(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(group_role_ref_dto).await
    }
    
    pub async fn delete_by_ids(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(group_role_ref_dto).await
    }
    
    pub async fn remove_by_ids(&self, group_role_ref_dto: GroupRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(group_role_ref_dto).await
    }
} 