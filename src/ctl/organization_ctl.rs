use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    pojo::organization_pojo::*,
    svc::organization_svc::OrganizationSvc,
    util::{exception::internal_err, paged_struct::PageData, result_struct::RespResult},
    AppState, ResultJson,
};

pub struct OrganizationCtl();

impl OrganizationCtl {
    /// Returns a simple greeting message for the root endpoint
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
    
    /// Retrieves a list of organizations based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A list of organization records matching the search criteria
    pub async fn list(
        Query(condition): Query<OrganizationCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Vec<OrganizationVo>> {
        let organizations = OrganizationSvc::get_instance(&state).list(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(organizations)))
    }
    
    /// Retrieves a paginated list of organizations based on the provided search conditions
    /// 
    /// # Arguments
    /// * `condition` - Query parameters containing search criteria and pagination info
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// A paginated list of organization records matching the search criteria
    pub async fn page(
        Query(condition): Query<OrganizationCondition>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<PageData<OrganizationVo>> {
        let organizations = OrganizationSvc::get_instance(&state).page(condition)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(organizations)))
    }
    
    /// Creates a new organization record
    /// 
    /// # Arguments
    /// * `organization_dto` - Organization data transfer object containing the new organization's information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The ID of the newly created organization
    pub async fn save(
        State(state): State<Arc<AppState>>,
        Json(organization_dto): Json<OrganizationDto>,
    ) -> ResultJson<i64> {
        let organization_id = OrganizationSvc::get_instance(&state).save(organization_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(organization_id)))
    }
    
    /// Retrieves a single organization by its ID
    /// 
    /// # Arguments
    /// * `organization_id` - The ID of the organization to retrieve
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The organization record if found, None otherwise
    pub async fn get_by_id(
        Path(organization_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> ResultJson<Option<OrganizationVo>> {
        let organization = OrganizationSvc::get_instance(&state).get_by_id(organization_id)
            .await
            .map_err(internal_err)?;
    
        Ok(Json(RespResult::ok(organization)))
    }
    
    /// Updates an existing organization record
    /// 
    /// # Arguments
    /// * `organization_dto` - Organization data transfer object containing the updated information
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records updated
    pub async fn update_by_id(
        State(state): State<Arc<AppState>>,
        Json(organization_dto): Json<OrganizationDto>,
    ) -> ResultJson<u64> {
        let result = OrganizationSvc::get_instance(&state).update_by_id(organization_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Soft deletes multiple organizations by their IDs (marks them as deleted)
    /// 
    /// # Arguments
    /// * `organization_dto` - Organization data transfer object containing the IDs to delete
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records marked as deleted
    pub async fn delete_by_ids(
        State(state): State<Arc<AppState>>,
        Json(organization_dto): Json<OrganizationDto>,
    ) -> ResultJson<u64> {
        let result = OrganizationSvc::get_instance(&state).delete_by_ids(organization_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
    
    /// Permanently removes multiple organizations by their IDs
    /// 
    /// # Arguments
    /// * `organization_dto` - Organization data transfer object containing the IDs to remove
    /// * `state` - Application state containing database connection
    /// 
    /// # Returns
    /// The number of records permanently removed
    pub async fn remove_by_ids(
        State(state): State<Arc<AppState>>,
        Json(organization_dto): Json<OrganizationDto>,
    ) -> ResultJson<u64> {
        let result = OrganizationSvc::get_instance(&state).remove_by_ids(organization_dto)
            .await
            .map_err(internal_err)?;
        Ok(Json(RespResult::ok(result)))
    }
} 