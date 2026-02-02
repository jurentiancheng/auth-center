use crate::pojo::position_pojo::*;
use crate::svc::position_svc::PositionSvc;

crate::impl_controller!(
    PositionCtl,
    PositionSvc,
    PositionCondition,
    PositionVo,
    PositionDto
);
