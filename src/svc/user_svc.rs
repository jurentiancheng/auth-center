use crate::mapper::user_mapper::UserMapperTrait;
use crate::pojo::user_pojo::*;

// 唯一需要业务逻辑的 CRUD 资源：密码落库前必须哈希。
// 钩子块跟在宏调用尾部 —— 既保留宏生成的整套 CRUD，又能插入业务逻辑，
// 不用把 60 行展开手写（这正是重构前这套宏断裂的地方）。
crate::impl_service!(
    UserSvc,
    UserMapperTrait,
    UserCondition,
    UserVo,
    UserDto,
    {
        /// 新建用户：明文密码换成 argon2 哈希。
        async fn before_save(&self, mut dto: UserDto) -> Result<UserDto, sea_orm::DbErr> {
            hash_password_in_place(&mut dto)?;
            Ok(dto)
        }

        /// 改用户：只有真的传了 password 才重新哈希；不传即视为不动该列
        /// （`UserDto` 上 `skip_serializing_if = "Option::is_none"` 保证了这一点）。
        async fn before_update(&self, mut dto: UserDto) -> Result<UserDto, sea_orm::DbErr> {
            hash_password_in_place(&mut dto)?;
            Ok(dto)
        }
    }
);

/// 把 `dto.password` 由明文替换为 PHC 哈希串。没传密码时什么都不做。
fn hash_password_in_place(dto: &mut UserDto) -> Result<(), sea_orm::DbErr> {
    let Some(plain) = dto.password.as_deref() else {
        return Ok(());
    };
    if plain.is_empty() {
        return Err(sea_orm::DbErr::Custom("密码不能为空".to_string()));
    }

    let hashed = crate::auth::password::hash_password(plain)
        .map_err(sea_orm::DbErr::Custom)?;
    dto.password = Some(hashed);
    Ok(())
}
