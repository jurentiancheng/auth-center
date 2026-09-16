use crate::pojo::organization_pojo::*;
use crate::svc::organization_svc::OrganizationSvc;

crate::impl_controller!(
    OrganizationCtl,
    OrganizationSvc,
    organization,
    OrganizationCondition,
    OrganizationVo,
    OrganizationDto
);
