use crate::mapper::group_role_ref_mapper::GroupRoleRefMapperTrait;
use crate::pojo::group_role_ref_pojo::*;

crate::impl_service!(
    GroupRoleRefSvc,
    GroupRoleRefMapperTrait,
    GroupRoleRefCondition,
    GroupRoleRefVo,
    GroupRoleRefDto
);
