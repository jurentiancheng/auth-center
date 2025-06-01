use std::sync::Arc;

use crate::{mapper::user_mapper::{UserMapper, UserMapperTrait}, pojo::user_pojo::*, util::paged_struct::PageData, AppState};
use once_cell::sync::OnceCell;
use sea_orm::DbErr;

pub struct UserSvc {
    mapper: &'static UserMapper,
}

impl UserSvc {

    pub fn new(state: &AppState) -> Self {
        Self { mapper: UserMapper::get_instance(Arc::new(state.clone())) }
    }

    pub fn get_instance(state: &AppState) -> &'static UserSvc {
        static INSTANCE: OnceCell<UserSvc> = OnceCell::new();
        INSTANCE.get_or_init(|| UserSvc::new(state))
    }

    pub async fn list(&self, condition: UserCondition) -> Result<Vec<UserVo>, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.list(condition).await
    }
    
    pub async fn page(&self, condition: UserCondition) -> Result<PageData<UserVo>, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.page(condition).await
    }
    
    pub async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserVo>, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.get_by_id(rec_id).await
    }
    
    pub async fn save(&self, user_dto: UserDto) -> Result<i64, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.save(user_dto).await
    }
    
    pub async fn update_by_id(&self, user_dto: UserDto) -> Result<u64, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.update_by_id(user_dto).await
    }
    
    pub async fn delete_by_ids(&self, user_dto: UserDto) -> Result<u64, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.delete_by_ids(user_dto).await
    }
    
    pub async fn remove_by_ids(&self, user_dto: UserDto) -> Result<u64, DbErr> {
        //let mapper = UserMapper::get_instance(Arc::new(state.clone()));
        self.mapper.remove_by_ids(user_dto).await
    }
    
}




