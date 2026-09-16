use crate::mapper::department_mapper::DepartmentMapperTrait;
use crate::pojo::department_pojo::*;

crate::impl_service!(
    DepartmentSvc,
    DepartmentMapperTrait,
    DepartmentCondition,
    DepartmentVo,
    DepartmentDto
);
