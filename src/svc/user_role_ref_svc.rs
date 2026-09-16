use crate::mapper::user_role_ref_mapper::UserRoleRefMapperTrait;
use crate::pojo::user_role_ref_pojo::*;

crate::impl_service!(
    UserRoleRefSvc,
    UserRoleRefMapperTrait,
    UserRoleRefCondition,
    UserRoleRefVo,
    UserRoleRefDto
);
