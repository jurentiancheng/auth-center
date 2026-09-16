use crate::mapper::group_mapper::GroupMapperTrait;
use crate::pojo::group_pojo::*;

crate::impl_service!(
    GroupSvc,
    GroupMapperTrait,
    GroupCondition,
    GroupVo,
    GroupDto
);
