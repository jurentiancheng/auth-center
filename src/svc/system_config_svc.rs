use crate::mapper::system_config_mapper::SystemConfigMapperTrait;
use crate::pojo::system_config_pojo::*;

crate::impl_service!(
    SystemConfigSvc,
    SystemConfigMapperTrait,
    SystemConfigCondition,
    SystemConfigVo,
    SystemConfigDto
);
