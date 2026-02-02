use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::system_config_pojo::*;

crate::define_mapper_trait!(SystemConfigMapperTrait, SystemConfigCondition, SystemConfigVo, SystemConfigDto);
crate::define_mapper_struct!(SystemConfigMapper);

impl SystemConfigMapper {
    fn build_query_wrapper(&self, condition: &SystemConfigCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(system_config::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(system_config::Column::Id.is_in(ids.clone()));
        }
        if let Some(key) = &condition.config_key {
            query_wrapper = query_wrapper.add(system_config::Column::ConfigKey.eq(key));
        }
        if let Some(config_type) = &condition.config_type {
            query_wrapper = query_wrapper.add(system_config::Column::ConfigType.eq(config_type));
        }
        query_wrapper
    }
}

crate::impl_mapper!(SystemConfigMapper, SystemConfigMapperTrait, SystemConfig, system_config, SystemConfigCondition, SystemConfigVo, SystemConfigDto);
