use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::group_role_ref_pojo::*,
    svc::group_role_ref_svc::GroupRoleRefSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct GroupRoleRefCtl();

impl GroupRoleRefCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of group role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of group role ref records matching the search criteria
    pub async fn list(
        Query(condition): Query<GroupRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<GroupRoleRefVo>> {
        let group_role_refs = GroupRoleRefSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(group_role_refs)))
    }
    
    /// Retrieves a paginated list of group role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of group role ref records matching the search criteria
    pub async fn page(
        Query(condition): Query<GroupRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<GroupRoleRefVo>> {
        let group_role_refs = GroupRoleRefSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(group_role_refs)))
    }
    
    /// Creates a new group role ref record
    /// 
    /// # Arguments
    /// * `group_role_ref_dto` - Group role ref data transfer object containing the new group role ref's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created group role ref
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(group_role_ref_dto): Json<GroupRoleRefDto>,
    ) -> ResultJson<i64> {
        let group_role_ref_id = GroupRoleRefSvc::get_instance(&state).save(group_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(group_role_ref_id)))
    }
    
    /// Retrieves a single group role ref by its ID
    /// 
    /// # Arguments
    /// * `group_role_ref_id` - The ID of the group role ref to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The group role ref record if found, None otherwise
    pub async fn get_by_id(
        Path(group_role_ref_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<GroupRoleRefVo>> {
        let group_role_ref = GroupRoleRefSvc::get_instance(&state).get_by_id(group_role_ref_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(group_role_ref)))
    }
    
    /// Updates an existing group role ref record
    /// 
    /// # Arguments
    /// * `group_role_ref_dto` - Group role ref data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(group_role_ref_dto): Json<GroupRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = GroupRoleRefSvc::get_instance(&state).update_by_id(group_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple group role ref records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `group_role_ref_dto` - Group role ref data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(group_role_ref_dto): Json<GroupRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = GroupRoleRefSvc::get_instance(&state).delete_by_ids(group_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple group role ref records by their IDs
    /// 
    /// # Arguments
    /// * `group_role_ref_dto` - Group role ref data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(group_role_ref_dto): Json<GroupRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = GroupRoleRefSvc::get_instance(&state).remove_by_ids(group_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 