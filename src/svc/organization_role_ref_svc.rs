use std::sync::Arc;

use crate::{mapper::organization_role_ref_mapper::{OrganizationRoleRefMapper, OrganizationRoleRefMapperTrait}, pojo::organization_role_ref_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct OrganizationRoleRefSvc {
    mapper: &'static OrganizationRoleRefMapper,
}

impl OrganizationRoleRefSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: OrganizationRoleRefMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static OrganizationRoleRefSvc {
        static INSTANCE: OnceCell<OrganizationRoleRefSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| OrganizationRoleRefSvc::new(state))
    }

    pub async fn list(&self, condition: OrganizationRoleRefCondition) -> Result<Vec<OrganizationRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: OrganizationRoleRefCondition) -> Result<PageData<OrganizationRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<OrganizationRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(organization_role_ref_dto).await
    }
    
    pub async fn update_by_id(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(organization_role_ref_dto).await
    }
    
    pub async fn delete_by_ids(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(organization_role_ref_dto).await
    }
    
    pub async fn remove_by_ids(&self, organization_role_ref_dto: OrganizationRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(organization_role_ref_dto).await
    }
} 