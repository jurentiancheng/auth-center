use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::position_role_ref_pojo::*;

crate::define_mapper_trait!(PositionRoleRefMapperTrait, PositionRoleRefCondition, PositionRoleRefVo, PositionRoleRefDto);
crate::define_mapper_struct!(PositionRoleRefMapper);

impl PositionRoleRefMapper {
    fn build_query_wrapper(&self, condition: &PositionRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(position_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(position_role_ref::Column::Id.is_in(ids.clone()));
        }
        if let Some(position_code) = &condition.position_code {
            query_wrapper = query_wrapper.add(position_role_ref::Column::PositionCode.eq(position_code));
        }
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(position_role_ref::Column::RoleCode.eq(role_code));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(position_role_ref::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(PositionRoleRefMapper, PositionRoleRefMapperTrait, PositionRoleRef, position_role_ref, PositionRoleRefCondition, PositionRoleRefVo, PositionRoleRefDto);
