use crate::pojo::user_wechat_info_pojo::*;
use crate::svc::user_wechat_info_svc::UserWechatInfoSvc;

crate::impl_controller!(
    UserWechatInfoCtl,
    UserWechatInfoSvc,
    user_wechat_info,
    UserWechatInfoCondition,
    UserWechatInfoVo,
    UserWechatInfoDto
);
