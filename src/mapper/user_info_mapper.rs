use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::user_info_pojo::*;

crate::define_mapper_trait!(UserInfoMapperTrait, UserInfoCondition, UserInfoVo, UserInfoDto);
crate::define_mapper_struct!(UserInfoMapper);

impl UserInfoMapper {
    fn build_query_wrapper(&self, condition: &UserInfoCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_info::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_info::Column::Id.is_in(ids.clone()));
        }
        if let Some(user_code) = &condition.user_code {
            query_wrapper = query_wrapper.add(user_info::Column::UserCode.eq(user_code));
        }
        if let Some(username) = &condition.username {
            query_wrapper = query_wrapper.add(user_info::Column::Username.eq(username));
        }
        if let Some(real_name) = &condition.real_name {
            query_wrapper = query_wrapper.add(user_info::Column::RealName.eq(real_name));
        }
        if let Some(nick_name) = &condition.nick_name {
            query_wrapper = query_wrapper.add(user_info::Column::NickName.eq(nick_name));
        }
        if let Some(cellphone) = &condition.cellphone {
            query_wrapper = query_wrapper.add(user_info::Column::Cellphone.eq(cellphone));
        }
        if let Some(gender) = &condition.gender {
            query_wrapper = query_wrapper.add(user_info::Column::Gender.eq(gender));
        }
        if let Some(user_type) = &condition.user_type {
            query_wrapper = query_wrapper.add(user_info::Column::UserType.eq(user_type));
        }
        if let Some(id_card_type) = &condition.id_card_type {
            query_wrapper = query_wrapper.add(user_info::Column::IdCardType.eq(id_card_type));
        }
        if let Some(id_card_no) = &condition.id_card_no {
            query_wrapper = query_wrapper.add(user_info::Column::IdCardNo.eq(id_card_no));
        }
        if let Some(email) = &condition.email {
            query_wrapper = query_wrapper.add(user_info::Column::Email.eq(email));
        }
        if let Some(qq) = &condition.qq {
            query_wrapper = query_wrapper.add(user_info::Column::Qq.eq(qq));
        }
        if let Some(wx_union_id) = &condition.wx_union_id {
            query_wrapper = query_wrapper.add(user_info::Column::WxUnionId.eq(wx_union_id));
        }
        if let Some(wx_open_id) = &condition.wx_open_id {
            query_wrapper = query_wrapper.add(user_info::Column::WxOpenId.eq(wx_open_id));
        }
        if let Some(wx_mini_open_id) = &condition.wx_mini_open_id {
            query_wrapper = query_wrapper.add(user_info::Column::WxMiniOpenId.eq(wx_mini_open_id));
        }
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(user_info::Column::ParentCode.eq(parent_code));
        }
        if let Some(path) = &condition.path {
            query_wrapper = query_wrapper.add(user_info::Column::Path.eq(path));
        }
        if let Some(status) = &condition.status {
            query_wrapper = query_wrapper.add(user_info::Column::Status.eq(status));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(user_info::Column::OrgCode.eq(org_code));
        }
        if let Some(department_code) = &condition.department_code {
            query_wrapper = query_wrapper.add(user_info::Column::DepartmentCode.eq(department_code));
        }
        if let Some(position_code) = &condition.position_code {
            query_wrapper = query_wrapper.add(user_info::Column::PositionCode.eq(position_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(UserInfoMapper, UserInfoMapperTrait, UserInfo, user_info, UserInfoCondition, UserInfoVo, UserInfoDto);
