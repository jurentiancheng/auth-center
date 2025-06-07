use std::sync::Arc;

use crate::{mapper::position_mapper::{PositionMapper, PositionMapperTrait}, pojo::position_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct PositionSvc {
    mapper: &'static PositionMapper,
}

impl PositionSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: PositionMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static PositionSvc {
        static INSTANCE: OnceCell<PositionSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| PositionSvc::new(state))
    }

    pub async fn list(&self, condition: PositionCondition) -> Result<Vec<PositionVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: PositionCondition) -> Result<PageData<PositionVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, position_dto: PositionDto) -> Result<i64, DbErr> {
        self.mapper.save(position_dto).await
    }
    
    pub async fn update_by_id(&self, position_dto: PositionDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(position_dto).await
    }
    
    pub async fn delete_by_ids(&self, position_dto: PositionDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(position_dto).await
    }
    
    pub async fn remove_by_ids(&self, position_dto: PositionDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(position_dto).await
    }
} 