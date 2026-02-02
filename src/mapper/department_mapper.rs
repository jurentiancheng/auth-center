use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::department_pojo::*;

crate::define_mapper_trait!(DepartmentMapperTrait, DepartmentCondition, DepartmentVo, DepartmentDto);
crate::define_mapper_struct!(DepartmentMapper);

impl DepartmentMapper {
    fn build_query_wrapper(&self, condition: &DepartmentCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(department::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(department::Column::Id.is_in(ids.clone()));
        }
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(department::Column::Code.eq(code));
        }
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(department::Column::ParentCode.eq(parent_code));
        }
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(department::Column::Name.like(format!("%{}%", name)));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(department::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(DepartmentMapper, DepartmentMapperTrait, Department, department, DepartmentCondition, DepartmentVo, DepartmentDto);
