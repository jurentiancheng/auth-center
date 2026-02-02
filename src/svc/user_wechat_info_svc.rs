use crate::mapper::user_wechat_info_mapper::{UserWechatInfoMapper, UserWechatInfoMapperTrait};
use crate::pojo::user_wechat_info_pojo::*;

crate::impl_service!(
    UserWechatInfoSvc,
    UserWechatInfoMapper,
    UserWechatInfoMapperTrait,
    UserWechatInfoCondition,
    UserWechatInfoVo,
    UserWechatInfoDto
);
