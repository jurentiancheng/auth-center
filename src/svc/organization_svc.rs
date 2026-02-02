use crate::mapper::organization_mapper::{OrganizationMapper, OrganizationMapperTrait};
use crate::pojo::organization_pojo::*;

crate::impl_service!(
    OrganizationSvc,
    OrganizationMapper,
    OrganizationMapperTrait,
    OrganizationCondition,
    OrganizationVo,
    OrganizationDto
);
