use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::department_role_ref_pojo::*,
    svc::department_role_ref_svc::DepartmentRoleRefSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct DepartmentRoleRefCtl();

impl DepartmentRoleRefCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of department role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of department role ref records matching the search criteria
    pub async fn list(
        Query(condition): Query<DepartmentRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<DepartmentRoleRefVo>> {
        let department_role_refs = DepartmentRoleRefSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(department_role_refs)))
    }
    
    /// Retrieves a paginated list of department role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of department role ref records matching the search criteria
    pub async fn page(
        Query(condition): Query<DepartmentRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<DepartmentRoleRefVo>> {
        let department_role_refs = DepartmentRoleRefSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(department_role_refs)))
    }
    
    /// Creates a new department role ref record
    /// 
    /// # Arguments
    /// * `department_role_ref_dto` - Department role ref data transfer object containing the new department role ref's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created department role ref
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(department_role_ref_dto): Json<DepartmentRoleRefDto>,
    ) -> ResultJson<i64> {
        let department_role_ref_id = DepartmentRoleRefSvc::get_instance(&state).save(department_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(department_role_ref_id)))
    }
    
    /// Retrieves a single department role ref by its ID
    /// 
    /// # Arguments
    /// * `department_role_ref_id` - The ID of the department role ref to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The department role ref record if found, None otherwise
    pub async fn get_by_id(
        Path(department_role_ref_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<DepartmentRoleRefVo>> {
        let department_role_ref = DepartmentRoleRefSvc::get_instance(&state).get_by_id(department_role_ref_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(department_role_ref)))
    }
    
    /// Updates an existing department role ref record
    /// 
    /// # Arguments
    /// * `department_role_ref_dto` - Department role ref data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(department_role_ref_dto): Json<DepartmentRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = DepartmentRoleRefSvc::get_instance(&state).update_by_id(department_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple department role ref records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `department_role_ref_dto` - Department role ref data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(department_role_ref_dto): Json<DepartmentRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = DepartmentRoleRefSvc::get_instance(&state).delete_by_ids(department_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple department role ref records by their IDs
    /// 
    /// # Arguments
    /// * `department_role_ref_dto` - Department role ref data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(department_role_ref_dto): Json<DepartmentRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = DepartmentRoleRefSvc::get_instance(&state).remove_by_ids(department_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 