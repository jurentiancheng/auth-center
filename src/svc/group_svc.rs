use std::sync::Arc;

use crate::{mapper::group_mapper::{GroupMapper, GroupMapperTrait}, pojo::group_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct GroupSvc {
    mapper: &'static GroupMapper,
}

impl GroupSvc {
    pub fn new(state: &AppState) -> Self {
        Self { mapper: GroupMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static GroupSvc {
        static INSTANCE: OnceCell<GroupSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| GroupSvc::new(state))
    }

    pub async fn list(&self, condition: GroupCondition) -> Result<Vec<GroupVo>, DbErr> {
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: GroupCondition) -> Result<PageData<GroupVo>, DbErr> {
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<GroupVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, group_dto: GroupDto) -> Result<i64, DbErr> {
        self.mapper.save(group_dto).await
    }
    
    pub async fn update_by_id(&self, group_dto: GroupDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(group_dto).await
    }
    
    pub async fn delete_by_ids(&self, group_dto: GroupDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(group_dto).await
    }
    
    pub async fn remove_by_ids(&self, group_dto: GroupDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(group_dto).await
    }
} 