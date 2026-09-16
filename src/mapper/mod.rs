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
pub use organization_mapper::*;
pub use role_mapper::*;
pub use permission_mapper::*;
pub use position_mapper::*;
pub use department_mapper::*;
pub use group_mapper::*;
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

use std::sync::Arc;

use sea_orm::DatabaseConnection;

/// Mapper 容器 —— 依赖注入的装配点。
///
/// 启动时按连接池构造一次并由 `AppState` 持有，全部以 `Arc<dyn XxxTrait>` 暴露。
/// Service 与 Controller 只依赖 trait，因此测试里可以把任意一个换成假实现
/// （重构前 mapper/svc 是 `OnceCell` 进程级单例，首个 state 获胜，换不掉也测不了）。
///
/// 手写的 mapper（`AuthMapper`）也在这里登记，避免依赖装配散落各处。
pub struct Mappers {
    /// 手写：认证相关的领域查询
    pub auth: Arc<dyn crate::auth::mapper::AuthMapperTrait>,
    pub organization: Arc<dyn OrganizationMapperTrait>,
    pub role: Arc<dyn RoleMapperTrait>,
    pub permission: Arc<dyn PermissionMapperTrait>,
    pub position: Arc<dyn PositionMapperTrait>,
    pub department: Arc<dyn DepartmentMapperTrait>,
    pub group: Arc<dyn GroupMapperTrait>,
    pub system_config: Arc<dyn SystemConfigMapperTrait>,
    pub user_wechat_info: Arc<dyn UserWechatInfoMapperTrait>,
    pub user: Arc<dyn UserMapperTrait>,
    pub user_info: Arc<dyn UserInfoMapperTrait>,
    pub user_role_ref: Arc<dyn UserRoleRefMapperTrait>,
    pub user_group_ref: Arc<dyn UserGroupRefMapperTrait>,
    pub position_role_ref: Arc<dyn PositionRoleRefMapperTrait>,
    pub department_role_ref: Arc<dyn DepartmentRoleRefMapperTrait>,
    pub group_role_ref: Arc<dyn GroupRoleRefMapperTrait>,
    pub organization_role_ref: Arc<dyn OrganizationRoleRefMapperTrait>,
}

impl Mappers {
    pub fn new(pool: DatabaseConnection) -> Self {
        Self {
            auth: Arc::new(crate::auth::mapper::AuthMapper::new(pool.clone())),
            organization: Arc::new(OrganizationMapper::new(pool.clone())),
            role: Arc::new(RoleMapper::new(pool.clone())),
            permission: Arc::new(PermissionMapper::new(pool.clone())),
            position: Arc::new(PositionMapper::new(pool.clone())),
            department: Arc::new(DepartmentMapper::new(pool.clone())),
            group: Arc::new(GroupMapper::new(pool.clone())),
            system_config: Arc::new(SystemConfigMapper::new(pool.clone())),
            user_wechat_info: Arc::new(UserWechatInfoMapper::new(pool.clone())),
            user: Arc::new(UserMapper::new(pool.clone())),
            user_info: Arc::new(UserInfoMapper::new(pool.clone())),
            user_role_ref: Arc::new(UserRoleRefMapper::new(pool.clone())),
            user_group_ref: Arc::new(UserGroupRefMapper::new(pool.clone())),
            position_role_ref: Arc::new(PositionRoleRefMapper::new(pool.clone())),
            department_role_ref: Arc::new(DepartmentRoleRefMapper::new(pool.clone())),
            group_role_ref: Arc::new(GroupRoleRefMapper::new(pool.clone())),
            organization_role_ref: Arc::new(OrganizationRoleRefMapper::new(pool)),
        }
    }
}
