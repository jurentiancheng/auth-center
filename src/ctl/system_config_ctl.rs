use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::system_config_pojo::*,
    svc::system_config_svc::SystemConfigSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct SystemConfigCtl();

impl SystemConfigCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of system config records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of system config records matching the search criteria
    pub async fn list(
        Query(condition): Query<SystemConfigCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<SystemConfigVo>> {
        let system_configs = SystemConfigSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(system_configs)))
    }
    
    /// Retrieves a paginated list of system config records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of system config records matching the search criteria
    pub async fn page(
        Query(condition): Query<SystemConfigCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<SystemConfigVo>> {
        let system_configs = SystemConfigSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(system_configs)))
    }
    
    /// Creates a new system config record
    /// 
    /// # Arguments
    /// * `system_config_dto` - System config data transfer object containing the new system config's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created system config
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(system_config_dto): Json<SystemConfigDto>,
    ) -> ResultJson<i64> {
        let system_config_id = SystemConfigSvc::get_instance(&state).save(system_config_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(system_config_id)))
    }
    
    /// Retrieves a single system config by its ID
    /// 
    /// # Arguments
    /// * `system_config_id` - The ID of the system config to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The system config record if found, None otherwise
    pub async fn get_by_id(
        Path(system_config_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<SystemConfigVo>> {
        let system_config = SystemConfigSvc::get_instance(&state).get_by_id(system_config_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(system_config)))
    }
    
    /// Updates an existing system config record
    /// 
    /// # Arguments
    /// * `system_config_dto` - System config data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(system_config_dto): Json<SystemConfigDto>,
    ) -> ResultJson<u64> {
        let result = SystemConfigSvc::get_instance(&state).update_by_id(system_config_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple system config records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `system_config_dto` - System config data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(system_config_dto): Json<SystemConfigDto>,
    ) -> ResultJson<u64> {
        let result = SystemConfigSvc::get_instance(&state).delete_by_ids(system_config_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple system config records by their IDs
    /// 
    /// # Arguments
    /// * `system_config_dto` - System config data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(system_config_dto): Json<SystemConfigDto>,
    ) -> ResultJson<u64> {
        let result = SystemConfigSvc::get_instance(&state).remove_by_ids(system_config_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 