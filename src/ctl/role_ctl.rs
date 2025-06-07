use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::role_pojo::*,
    svc::role_svc::RoleSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct RoleCtl();

impl RoleCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of roles based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of role records matching the search criteria
    pub async fn list(
        Query(condition): Query<RoleCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<RoleVo>> {
        let roles = RoleSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(roles)))
    }
    
    /// Retrieves a paginated list of roles based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of role records matching the search criteria
    pub async fn page(
        Query(condition): Query<RoleCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<RoleVo>> {
        let roles = RoleSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(roles)))
    }
    
    /// Creates a new role record
    /// 
    /// # Arguments
    /// * `role_dto` - Role data transfer object containing the new role's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created role
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(role_dto): Json<RoleDto>,
    ) -> ResultJson<i64> {
        let role_id = RoleSvc::get_instance(&state).save(role_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(role_id)))
    }
    
    /// Retrieves a single role by its ID
    /// 
    /// # Arguments
    /// * `role_id` - The ID of the role to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The role record if found, None otherwise
    pub async fn get_by_id(
        Path(role_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<RoleVo>> {
        let role = RoleSvc::get_instance(&state).get_by_id(role_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(role)))
    }
    
    /// Updates an existing role record
    /// 
    /// # Arguments
    /// * `role_dto` - Role data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(role_dto): Json<RoleDto>,
    ) -> ResultJson<u64> {
        let result = RoleSvc::get_instance(&state).update_by_id(role_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple roles by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `role_dto` - Role data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(role_dto): Json<RoleDto>,
    ) -> ResultJson<u64> {
        let result = RoleSvc::get_instance(&state).delete_by_ids(role_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple roles by their IDs
    /// 
    /// # Arguments
    /// * `role_dto` - Role data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(role_dto): Json<RoleDto>,
    ) -> ResultJson<u64> {
        let result = RoleSvc::get_instance(&state).remove_by_ids(role_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 