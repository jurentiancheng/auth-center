use std::sync::Arc;

use crate::{mapper::position_role_ref_mapper::{PositionRoleRefMapper, PositionRoleRefMapperTrait}, pojo::position_role_ref_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct PositionRoleRefSvc {
    mapper: &'static PositionRoleRefMapper,
}

impl PositionRoleRefSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: PositionRoleRefMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static PositionRoleRefSvc {
        static INSTANCE: OnceCell<PositionRoleRefSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| PositionRoleRefSvc::new(state))
    }

    pub async fn list(&self, condition: PositionRoleRefCondition) -> Result<Vec<PositionRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: PositionRoleRefCondition) -> Result<PageData<PositionRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(position_role_ref_dto).await
    }
    
    pub async fn update_by_id(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(position_role_ref_dto).await
    }
    
    pub async fn delete_by_ids(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(position_role_ref_dto).await
    }
    
    pub async fn remove_by_ids(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(position_role_ref_dto).await
    }
} 