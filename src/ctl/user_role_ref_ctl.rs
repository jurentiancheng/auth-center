use crate::pojo::user_role_ref_pojo::*;
use crate::svc::user_role_ref_svc::UserRoleRefSvc;

crate::impl_controller!(
    UserRoleRefCtl,
    UserRoleRefSvc,
    UserRoleRefCondition,
    UserRoleRefVo,
    UserRoleRefDto
);
