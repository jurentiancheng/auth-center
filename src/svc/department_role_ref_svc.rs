use crate::mapper::department_role_ref_mapper::{DepartmentRoleRefMapper, DepartmentRoleRefMapperTrait};
use crate::pojo::department_role_ref_pojo::*;

crate::impl_service!(
    DepartmentRoleRefSvc,
    DepartmentRoleRefMapper,
    DepartmentRoleRefMapperTrait,
    DepartmentRoleRefCondition,
    DepartmentRoleRefVo,
    DepartmentRoleRefDto
);
