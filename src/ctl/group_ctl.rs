use crate::pojo::group_pojo::*;
use crate::svc::group_svc::GroupSvc;

crate::impl_controller!(
    GroupCtl,
    GroupSvc,
    GroupCondition,
    GroupVo,
    GroupDto
);
