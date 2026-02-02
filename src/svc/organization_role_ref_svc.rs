use crate::mapper::organization_role_ref_mapper::{OrganizationRoleRefMapper, OrganizationRoleRefMapperTrait};
use crate::pojo::organization_role_ref_pojo::*;

crate::impl_service!(
    OrganizationRoleRefSvc,
    OrganizationRoleRefMapper,
    OrganizationRoleRefMapperTrait,
    OrganizationRoleRefCondition,
    OrganizationRoleRefVo,
    OrganizationRoleRefDto
);
