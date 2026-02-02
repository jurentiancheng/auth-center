use crate::pojo::system_config_pojo::*;
use crate::svc::system_config_svc::SystemConfigSvc;

crate::impl_controller!(
    SystemConfigCtl,
    SystemConfigSvc,
    SystemConfigCondition,
    SystemConfigVo,
    SystemConfigDto
);
