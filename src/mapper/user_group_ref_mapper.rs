use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::user_group_ref_pojo::*;

crate::define_mapper_trait!(UserGroupRefMapperTrait, UserGroupRefCondition, UserGroupRefVo, UserGroupRefDto);
crate::define_mapper_struct!(UserGroupRefMapper);

impl UserGroupRefMapper {
    fn build_query_wrapper(&self, condition: &UserGroupRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_group_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_group_ref::Column::Id.is_in(ids.clone()));
        }
        if let Some(user_code) = &condition.user_code {
            query_wrapper = query_wrapper.add(user_group_ref::Column::UserCode.eq(user_code));
        }
        if let Some(group_code) = &condition.group_code {
            query_wrapper = query_wrapper.add(user_group_ref::Column::GroupCode.eq(group_code));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(user_group_ref::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(UserGroupRefMapper, UserGroupRefMapperTrait, UserGroupRef, user_group_ref, UserGroupRefCondition, UserGroupRefVo, UserGroupRefDto);
