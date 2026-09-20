pub mod organization_mapper;
pub mod role_mapper;
pub mod permission_mapper;
pub mod position_mapper;
pub mod department_mapper;
pub mod group_mapper;
pub mod system_config_mapper;
pub mod user_wechat_info_mapper;
pub mod user_mapper;
pub mod user_info_mapper;
pub mod user_role_ref_mapper;
pub mod user_group_ref_mapper;
pub mod position_role_ref_mapper;
pub mod department_role_ref_mapper;
pub mod group_role_ref_mapper;
pub mod organization_role_ref_mapper;
use std::sync::Arc;

pub use organization_mapper::*;
pub use role_mapper::*;
pub use permission_mapper::*;
pub use position_mapper::*;
pub use department_mapper::*;
pub use group_mapper::*;
use sea_orm::DatabaseConnection;
pub use system_config_mapper::*;
pub use user_wechat_info_mapper::*;
pub use user_mapper::*;
pub use user_info_mapper::*;
pub use user_role_ref_mapper::*;
pub use user_group_ref_mapper::*;
pub use position_role_ref_mapper::*;
pub use department_role_ref_mapper::*;
pub use group_role_ref_mapper::*;
pub use organization_role_ref_mapper::*;


#[derive(Clone)]
pub struct Mappers {
     pub department: Arc<dyn DepartmentMapperTrait>,
     pub department_role_ref_mapper: Arc<dyn DepartmentRoleRefMapperTrait>,
     pub group: Arc<dyn GroupMapperTrait>,
     pub group_role_ref: Arc<dyn GroupRoleRefMapperTrait>,
     pub organization: Arc<dyn OrganizationMapperTrait>,
     pub organization_role_ref: Arc<dyn OrganizationRoleRefMapperTrait>
}

impl Mappers {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { 
            department: Arc::new(DepartmentMapper::new(db_conn.clone())),
            department_role_ref_mapper: Arc::new(DepartmentRoleRefMapper::new(db_conn.clone())),
            group: Arc::new(GroupMapper::new(db_conn.clone())),
            group_role_ref: Arc::new(GroupRoleRefMapper::new(db_conn.clone())),
            organization: Arc::new(OrganizationMapper::new(db_conn.clone())),
            organization_role_ref: Arc::new(OrganizationRoleRefMapper::new(db_conn.clone()))

         }
    }
}