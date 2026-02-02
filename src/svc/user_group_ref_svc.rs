use crate::mapper::user_group_ref_mapper::{UserGroupRefMapper, UserGroupRefMapperTrait};
use crate::pojo::user_group_ref_pojo::*;

crate::impl_service!(
    UserGroupRefSvc,
    UserGroupRefMapper,
    UserGroupRefMapperTrait,
    UserGroupRefCondition,
    UserGroupRefVo,
    UserGroupRefDto
);
