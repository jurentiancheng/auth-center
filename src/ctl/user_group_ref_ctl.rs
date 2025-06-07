use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::user_group_ref_pojo::*,
    svc::user_group_ref_svc::UserGroupRefSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct UserGroupRefCtl();

impl UserGroupRefCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of user group ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of user group ref records matching the search criteria
    pub async fn list(
        Query(condition): Query<UserGroupRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<UserGroupRefVo>> {
        let user_group_refs = UserGroupRefSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_group_refs)))
    }
    
    /// Retrieves a paginated list of user group ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of user group ref records matching the search criteria
    pub async fn page(
        Query(condition): Query<UserGroupRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<UserGroupRefVo>> {
        let user_group_refs = UserGroupRefSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_group_refs)))
    }
    
    /// Creates a new user group ref record
    /// 
    /// # Arguments
    /// * `user_group_ref_dto` - User group ref data transfer object containing the new user group ref's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created user group ref
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(user_group_ref_dto): Json<UserGroupRefDto>,
    ) -> ResultJson<i64> {
        let user_group_ref_id = UserGroupRefSvc::get_instance(&state).save(user_group_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(user_group_ref_id)))
    }
    
    /// Retrieves a single user group ref by its ID
    /// 
    /// # Arguments
    /// * `user_group_ref_id` - The ID of the user group ref to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The user group ref record if found, None otherwise
    pub async fn get_by_id(
        Path(user_group_ref_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<UserGroupRefVo>> {
        let user_group_ref = UserGroupRefSvc::get_instance(&state).get_by_id(user_group_ref_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_group_ref)))
    }
    
    /// Updates an existing user group ref record
    /// 
    /// # Arguments
    /// * `user_group_ref_dto` - User group ref data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(user_group_ref_dto): Json<UserGroupRefDto>,
    ) -> ResultJson<u64> {
        let result = UserGroupRefSvc::get_instance(&state).update_by_id(user_group_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple user group ref records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `user_group_ref_dto` - User group ref data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_group_ref_dto): Json<UserGroupRefDto>,
    ) -> ResultJson<u64> {
        let result = UserGroupRefSvc::get_instance(&state).delete_by_ids(user_group_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple user group ref records by their IDs
    /// 
    /// # Arguments
    /// * `user_group_ref_dto` - User group ref data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_group_ref_dto): Json<UserGroupRefDto>,
    ) -> ResultJson<u64> {
        let result = UserGroupRefSvc::get_instance(&state).remove_by_ids(user_group_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 