use crate::mapper::system_config_mapper::{SystemConfigMapper, SystemConfigMapperTrait};
use crate::pojo::system_config_pojo::*;

crate::impl_service!(
    SystemConfigSvc,
    SystemConfigMapper,
    SystemConfigMapperTrait,
    SystemConfigCondition,
    SystemConfigVo,
    SystemConfigDto
);
