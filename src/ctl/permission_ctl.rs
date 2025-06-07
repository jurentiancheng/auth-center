use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::permission_pojo::*,
    svc::permission_svc::PermissionSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct PermissionCtl();

impl PermissionCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of permissions based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of permission records matching the search criteria
    pub async fn list(
        Query(condition): Query<PermissionCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<PermissionVo>> {
        let permissions = PermissionSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(permissions)))
    }
    
    /// Retrieves a paginated list of permissions based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of permission records matching the search criteria
    pub async fn page(
        Query(condition): Query<PermissionCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<PermissionVo>> {
        let permissions = PermissionSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(permissions)))
    }
    
    /// Creates a new permission record
    /// 
    /// # Arguments
    /// * `permission_dto` - Permission data transfer object containing the new permission's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created permission
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(permission_dto): Json<PermissionDto>,
    ) -> ResultJson<i64> {
        let permission_id = PermissionSvc::get_instance(&state).save(permission_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(permission_id)))
    }
    
    /// Retrieves a single permission by its ID
    /// 
    /// # Arguments
    /// * `permission_id` - The ID of the permission to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The permission record if found, None otherwise
    pub async fn get_by_id(
        Path(permission_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<PermissionVo>> {
        let permission = PermissionSvc::get_instance(&state).get_by_id(permission_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(permission)))
    }
    
    /// Updates an existing permission record
    /// 
    /// # Arguments
    /// * `permission_dto` - Permission data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(permission_dto): Json<PermissionDto>,
    ) -> ResultJson<u64> {
        let result = PermissionSvc::get_instance(&state).update_by_id(permission_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple permissions by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `permission_dto` - Permission data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(permission_dto): Json<PermissionDto>,
    ) -> ResultJson<u64> {
        let result = PermissionSvc::get_instance(&state).delete_by_ids(permission_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple permissions by their IDs
    /// 
    /// # Arguments
    /// * `permission_dto` - Permission data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(permission_dto): Json<PermissionDto>,
    ) -> ResultJson<u64> {
        let result = PermissionSvc::get_instance(&state).remove_by_ids(permission_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 