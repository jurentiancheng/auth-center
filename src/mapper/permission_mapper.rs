use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::permission_pojo::*;

crate::define_mapper_trait!(PermissionMapperTrait, PermissionCondition, PermissionVo, PermissionDto);
crate::define_mapper_struct!(PermissionMapper);

impl PermissionMapper {
    fn build_query_wrapper(&self, condition: &PermissionCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(permission::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(permission::Column::Id.is_in(ids.clone()));
        }
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(permission::Column::Code.eq(code));
        }
        if let Some(parent_uuid) = &condition.parent_uuid {
            query_wrapper = query_wrapper.add(permission::Column::ParentUuid.eq(parent_uuid));
        }
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(permission::Column::Name.eq(name));
        }
        if let Some(application) = &condition.application {
            query_wrapper = query_wrapper.add(permission::Column::Application.eq(application));
        }
        query_wrapper
    }
}

crate::impl_mapper!(PermissionMapper, PermissionMapperTrait, Permission, permission, PermissionCondition, PermissionVo, PermissionDto);
