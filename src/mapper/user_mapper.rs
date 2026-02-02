use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::user_pojo::*;

// 定义 Trait
crate::define_mapper_trait!(UserMapperTrait, UserCondition, UserVo, UserDto);

// 定义 Mapper 结构体
crate::define_mapper_struct!(UserMapper);

impl UserMapper {
    fn build_query_wrapper(&self, condition: &UserCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user::Column::Id.is_in(ids.clone()));
        }
        if let Some(user_name) = &condition.user_name {
            query_wrapper = query_wrapper.add(user::Column::UserName.eq(user_name));
        }
        if let Some(real_name) = &condition.real_name {
            query_wrapper = query_wrapper.add(user::Column::RealName.eq(real_name));
        }
        query_wrapper
    }
}

// 实现 CRUD 方法
crate::impl_mapper!(
    UserMapper,
    UserMapperTrait,
    User,
    user,
    UserCondition,
    UserVo,
    UserDto
);
