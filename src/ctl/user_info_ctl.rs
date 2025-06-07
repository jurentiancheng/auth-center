use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::user_info_pojo::*,
    svc::user_info_svc::UserInfoSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct UserInfoCtl();

impl UserInfoCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of user info records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of user info records matching the search criteria
    pub async fn list(
        Query(condition): Query<UserInfoCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<UserInfoVo>> {
        let user_infos = UserInfoSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_infos)))
    }
    
    /// Retrieves a paginated list of user info records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of user info records matching the search criteria
    pub async fn page(
        Query(condition): Query<UserInfoCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<UserInfoVo>> {
        let user_infos = UserInfoSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_infos)))
    }
    
    /// Creates a new user info record
    /// 
    /// # Arguments
    /// * `user_info_dto` - User info data transfer object containing the new user info's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created user info
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(user_info_dto): Json<UserInfoDto>,
    ) -> ResultJson<i64> {
        let user_info_id = UserInfoSvc::get_instance(&state).save(user_info_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(user_info_id)))
    }
    
    /// Retrieves a single user info by its ID
    /// 
    /// # Arguments
    /// * `user_info_id` - The ID of the user info to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The user info record if found, None otherwise
    pub async fn get_by_id(
        Path(user_info_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<UserInfoVo>> {
        let user_info = UserInfoSvc::get_instance(&state).get_by_id(user_info_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_info)))
    }
    
    /// Updates an existing user info record
    /// 
    /// # Arguments
    /// * `user_info_dto` - User info data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(user_info_dto): Json<UserInfoDto>,
    ) -> ResultJson<u64> {
        let result = UserInfoSvc::get_instance(&state).update_by_id(user_info_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple user info records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `user_info_dto` - User info data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_info_dto): Json<UserInfoDto>,
    ) -> ResultJson<u64> {
        let result = UserInfoSvc::get_instance(&state).delete_by_ids(user_info_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple user info records by their IDs
    /// 
    /// # Arguments
    /// * `user_info_dto` - User info data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_info_dto): Json<UserInfoDto>,
    ) -> ResultJson<u64> {
        let result = UserInfoSvc::get_instance(&state).remove_by_ids(user_info_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 