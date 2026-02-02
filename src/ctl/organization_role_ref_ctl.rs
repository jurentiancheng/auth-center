use crate::pojo::organization_role_ref_pojo::*;
use crate::svc::organization_role_ref_svc::OrganizationRoleRefSvc;

crate::impl_controller!(
    OrganizationRoleRefCtl,
    OrganizationRoleRefSvc,
    OrganizationRoleRefCondition,
    OrganizationRoleRefVo,
    OrganizationRoleRefDto
);
