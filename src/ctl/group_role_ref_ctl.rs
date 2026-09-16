use crate::pojo::group_role_ref_pojo::*;
use crate::svc::group_role_ref_svc::GroupRoleRefSvc;

crate::impl_controller!(
    GroupRoleRefCtl,
    GroupRoleRefSvc,
    group_role_ref,
    GroupRoleRefCondition,
    GroupRoleRefVo,
    GroupRoleRefDto
);
