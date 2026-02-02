use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::group_pojo::*;

crate::define_mapper_trait!(GroupMapperTrait, GroupCondition, GroupVo, GroupDto);
crate::define_mapper_struct!(GroupMapper);

impl GroupMapper {
    fn build_query_wrapper(&self, condition: &GroupCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(group::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(group::Column::Id.is_in(ids.clone()));
        }
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(group::Column::Code.eq(code));
        }
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(group::Column::Name.eq(name));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(group::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(GroupMapper, GroupMapperTrait, Group, group, GroupCondition, GroupVo, GroupDto);
