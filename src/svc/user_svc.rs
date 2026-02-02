use crate::mapper::user_mapper::{UserMapper, UserMapperTrait};
use crate::pojo::user_pojo::*;

crate::impl_service!(
    UserSvc,
    UserMapper,
    UserMapperTrait,
    UserCondition,
    UserVo,
    UserDto
);
