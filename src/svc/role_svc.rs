use crate::mapper::role_mapper::{RoleMapper, RoleMapperTrait};
use crate::pojo::role_pojo::*;

crate::impl_service!(
    RoleSvc,
    RoleMapper,
    RoleMapperTrait,
    RoleCondition,
    RoleVo,
    RoleDto
);
