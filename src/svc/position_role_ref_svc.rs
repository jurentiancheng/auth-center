use crate::mapper::position_role_ref_mapper::PositionRoleRefMapperTrait;
use crate::pojo::position_role_ref_pojo::*;

crate::impl_service!(
    PositionRoleRefSvc,
    PositionRoleRefMapperTrait,
    PositionRoleRefCondition,
    PositionRoleRefVo,
    PositionRoleRefDto
);
