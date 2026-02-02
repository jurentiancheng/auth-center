use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::user_role_ref_pojo::*;

crate::define_mapper_trait!(UserRoleRefMapperTrait, UserRoleRefCondition, UserRoleRefVo, UserRoleRefDto);
crate::define_mapper_struct!(UserRoleRefMapper);

impl UserRoleRefMapper {
    fn build_query_wrapper(&self, condition: &UserRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_role_ref::Column::Id.is_in(ids.clone()));
        }
        if let Some(user_code) = &condition.user_code {
            query_wrapper = query_wrapper.add(user_role_ref::Column::UserCode.eq(user_code));
        }
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(user_role_ref::Column::RoleCode.eq(role_code));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(user_role_ref::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(UserRoleRefMapper, UserRoleRefMapperTrait, UserRoleRef, user_role_ref, UserRoleRefCondition, UserRoleRefVo, UserRoleRefDto);
