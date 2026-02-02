use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::organization_role_ref_pojo::*;

crate::define_mapper_trait!(OrganizationRoleRefMapperTrait, OrganizationRoleRefCondition, OrganizationRoleRefVo, OrganizationRoleRefDto);
crate::define_mapper_struct!(OrganizationRoleRefMapper);

impl OrganizationRoleRefMapper {
    fn build_query_wrapper(&self, condition: &OrganizationRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(organization_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(organization_role_ref::Column::Id.is_in(ids.clone()));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(organization_role_ref::Column::OrgCode.eq(org_code));
        }
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(organization_role_ref::Column::RoleCode.eq(role_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(OrganizationRoleRefMapper, OrganizationRoleRefMapperTrait, OrganizationRoleRef, organization_role_ref, OrganizationRoleRefCondition, OrganizationRoleRefVo, OrganizationRoleRefDto);
