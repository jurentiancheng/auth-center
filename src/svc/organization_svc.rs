use crate::mapper::organization_mapper::OrganizationMapperTrait;
use crate::pojo::organization_pojo::*;

crate::impl_service!(
    OrganizationSvc,
    OrganizationMapperTrait,
    OrganizationCondition,
    OrganizationVo,
    OrganizationDto
);
