use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::user_wechat_info_pojo::*;

crate::define_mapper_trait!(UserWechatInfoMapperTrait, UserWechatInfoCondition, UserWechatInfoVo, UserWechatInfoDto);
crate::define_mapper_struct!(UserWechatInfoMapper);

impl UserWechatInfoMapper {
    fn build_query_wrapper(&self, condition: &UserWechatInfoCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_wechat_info::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::Id.is_in(ids.clone()));
        }
        if let Some(union_id) = &condition.union_id {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::UnionId.eq(union_id));
        }
        if let Some(open_id) = &condition.mini_open_id {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::MiniOpenId.eq(open_id));
        }
        if let Some(nickname) = &condition.nickname {
            query_wrapper = query_wrapper.add(user_wechat_info::Column::Nickname.like(nickname));
        }
        query_wrapper
    }
}

crate::impl_mapper!(UserWechatInfoMapper, UserWechatInfoMapperTrait, UserWechatInfo, user_wechat_info, UserWechatInfoCondition, UserWechatInfoVo, UserWechatInfoDto);
