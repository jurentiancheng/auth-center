use crate::mapper::organization_role_ref_mapper::OrganizationRoleRefMapperTrait;
use crate::pojo::organization_role_ref_pojo::*;

crate::impl_service!(
    OrganizationRoleRefSvc,
    OrganizationRoleRefMapperTrait,
    OrganizationRoleRefCondition,
    OrganizationRoleRefVo,
    OrganizationRoleRefDto
);
