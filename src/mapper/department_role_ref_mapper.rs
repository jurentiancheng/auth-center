use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::department_role_ref_pojo::*;

crate::define_mapper_trait!(DepartmentRoleRefMapperTrait, DepartmentRoleRefCondition, DepartmentRoleRefVo, DepartmentRoleRefDto);
crate::define_mapper_struct!(DepartmentRoleRefMapper);

impl DepartmentRoleRefMapper {
    fn build_query_wrapper(&self, condition: &DepartmentRoleRefCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(department_role_ref::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(department_role_ref::Column::Id.is_in(ids.clone()));
        }
        if let Some(department_code) = &condition.department_code {
            query_wrapper = query_wrapper.add(department_role_ref::Column::DepartmentCode.eq(department_code));
        }
        if let Some(role_code) = &condition.role_code {
            query_wrapper = query_wrapper.add(department_role_ref::Column::RoleCode.eq(role_code));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(department_role_ref::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(DepartmentRoleRefMapper, DepartmentRoleRefMapperTrait, DepartmentRoleRef, department_role_ref, DepartmentRoleRefCondition, DepartmentRoleRefVo, DepartmentRoleRefDto);
