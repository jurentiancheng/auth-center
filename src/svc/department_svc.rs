use std::sync::Arc;

use crate::{mapper::department_mapper::{DepartmentMapper, DepartmentMapperTrait}, pojo::department_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct DepartmentSvc {
    mapper: &'static DepartmentMapper,
}

impl DepartmentSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: DepartmentMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static DepartmentSvc {
        static INSTANCE: OnceCell<DepartmentSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| DepartmentSvc::new(state))
    }

    pub async fn list(&self, condition: DepartmentCondition) -> Result<Vec<DepartmentVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: DepartmentCondition) -> Result<PageData<DepartmentVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<DepartmentVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, department_dto: DepartmentDto) -> Result<i64, DbErr> {
        self.mapper.save(department_dto).await
    }
    
    pub async fn update_by_id(&self, department_dto: DepartmentDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(department_dto).await
    }
    
    pub async fn delete_by_ids(&self, department_dto: DepartmentDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(department_dto).await
    }
    
    pub async fn remove_by_ids(&self, department_dto: DepartmentDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(department_dto).await
    }
} 