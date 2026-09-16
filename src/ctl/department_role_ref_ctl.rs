use crate::pojo::department_role_ref_pojo::*;
use crate::svc::department_role_ref_svc::DepartmentRoleRefSvc;

crate::impl_controller!(
    DepartmentRoleRefCtl,
    DepartmentRoleRefSvc,
    department_role_ref,
    DepartmentRoleRefCondition,
    DepartmentRoleRefVo,
    DepartmentRoleRefDto
);
