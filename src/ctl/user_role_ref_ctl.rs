use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::user_role_ref_pojo::*,
    svc::user_role_ref_svc::UserRoleRefSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct UserRoleRefCtl();

impl UserRoleRefCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of user role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of user role ref records matching the search criteria
    pub async fn list(
        Query(condition): Query<UserRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<UserRoleRefVo>> {
        let user_role_refs = UserRoleRefSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_role_refs)))
    }
    
    /// Retrieves a paginated list of user role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of user role ref records matching the search criteria
    pub async fn page(
        Query(condition): Query<UserRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<UserRoleRefVo>> {
        let user_role_refs = UserRoleRefSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_role_refs)))
    }
    
    /// Creates a new user role ref record
    /// 
    /// # Arguments
    /// * `user_role_ref_dto` - User role ref data transfer object containing the new user role ref's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created user role ref
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(user_role_ref_dto): Json<UserRoleRefDto>,
    ) -> ResultJson<i64> {
        let user_role_ref_id = UserRoleRefSvc::get_instance(&state).save(user_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(user_role_ref_id)))
    }
    
    /// Retrieves a single user role ref by its ID
    /// 
    /// # Arguments
    /// * `user_role_ref_id` - The ID of the user role ref to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The user role ref record if found, None otherwise
    pub async fn get_by_id(
        Path(user_role_ref_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<UserRoleRefVo>> {
        let user_role_ref = UserRoleRefSvc::get_instance(&state).get_by_id(user_role_ref_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(user_role_ref)))
    }
    
    /// Updates an existing user role ref record
    /// 
    /// # Arguments
    /// * `user_role_ref_dto` - User role ref data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(user_role_ref_dto): Json<UserRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = UserRoleRefSvc::get_instance(&state).update_by_id(user_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple user role ref records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `user_role_ref_dto` - User role ref data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_role_ref_dto): Json<UserRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = UserRoleRefSvc::get_instance(&state).delete_by_ids(user_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple user role ref records by their IDs
    /// 
    /// # Arguments
    /// * `user_role_ref_dto` - User role ref data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(user_role_ref_dto): Json<UserRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = UserRoleRefSvc::get_instance(&state).remove_by_ids(user_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 