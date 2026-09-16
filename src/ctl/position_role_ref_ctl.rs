use crate::pojo::position_role_ref_pojo::*;
use crate::svc::position_role_ref_svc::PositionRoleRefSvc;

crate::impl_controller!(
    PositionRoleRefCtl,
    PositionRoleRefSvc,
    position_role_ref,
    PositionRoleRefCondition,
    PositionRoleRefVo,
    PositionRoleRefDto
);
