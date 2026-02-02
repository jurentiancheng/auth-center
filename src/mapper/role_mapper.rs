use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::role_pojo::*;

crate::define_mapper_trait!(RoleMapperTrait, RoleCondition, RoleVo, RoleDto);
crate::define_mapper_struct!(RoleMapper);

impl RoleMapper {
    fn build_query_wrapper(&self, condition: &RoleCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(role::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(role::Column::Id.is_in(ids.clone()));
        }
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(role::Column::Code.eq(code));
        }
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(role::Column::ParentCode.eq(parent_code));
        }
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(role::Column::Name.eq(name));
        }
        if let Some(application) = &condition.application {
            query_wrapper = query_wrapper.add(role::Column::Application.eq(application));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(role::Column::OrgCode.eq(org_code));
        }
        if let Some(role_type) = &condition.role_type {
            query_wrapper = query_wrapper.add(role::Column::RoleType.eq(*role_type));
        }
        query_wrapper
    }
}

crate::impl_mapper!(RoleMapper, RoleMapperTrait, Role, role, RoleCondition, RoleVo, RoleDto);
