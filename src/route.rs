use crate::ctl;
use axum::{ routing::{get, post, put}, Router};
use std::error::Error;
use crate::init_status;

use ctl::{
    user_ctl::UserCtl,
    organization_ctl::OrganizationCtl,
    role_ctl::RoleCtl,
    permission_ctl::PermissionCtl,
    department_ctl::DepartmentCtl,
    position_ctl::PositionCtl,
    group_ctl::GroupCtl,
    user_info_ctl::UserInfoCtl,
    user_wechat_info_ctl::UserWechatInfoCtl,
    system_config_ctl::SystemConfigCtl,
    user_role_ref_ctl::UserRoleRefCtl,
    user_group_ref_ctl::UserGroupRefCtl,
    group_role_ref_ctl::GroupRoleRefCtl,
    department_role_ref_ctl::DepartmentRoleRefCtl,
    position_role_ref_ctl::PositionRoleRefCtl,
    organization_role_ref_ctl::OrganizationRoleRefCtl,
};


pub async fn build_app_route() -> Result<Router, Box<dyn Error>> {
    
    let app = Router::new()
        // User routes
        .route("/", get(UserCtl::root))
        .route(
            "/user",
            post(UserCtl::save)
                .put(UserCtl::update_by_id)
                .delete(UserCtl::remove_by_ids)
        )
        .route("/user/delByIds", put(UserCtl::delete_by_ids))
        .route("/user/list", get(UserCtl::list))
        .route("/user/page", get(UserCtl::page))
        .route("/user/:id", get(UserCtl::get_by_id))

        // Organization routes
        .route(
            "/organization",
            post(OrganizationCtl::save)
                .put(OrganizationCtl::update_by_id)
                .delete(OrganizationCtl::remove_by_ids)
        )
        .route("/organization/delByIds", put(OrganizationCtl::delete_by_ids))
        .route("/organization/list", get(OrganizationCtl::list))
        .route("/organization/page", get(OrganizationCtl::page))
        .route("/organization/:id", get(OrganizationCtl::get_by_id))

        // Role routes
        .route(
            "/role",
            post(RoleCtl::save)
                .put(RoleCtl::update_by_id)
                .delete(RoleCtl::remove_by_ids)
        )
        .route("/role/delByIds", put(RoleCtl::delete_by_ids))
        .route("/role/list", get(RoleCtl::list))
        .route("/role/page", get(RoleCtl::page))
        .route("/role/:id", get(RoleCtl::get_by_id))

        // Permission routes
        .route(
            "/permission",
            post(PermissionCtl::save)
                .put(PermissionCtl::update_by_id)
                .delete(PermissionCtl::remove_by_ids)
        )
        .route("/permission/delByIds", put(PermissionCtl::delete_by_ids))
        .route("/permission/list", get(PermissionCtl::list))
        .route("/permission/page", get(PermissionCtl::page))
        .route("/permission/:id", get(PermissionCtl::get_by_id))

        // Department routes
        .route(
            "/department",
            post(DepartmentCtl::save)
                .put(DepartmentCtl::update_by_id)
                .delete(DepartmentCtl::remove_by_ids)
        )
        .route("/department/delByIds", put(DepartmentCtl::delete_by_ids))
        .route("/department/list", get(DepartmentCtl::list))
        .route("/department/page", get(DepartmentCtl::page))
        .route("/department/:id", get(DepartmentCtl::get_by_id))

        // Position routes
        .route(
            "/position",
            post(PositionCtl::save)
                .put(PositionCtl::update_by_id)
                .delete(PositionCtl::remove_by_ids)
        )
        .route("/position/delByIds", put(PositionCtl::delete_by_ids))
        .route("/position/list", get(PositionCtl::list))
        .route("/position/page", get(PositionCtl::page))
        .route("/position/:id", get(PositionCtl::get_by_id))

        // Group routes
        .route(
            "/group",
            post(GroupCtl::save)
                .put(GroupCtl::update_by_id)
                .delete(GroupCtl::remove_by_ids)
        )
        .route("/group/delByIds", put(GroupCtl::delete_by_ids))
        .route("/group/list", get(GroupCtl::list))
        .route("/group/page", get(GroupCtl::page))
        .route("/group/:id", get(GroupCtl::get_by_id))

        // UserInfo routes
        .route(
            "/userInfo",
            post(UserInfoCtl::save)
                .put(UserInfoCtl::update_by_id)
                .delete(UserInfoCtl::remove_by_ids)
        )
        .route("/userInfo/delByIds", put(UserInfoCtl::delete_by_ids))
        .route("/userInfo/list", get(UserInfoCtl::list))
        .route("/userInfo/page", get(UserInfoCtl::page))
        .route("/userInfo/:id", get(UserInfoCtl::get_by_id))

        // UserWechatInfo routes
        .route(
            "/userWechatInfo",
            post(UserWechatInfoCtl::save)
                .put(UserWechatInfoCtl::update_by_id)
                .delete(UserWechatInfoCtl::remove_by_ids)
        )
        .route("/userWechatInfo/delByIds", put(UserWechatInfoCtl::delete_by_ids))
        .route("/userWechatInfo/list", get(UserWechatInfoCtl::list))
        .route("/userWechatInfo/page", get(UserWechatInfoCtl::page))
        .route("/userWechatInfo/:id", get(UserWechatInfoCtl::get_by_id))

        // SystemConfig routes
        .route(
            "/systemConfig",
            post(SystemConfigCtl::save)
                .put(SystemConfigCtl::update_by_id)
                .delete(SystemConfigCtl::remove_by_ids)
        )
        .route("/systemConfig/delByIds", put(SystemConfigCtl::delete_by_ids))
        .route("/systemConfig/list", get(SystemConfigCtl::list))
        .route("/systemConfig/page", get(SystemConfigCtl::page))
        .route("/systemConfig/:id", get(SystemConfigCtl::get_by_id))

        // UserRoleRef routes
        .route(
            "/userRoleRef",
            post(UserRoleRefCtl::save)
                .put(UserRoleRefCtl::update_by_id)
                .delete(UserRoleRefCtl::remove_by_ids)
        )
        .route("/userRoleRef/delByIds", put(UserRoleRefCtl::delete_by_ids))
        .route("/userRoleRef/list", get(UserRoleRefCtl::list))
        .route("/userRoleRef/page", get(UserRoleRefCtl::page))
        .route("/userRoleRef/:id", get(UserRoleRefCtl::get_by_id))

        // UserGroupRef routes
        .route(
            "/userGroupRef",
            post(UserGroupRefCtl::save)
                .put(UserGroupRefCtl::update_by_id)
                .delete(UserGroupRefCtl::remove_by_ids)
        )
        .route("/userGroupRef/delByIds", put(UserGroupRefCtl::delete_by_ids))
        .route("/userGroupRef/list", get(UserGroupRefCtl::list))
        .route("/userGroupRef/page", get(UserGroupRefCtl::page))
        .route("/userGroupRef/:id", get(UserGroupRefCtl::get_by_id))

        // GroupRoleRef routes
        .route(
            "/groupRoleRef",
            post(GroupRoleRefCtl::save)
                .put(GroupRoleRefCtl::update_by_id)
                .delete(GroupRoleRefCtl::remove_by_ids)
        )
        .route("/groupRoleRef/delByIds", put(GroupRoleRefCtl::delete_by_ids))
        .route("/groupRoleRef/list", get(GroupRoleRefCtl::list))
        .route("/groupRoleRef/page", get(GroupRoleRefCtl::page))
        .route("/groupRoleRef/:id", get(GroupRoleRefCtl::get_by_id))

        // DepartmentRoleRef routes
        .route(
            "/departmentRoleRef",
            post(DepartmentRoleRefCtl::save)
                .put(DepartmentRoleRefCtl::update_by_id)
                .delete(DepartmentRoleRefCtl::remove_by_ids)
        )
        .route("/departmentRoleRef/delByIds", put(DepartmentRoleRefCtl::delete_by_ids))
        .route("/departmentRoleRef/list", get(DepartmentRoleRefCtl::list))
        .route("/departmentRoleRef/page", get(DepartmentRoleRefCtl::page))
        .route("/departmentRoleRef/:id", get(DepartmentRoleRefCtl::get_by_id))

        // PositionRoleRef routes
        .route(
            "/positionRoleRef",
            post(PositionRoleRefCtl::save)
                .put(PositionRoleRefCtl::update_by_id)
                .delete(PositionRoleRefCtl::remove_by_ids)
        )
        .route("/positionRoleRef/delByIds", put(PositionRoleRefCtl::delete_by_ids))
        .route("/positionRoleRef/list", get(PositionRoleRefCtl::list))
        .route("/positionRoleRef/page", get(PositionRoleRefCtl::page))
        .route("/positionRoleRef/:id", get(PositionRoleRefCtl::get_by_id))

        // OrganizationRoleRef routes
        .route(
            "/organizationRoleRef",
            post(OrganizationRoleRefCtl::save)
                .put(OrganizationRoleRefCtl::update_by_id)
                .delete(OrganizationRoleRefCtl::remove_by_ids)
        )
        .route("/organizationRoleRef/delByIds", put(OrganizationRoleRefCtl::delete_by_ids))
        .route("/organizationRoleRef/list", get(OrganizationRoleRefCtl::list))
        .route("/organizationRoleRef/page", get(OrganizationRoleRefCtl::page))
        .route("/organizationRoleRef/:id", get(OrganizationRoleRefCtl::get_by_id))

        .with_state(init_status().await?);
    Ok(app)
}