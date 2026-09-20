use std::sync::Arc;

use crate::{
    mapper::UserRoleRefMapperTrait,
    pojo::user_role_ref_pojo::*,
    util::paged_struct::PageData,
};
use sea_orm::DbErr;

pub struct UserRoleRefSvc {
    mapper: Arc<dyn UserRoleRefMapperTrait>,
}

impl UserRoleRefSvc {
    pub fn new(mapper: Arc<dyn UserRoleRefMapperTrait>) -> Self {
        Self { mapper }
    }

    pub async fn list(&self, condition: UserRoleRefCondition) -> Result<Vec<UserRoleRefVo>, DbErr> {
        self.mapper.list(condition).await
    }

    pub async fn page(
        &self,
        condition: UserRoleRefCondition,
    ) -> Result<PageData<UserRoleRefVo>, DbErr> {
        self.mapper.page(condition).await
    }

    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserRoleRefVo>, DbErr> {
        self.mapper.get_by_id(rec_id).await
    }

    pub async fn save(&self, user_role_ref_dto: UserRoleRefDto) -> Result<i64, DbErr> {
        self.mapper.save(user_role_ref_dto).await
    }

    pub async fn update_by_id(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.update_by_id(user_role_ref_dto).await
    }

    pub async fn delete_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.delete_by_ids(user_role_ref_dto).await
    }

    pub async fn remove_by_ids(&self, user_role_ref_dto: UserRoleRefDto) -> Result<u64, DbErr> {
        self.mapper.remove_by_ids(user_role_ref_dto).await
    }
}
