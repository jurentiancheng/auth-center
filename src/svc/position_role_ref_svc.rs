use std::sync::Arc;

use crate::{
    mapper::PositionRoleRefMapperTrait,
    pojo::position_role_ref_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct PositionRoleRefSvc {
    mapper: Arc<dyn PositionRoleRefMapperTrait>,
}

impl PositionRoleRefSvc {
    pub fn new(mapper: Arc<dyn PositionRoleRefMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(
        &self,
        condition: PositionRoleRefCondition,
    ) -> Result<Vec<PositionRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }

    pub async fn page(
        &self,
        condition: PositionRoleRefCondition,
    ) -> Result<PageData<PositionRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }

    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<PositionRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }

    pub async fn save(&self, position_role_ref_dto: PositionRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(position_role_ref_dto).await
    }

    pub async fn update_by_id(
        &self,
        position_role_ref_dto: PositionRoleRefDto,
    ) -> Result<u64, DbErr> {
        self.mapper.update_by_id(position_role_ref_dto).await
    }

    pub async fn delete_by_ids(
        &self,
        position_role_ref_dto: PositionRoleRefDto,
    ) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(position_role_ref_dto).await
    }

    pub async fn remove_by_ids(
        &self,
        position_role_ref_dto: PositionRoleRefDto,
    ) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(position_role_ref_dto).await
    }
}
