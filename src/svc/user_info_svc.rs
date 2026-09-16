use crate::mapper::user_info_mapper::UserInfoMapperTrait;
use crate::pojo::user_info_pojo::*;

crate::impl_service!(
    UserInfoSvc,
    UserInfoMapperTrait,
    UserInfoCondition,
    UserInfoVo,
    UserInfoDto
);
