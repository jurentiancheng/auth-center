use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::position_pojo::*;

crate::define_mapper_trait!(PositionMapperTrait, PositionCondition, PositionVo, PositionDto);
crate::define_mapper_struct!(PositionMapper);

impl PositionMapper {
    fn build_query_wrapper(&self, condition: &PositionCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(position::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(position::Column::Id.is_in(ids.clone()));
        }
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(position::Column::Code.eq(code));
        }
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(position::Column::Name.eq(name));
        }
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(position::Column::OrgCode.eq(org_code));
        }
        query_wrapper
    }
}

crate::impl_mapper!(PositionMapper, PositionMapperTrait, Position, position, PositionCondition, PositionVo, PositionDto);
