use crate::mapper::user_info_mapper::{UserInfoMapper, UserInfoMapperTrait};
use crate::pojo::user_info_pojo::*;

crate::impl_service!(
    UserInfoSvc,
    UserInfoMapper,
    UserInfoMapperTrait,
    UserInfoCondition,
    UserInfoVo,
    UserInfoDto
);
