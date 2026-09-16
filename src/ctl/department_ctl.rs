use crate::pojo::department_pojo::*;
use crate::svc::department_svc::DepartmentSvc;

crate::impl_controller!(
    DepartmentCtl,
    DepartmentSvc,
    department,
    DepartmentCondition,
    DepartmentVo,
    DepartmentDto
);
