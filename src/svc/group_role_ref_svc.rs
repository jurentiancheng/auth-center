use crate::mapper::group_role_ref_mapper::{GroupRoleRefMapper, GroupRoleRefMapperTrait};
use crate::pojo::group_role_ref_pojo::*;

crate::impl_service!(
    GroupRoleRefSvc,
    GroupRoleRefMapper,
    GroupRoleRefMapperTrait,
    GroupRoleRefCondition,
    GroupRoleRefVo,
    GroupRoleRefDto
);
