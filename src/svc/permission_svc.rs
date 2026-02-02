use crate::mapper::permission_mapper::{PermissionMapper, PermissionMapperTrait};
use crate::pojo::permission_pojo::*;

crate::impl_service!(
    PermissionSvc,
    PermissionMapper,
    PermissionMapperTrait,
    PermissionCondition,
    PermissionVo,
    PermissionDto
);
