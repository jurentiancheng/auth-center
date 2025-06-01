use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::user_pojo::*,
    svc::user_svc::{ UserSvc },
    util::{ exception::internal_err, paged_struct::PageData, result_struct::RespResult },
    AppState, ResultJson,
};


pub struct UserCtl();

impl UserCtl {

    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of users based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of user records matching the search criteria
    pub async fn list(
        Query(condition): Query<UserCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<UserVo>> {
        
        let users = UserSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(users)))
    }
    
    /// Retrieves a paginated list of users based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of user records matching the search criteria
    pub async fn page(
        Query(condition): Query<UserCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<UserVo>> {
        let users = UserSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(users)))
    }
    
    /// Creates a new user record
    /// 
    /// # Arguments
    /// * `user_dto` - User data transfer object containing the new user's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created user
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(user_dto): Json<UserDto>,
    ) -> ResultJson<i64> {
        let user_id = UserSvc::get_instance(&state).save(user_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(user_id)))
    }
    
    /// Retrieves a single user by their ID
    /// 
    /// # Arguments
    /// * `user_id` - The ID of the user to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The user record if found, None otherwise
    pub async fn get_by_id(
        Path(user_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<UserVo>> {
        let users = UserSvc::get_instance(&state).get_by_id(user_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(users)))
    }
    
    /// Updates an existing user record
    /// 
    /// # Arguments
    /// * `user_dto` - User data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(user_dto): Json<UserDto>,
    ) -> ResultJson<u64> {
        let result = UserSvc::get_instance(&state).update_by_id(user_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple users by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `user_dto` - User data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_dto): Json<UserDto>,
    ) -> ResultJson<u64> {
        let result = UserSvc::get_instance(&state).delete_by_ids(user_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple users by their IDs
    /// 
    /// # Arguments
    /// * `user_dto` - User data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_dto): Json<UserDto>,
    ) -> ResultJson<u64> {
        let result = UserSvc::get_instance(&state).remove_by_ids(user_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
}

