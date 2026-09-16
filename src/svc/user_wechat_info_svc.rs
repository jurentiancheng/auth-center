use crate::mapper::user_wechat_info_mapper::UserWechatInfoMapperTrait;
use crate::pojo::user_wechat_info_pojo::*;

crate::impl_service!(
    UserWechatInfoSvc,
    UserWechatInfoMapperTrait,
    UserWechatInfoCondition,
    UserWechatInfoVo,
    UserWechatInfoDto
);
