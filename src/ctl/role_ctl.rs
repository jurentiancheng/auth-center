use crate::pojo::role_pojo::*;
use crate::svc::role_svc::RoleSvc;

crate::impl_controller!(
    RoleCtl,
    RoleSvc,
    RoleCondition,
    RoleVo,
    RoleDto
);
