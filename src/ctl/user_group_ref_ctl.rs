use crate::pojo::user_group_ref_pojo::*;
use crate::svc::user_group_ref_svc::UserGroupRefSvc;

crate::impl_controller!(
    UserGroupRefCtl,
    UserGroupRefSvc,
    UserGroupRefCondition,
    UserGroupRefVo,
    UserGroupRefDto
);
