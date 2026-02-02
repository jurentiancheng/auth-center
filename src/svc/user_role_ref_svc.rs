use crate::mapper::user_role_ref_mapper::{UserRoleRefMapper, UserRoleRefMapperTrait};
use crate::pojo::user_role_ref_pojo::*;

crate::impl_service!(
    UserRoleRefSvc,
    UserRoleRefMapper,
    UserRoleRefMapperTrait,
    UserRoleRefCondition,
    UserRoleRefVo,
    UserRoleRefDto
);
