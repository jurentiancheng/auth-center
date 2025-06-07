use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::organization_role_ref_pojo::*,
    svc::organization_role_ref_svc::OrganizationRoleRefSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct OrganizationRoleRefCtl();

impl OrganizationRoleRefCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of organization role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of organization role ref records matching the search criteria
    pub async fn list(
        Query(condition): Query<OrganizationRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<OrganizationRoleRefVo>> {
        let organization_role_refs = OrganizationRoleRefSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(organization_role_refs)))
    }
    
    /// Retrieves a paginated list of organization role ref records based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of organization role ref records matching the search criteria
    pub async fn page(
        Query(condition): Query<OrganizationRoleRefCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<OrganizationRoleRefVo>> {
        let organization_role_refs = OrganizationRoleRefSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(organization_role_refs)))
    }
    
    /// Creates a new organization role ref record
    /// 
    /// # Arguments
    /// * `organization_role_ref_dto` - Organization role ref data transfer object containing the new organization role ref's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created organization role ref
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(organization_role_ref_dto): Json<OrganizationRoleRefDto>,
    ) -> ResultJson<i64> {
        let organization_role_ref_id = OrganizationRoleRefSvc::get_instance(&state).save(organization_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(organization_role_ref_id)))
    }
    
    /// Retrieves a single organization role ref by its ID
    /// 
    /// # Arguments
    /// * `organization_role_ref_id` - The ID of the organization role ref to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The organization role ref record if found, None otherwise
    pub async fn get_by_id(
        Path(organization_role_ref_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<OrganizationRoleRefVo>> {
        let organization_role_ref = OrganizationRoleRefSvc::get_instance(&state).get_by_id(organization_role_ref_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(organization_role_ref)))
    }
    
    /// Updates an existing organization role ref record
    /// 
    /// # Arguments
    /// * `organization_role_ref_dto` - Organization role ref data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(organization_role_ref_dto): Json<OrganizationRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = OrganizationRoleRefSvc::get_instance(&state).update_by_id(organization_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple organization role ref records by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `organization_role_ref_dto` - Organization role ref data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(organization_role_ref_dto): Json<OrganizationRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = OrganizationRoleRefSvc::get_instance(&state).delete_by_ids(organization_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple organization role ref records by their IDs
    /// 
    /// # Arguments
    /// * `organization_role_ref_dto` - Organization role ref data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(organization_role_ref_dto): Json<OrganizationRoleRefDto>,
    ) -> ResultJson<u64> {
        let result = OrganizationRoleRefSvc::get_instance(&state).remove_by_ids(organization_role_ref_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 