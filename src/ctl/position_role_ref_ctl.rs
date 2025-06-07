use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::position_role_ref_pojo::*,
    svc::position_role_ref_svc::PositionRoleRefSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct PositionRoleRefCtl();

impl PositionRoleRefCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of position role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of position role ref records matching the search criteria
    pub async fn list(
        Query(condition): Query<PositionRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<PositionRoleRefVo>> {
        let position_role_refs = PositionRoleRefSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(position_role_refs)))
    }
    
    /// Retrieves a paginated list of position role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of position role ref records matching the search criteria
    pub async fn page(
        Query(condition): Query<PositionRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<PositionRoleRefVo>> {
        let position_role_refs = PositionRoleRefSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(position_role_refs)))
    }
    
    /// Creates a new position role ref record
    /// 
    /// # Arguments
    /// * `position_role_ref_dto` - Position role ref data transfer object containing the new position role ref's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created position role ref
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(position_role_ref_dto): Json<PositionRoleRefDto>,
    ) -> ResultJson<i64> {
        let position_role_ref_id = PositionRoleRefSvc::get_instance(&state).save(position_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(position_role_ref_id)))
    }
    
    /// Retrieves a single position role ref by its ID
    /// 
    /// # Arguments
    /// * `position_role_ref_id` - The ID of the position role ref to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The position role ref record if found, None otherwise
    pub async fn get_by_id(
        Path(position_role_ref_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<PositionRoleRefVo>> {
        let position_role_ref = PositionRoleRefSvc::get_instance(&state).get_by_id(position_role_ref_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(position_role_ref)))
    }
    
    /// Updates an existing position role ref record
    /// 
    /// # Arguments
    /// * `position_role_ref_dto` - Position role ref data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(position_role_ref_dto): Json<PositionRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = PositionRoleRefSvc::get_instance(&state).update_by_id(position_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple position role ref records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `position_role_ref_dto` - Position role ref data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(position_role_ref_dto): Json<PositionRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = PositionRoleRefSvc::get_instance(&state).delete_by_ids(position_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple position role ref records by their IDs
    /// 
    /// # Arguments
    /// * `position_role_ref_dto` - Position role ref data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(position_role_ref_dto): Json<PositionRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = PositionRoleRefSvc::get_instance(&state).remove_by_ids(position_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 