use std::sync::Arc;

use crate::{
    mapper::DepartmentRoleRefMapperTrait,
    pojo::department_role_ref_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct DepartmentRoleRefSvc {
    mapper: Arc<dyn DepartmentRoleRefMapperTrait>,
}

impl DepartmentRoleRefSvc {
    pub fn new(mapper: Arc<dyn DepartmentRoleRefMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(&self, condition: DepartmentRoleRefCondition) -> Result<Vec<DepartmentRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: DepartmentRoleRefCondition) -> Result<PageData<DepartmentRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<DepartmentRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(department_role_ref_dto).await
    }
    
    pub async fn update_by_id(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(department_role_ref_dto).await
    }
    
    pub async fn delete_by_ids(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(department_role_ref_dto).await
    }
    
    pub async fn remove_by_ids(&self, department_role_ref_dto: DepartmentRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(department_role_ref_dto).await
    }
} 