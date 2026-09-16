use crate::mapper::permission_mapper::PermissionMapperTrait;
use crate::pojo::permission_pojo::*;

crate::impl_service!(
    PermissionSvc,
    PermissionMapperTrait,
    PermissionCondition,
    PermissionVo,
    PermissionDto
);
