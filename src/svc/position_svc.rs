use crate::mapper::position_mapper::{PositionMapper, PositionMapperTrait};
use crate::pojo::position_pojo::*;

crate::impl_service!(
    PositionSvc,
    PositionMapper,
    PositionMapperTrait,
    PositionCondition,
    PositionVo,
    PositionDto
);
