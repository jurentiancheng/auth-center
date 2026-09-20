pub mod department_role_ref_svc;
pub mod department_svc;
pub mod group_role_ref_svc;
pub mod group_svc;
pub mod organization_role_ref_svc;
pub mod organization_svc;
pub mod permission_svc;
pub mod position_role_ref_svc;
pub mod position_svc;
pub mod role_svc;
pub mod system_config_svc;
pub mod user_group_ref_svc;
pub mod user_info_svc;
pub mod user_role_ref_svc;
pub mod user_svc;
pub mod user_wechat_info_svc;

pub use department_role_ref_svc::DepartmentRoleRefSvc;
pub use department_svc::DepartmentSvc;
pub use group_role_ref_svc::GroupRoleRefSvc;
pub use group_svc::GroupSvc;
pub use organization_role_ref_svc::OrganizationRoleRefSvc;
pub use organization_svc::OrganizationSvc;
pub use permission_svc::PermissionSvc;
pub use position_role_ref_svc::PositionRoleRefSvc;
pub use position_svc::PositionSvc;
pub use role_svc::RoleSvc;
pub use system_config_svc::SystemConfigSvc;
pub use user_group_ref_svc::UserGroupRefSvc;
pub use user_info_svc::UserInfoSvc;
pub use user_role_ref_svc::UserRoleRefSvc;
pub use user_svc::UserSvc;
pub use user_wechat_info_svc::UserWechatInfoSvc;

use std::sync::Arc;

use crate::mapper::Mappers;

/// 服务层聚合容器，字段与 [`Mappers`] 一一对应。
///
/// 在启动时由 [`Svcs::new`] 构建一次并挂到 `AppState` 上，控制器直接通过
/// `state.svcs.xxx` 调用，不再按请求构造。svc 依赖发生变化时只需修改
/// [`Svcs::new`] 中的接线，控制器无需改动。
#[derive(Clone)]
pub struct Svcs {
    pub department: Arc<DepartmentSvc>,
    pub department_role_ref: Arc<DepartmentRoleRefSvc>,
    pub group: Arc<GroupSvc>,
    pub group_role_ref: Arc<GroupRoleRefSvc>,
    pub organization: Arc<OrganizationSvc>,
    pub organization_role_ref: Arc<OrganizationRoleRefSvc>,
    pub permission: Arc<PermissionSvc>,
    pub position: Arc<PositionSvc>,
    pub position_role_ref: Arc<PositionRoleRefSvc>,
    pub role: Arc<RoleSvc>,
    pub system_config: Arc<SystemConfigSvc>,
    pub user_group_ref: Arc<UserGroupRefSvc>,
    pub user_info: Arc<UserInfoSvc>,
    pub user: Arc<UserSvc>,
    pub user_role_ref: Arc<UserRoleRefSvc>,
    pub user_wechat_info: Arc<UserWechatInfoSvc>,
}

impl Svcs {
    pub fn new(mappers: &Mappers) -> Self {
        Self {
            department: Arc::new(DepartmentSvc::new(mappers.department.clone())),
            department_role_ref: Arc::new(DepartmentRoleRefSvc::new(
                mappers.department_role_ref.clone(),
            )),
            group: Arc::new(GroupSvc::new(mappers.group.clone())),
            group_role_ref: Arc::new(GroupRoleRefSvc::new(mappers.group_role_ref.clone())),
            organization: Arc::new(OrganizationSvc::new(mappers.organization.clone())),
            organization_role_ref: Arc::new(OrganizationRoleRefSvc::new(
                mappers.organization_role_ref.clone(),
            )),
            permission: Arc::new(PermissionSvc::new(mappers.permission.clone())),
            position: Arc::new(PositionSvc::new(mappers.position.clone())),
            position_role_ref: Arc::new(PositionRoleRefSvc::new(mappers.position_role_ref.clone())),
            role: Arc::new(RoleSvc::new(mappers.role.clone())),
            system_config: Arc::new(SystemConfigSvc::new(mappers.system_config.clone())),
            user_group_ref: Arc::new(UserGroupRefSvc::new(mappers.user_group_ref.clone())),
            user_info: Arc::new(UserInfoSvc::new(mappers.user_info.clone())),
            user: Arc::new(UserSvc::new(mappers.user.clone())),
            user_role_ref: Arc::new(UserRoleRefSvc::new(mappers.user_role_ref.clone())),
            user_wechat_info: Arc::new(UserWechatInfoSvc::new(mappers.user_wechat_info.clone())),
        }
    }
}
