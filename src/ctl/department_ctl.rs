use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::department_pojo::*,
    svc::department_svc::DepartmentSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct DepartmentCtl();

impl DepartmentCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of departments based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of department records matching the search criteria
    pub async fn list(
        Query(condition): Query<DepartmentCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<DepartmentVo>> {
        let departments = DepartmentSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(departments)))
    }
    
    /// Retrieves a paginated list of departments based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of department records matching the search criteria
    pub async fn page(
        Query(condition): Query<DepartmentCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<DepartmentVo>> {
        let departments = DepartmentSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(departments)))
    }
    
    /// Creates a new department record
    /// 
    /// # Arguments
    /// * `department_dto` - Department data transfer object containing the new department's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created department
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(department_dto): Json<DepartmentDto>,
    ) -> ResultJson<i64> {
        let department_id = DepartmentSvc::get_instance(&state).save(department_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(department_id)))
    }
    
    /// Retrieves a single department by its ID
    /// 
    /// # Arguments
    /// * `department_id` - The ID of the department to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The department record if found, None otherwise
    pub async fn get_by_id(
        Path(department_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<DepartmentVo>> {
        let department = DepartmentSvc::get_instance(&state).get_by_id(department_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(department)))
    }
    
    /// Updates an existing department record
    /// 
    /// # Arguments
    /// * `department_dto` - Department data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(department_dto): Json<DepartmentDto>,
    ) -> ResultJson<u64> {
        let result = DepartmentSvc::get_instance(&state).update_by_id(department_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple departments by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `department_dto` - Department data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(department_dto): Json<DepartmentDto>,
    ) -> ResultJson<u64> {
        let result = DepartmentSvc::get_instance(&state).delete_by_ids(department_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple departments by their IDs
    /// 
    /// # Arguments
    /// * `department_dto` - Department data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(department_dto): Json<DepartmentDto>,
    ) -> ResultJson<u64> {
        let result = DepartmentSvc::get_instance(&state).remove_by_ids(department_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 