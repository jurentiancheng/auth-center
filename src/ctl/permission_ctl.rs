use crate::pojo::permission_pojo::*;
use crate::svc::permission_svc::PermissionSvc;

crate::impl_controller!(
    PermissionCtl,
    PermissionSvc,
    PermissionCondition,
    PermissionVo,
    PermissionDto
);
