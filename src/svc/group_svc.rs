use crate::mapper::group_mapper::{GroupMapper, GroupMapperTrait};
use crate::pojo::group_pojo::*;

crate::impl_service!(
    GroupSvc,
    GroupMapper,
    GroupMapperTrait,
    GroupCondition,
    GroupVo,
    GroupDto
);
