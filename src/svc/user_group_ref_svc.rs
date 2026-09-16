use crate::mapper::user_group_ref_mapper::UserGroupRefMapperTrait;
use crate::pojo::user_group_ref_pojo::*;

crate::impl_service!(
    UserGroupRefSvc,
    UserGroupRefMapperTrait,
    UserGroupRefCondition,
    UserGroupRefVo,
    UserGroupRefDto
);
