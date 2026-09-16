use crate::mapper::department_role_ref_mapper::DepartmentRoleRefMapperTrait;
use crate::pojo::department_role_ref_pojo::*;

crate::impl_service!(
    DepartmentRoleRefSvc,
    DepartmentRoleRefMapperTrait,
    DepartmentRoleRefCondition,
    DepartmentRoleRefVo,
    DepartmentRoleRefDto
);
