use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::position_pojo::*,
    svc::position_svc::PositionSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct PositionCtl();

impl PositionCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of positions based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of position records matching the search criteria
    pub async fn list(
        Query(condition): Query<PositionCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<PositionVo>> {
        let positions = PositionSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(positions)))
    }
    
    /// Retrieves a paginated list of positions based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of position records matching the search criteria
    pub async fn page(
        Query(condition): Query<PositionCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<PositionVo>> {
        let positions = PositionSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(positions)))
    }
    
    /// Creates a new position record
    /// 
    /// # Arguments
    /// * `position_dto` - Position data transfer object containing the new position's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created position
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(position_dto): Json<PositionDto>,
    ) -> ResultJson<i64> {
        let position_id = PositionSvc::get_instance(&state).save(position_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(position_id)))
    }
    
    /// Retrieves a single position by its ID
    /// 
    /// # Arguments
    /// * `position_id` - The ID of the position to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The position record if found, None otherwise
    pub async fn get_by_id(
        Path(position_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<PositionVo>> {
        let position = PositionSvc::get_instance(&state).get_by_id(position_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(position)))
    }
    
    /// Updates an existing position record
    /// 
    /// # Arguments
    /// * `position_dto` - Position data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(position_dto): Json<PositionDto>,
    ) -> ResultJson<u64> {
        let result = PositionSvc::get_instance(&state).update_by_id(position_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple positions by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `position_dto` - Position data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(position_dto): Json<PositionDto>,
    ) -> ResultJson<u64> {
        let result = PositionSvc::get_instance(&state).delete_by_ids(position_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple positions by their IDs
    /// 
    /// # Arguments
    /// * `position_dto` - Position data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(position_dto): Json<PositionDto>,
    ) -> ResultJson<u64> {
        let result = PositionSvc::get_instance(&state).remove_by_ids(position_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 