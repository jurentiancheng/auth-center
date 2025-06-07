use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::group_pojo::*,
    svc::group_svc::GroupSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct GroupCtl();

impl GroupCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of groups based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of group records matching the search criteria
    pub async fn list(
        Query(condition): Query<GroupCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<GroupVo>> {
        let groups = GroupSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(groups)))
    }
    
    /// Retrieves a paginated list of groups based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of group records matching the search criteria
    pub async fn page(
        Query(condition): Query<GroupCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<GroupVo>> {
        let groups = GroupSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(groups)))
    }
    
    /// Creates a new group record
    /// 
    /// # Arguments
    /// * `group_dto` - Group data transfer object containing the new group's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created group
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(group_dto): Json<GroupDto>,
    ) -> ResultJson<i64> {
        let group_id = GroupSvc::get_instance(&state).save(group_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(group_id)))
    }
    
    /// Retrieves a single group by its ID
    /// 
    /// # Arguments
    /// * `group_id` - The ID of the group to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The group record if found, None otherwise
    pub async fn get_by_id(
        Path(group_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<GroupVo>> {
        let group = GroupSvc::get_instance(&state).get_by_id(group_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(group)))
    }
    
    /// Updates an existing group record
    /// 
    /// # Arguments
    /// * `group_dto` - Group data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(group_dto): Json<GroupDto>,
    ) -> ResultJson<u64> {
        let result = GroupSvc::get_instance(&state).update_by_id(group_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple groups by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `group_dto` - Group data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(group_dto): Json<GroupDto>,
    ) -> ResultJson<u64> {
        let result = GroupSvc::get_instance(&state).delete_by_ids(group_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple groups by their IDs
    /// 
    /// # Arguments
    /// * `group_dto` - Group data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(group_dto): Json<GroupDto>,
    ) -> ResultJson<u64> {
        let result = GroupSvc::get_instance(&state).remove_by_ids(group_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 