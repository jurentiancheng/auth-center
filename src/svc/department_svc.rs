use crate::mapper::department_mapper::{DepartmentMapper, DepartmentMapperTrait};
use crate::pojo::department_pojo::*;

crate::impl_service!(
    DepartmentSvc,
    DepartmentMapper,
    DepartmentMapperTrait,
    DepartmentCondition,
    DepartmentVo,
    DepartmentDto
);
