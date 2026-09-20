use std::sync::Arc;

use crate::{
    mapper::UserGroupRefMapperTrait,
    pojo::user_group_ref_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct UserGroupRefSvc {
    mapper: Arc<dyn UserGroupRefMapperTrait>,
}

impl UserGroupRefSvc {
    pub fn new(mapper: Arc<dyn UserGroupRefMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(
        &self,
        condition: UserGroupRefCondition,
    ) -> Result<Vec<UserGroupRefVo>, DbErr> {
        self.mapper.list(condition).await
    }

    pub async fn page(
        &self,
        condition: UserGroupRefCondition,
    ) -> Result<PageData<UserGroupRefVo>, DbErr> {
        self.mapper.page(condition).await
    }

    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserGroupRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }

    pub async fn save(&self, user_group_ref_dto: UserGroupRefDto) -> Result<i64, DbErr> {
        self.mapper.save(user_group_ref_dto).await
    }

    pub async fn update_by_id(&self, user_group_ref_dto: UserGroupRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_group_ref_dto).await
    }

    pub async fn delete_by_ids(&self, user_group_ref_dto: UserGroupRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_group_ref_dto).await
    }

    pub async fn remove_by_ids(&self, user_group_ref_dto: UserGroupRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_group_ref_dto).await
    }
}
