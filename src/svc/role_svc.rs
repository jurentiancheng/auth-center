use crate::mapper::role_mapper::RoleMapperTrait;
use crate::pojo::role_pojo::*;

crate::impl_service!(
    RoleSvc,
    RoleMapperTrait,
    RoleCondition,
    RoleVo,
    RoleDto
);
