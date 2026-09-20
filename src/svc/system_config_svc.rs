use std::sync::Arc;

use crate::{
    mapper::SystemConfigMapperTrait,
    pojo::system_config_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct SystemConfigSvc {
    mapper: Arc<dyn SystemConfigMapperTrait>,
}

impl SystemConfigSvc {
    pub fn new(mapper: Arc<dyn SystemConfigMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(
        &self,
        condition: SystemConfigCondition,
    ) -> Result<Vec<SystemConfigVo>, DbErr> {
        self.mapper.list(condition).await
    }

    pub async fn page(
        &self,
        condition: SystemConfigCondition,
    ) -> Result<PageData<SystemConfigVo>, DbErr> {
        self.mapper.page(condition).await
    }

    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<SystemConfigVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }

    pub async fn save(&self, system_config_dto: SystemConfigDto) -> Result<i64, DbErr> {
        self.mapper.save(system_config_dto).await
    }

    pub async fn update_by_id(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(system_config_dto).await
    }

    pub async fn delete_by_ids(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(system_config_dto).await
    }

    pub async fn remove_by_ids(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(system_config_dto).await
    }
}
