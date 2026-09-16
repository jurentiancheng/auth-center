use crate::pojo::user_pojo::*;
use crate::svc::user_svc::UserSvc;

crate::impl_controller!(
    UserCtl,
    UserSvc,
    user,
    UserCondition,
    UserVo,
    UserDto
);

impl UserCtl {
    pub async fn root() -> &'static str {
        "Hello, World!"
    }
}
