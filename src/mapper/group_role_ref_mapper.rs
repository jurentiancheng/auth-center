use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::group_role_ref_pojo::*;

crate::define_mapper_trait!(GroupRoleRefMapperTrait, GroupRoleRefCondition, GroupRoleRefVo, GroupRoleRefDto);
crate::define_mapper_struct!(GroupRoleRefMapper);

impl GroupRoleRefMapper {
    fn build_query_wrapper(&self, condition: &GroupRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(group_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(group_role_ref::Column::Id.is_in(ids.clone()));
        }
        if let Some(group_code) = &condition.group_code {
            query_wrapper = query_wrapper.add(group_role_ref::Column::GroupCode.eq(group_code));
        }
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(group_role_ref::Column::RoleCode.eq(role_code));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(group_role_ref::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(GroupRoleRefMapper, GroupRoleRefMapperTrait, GroupRoleRef, group_role_ref, GroupRoleRefCondition, GroupRoleRefVo, GroupRoleRefDto);
