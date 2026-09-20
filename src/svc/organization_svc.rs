use std::sync::Arc;

use crate::{
    mapper::OrganizationMapperTrait,
    pojo::organization_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct OrganizationSvc {
    mapper: Arc<dyn OrganizationMapperTrait>,
}

impl OrganizationSvc {
    pub fn new(mapper: Arc<dyn OrganizationMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(&self, condition: OrganizationCondition) -> Result<Vec<OrganizationVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: OrganizationCondition) -> Result<PageData<OrganizationVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<OrganizationVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, organization_dto: OrganizationDto) -> Result<i64, DbErr> {
        self.mapper.save(organization_dto).await
    }
    
    pub async fn update_by_id(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(organization_dto).await
    }
    
    pub async fn delete_by_ids(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(organization_dto).await
    }
    
    pub async fn remove_by_ids(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(organization_dto).await
    }
} 