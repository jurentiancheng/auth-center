use crate::mapper::position_role_ref_mapper::{PositionRoleRefMapper, PositionRoleRefMapperTrait};
use crate::pojo::position_role_ref_pojo::*;

crate::impl_service!(
    PositionRoleRefSvc,
    PositionRoleRefMapper,
    PositionRoleRefMapperTrait,
    PositionRoleRefCondition,
    PositionRoleRefVo,
    PositionRoleRefDto
);
