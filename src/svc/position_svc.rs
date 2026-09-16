use crate::mapper::position_mapper::PositionMapperTrait;
use crate::pojo::position_pojo::*;

crate::impl_service!(
    PositionSvc,
    PositionMapperTrait,
    PositionCondition,
    PositionVo,
    PositionDto
);
