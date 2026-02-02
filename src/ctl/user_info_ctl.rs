use crate::pojo::user_info_pojo::*;
use crate::svc::user_info_svc::UserInfoSvc;

crate::impl_controller!(
    UserInfoCtl,
    UserInfoSvc,
    UserInfoCondition,
    UserInfoVo,
    UserInfoDto
);
