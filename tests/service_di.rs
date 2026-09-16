//! 证明「依赖注入 + 业务钩子」在**外部 crate** 里真的可用。
//!
//! 这些断言在重构前一条都写不出来：那时 mapper/svc 由 `OnceCell` 持有进程级单例
//! （首个 `AppState` 获胜），Service 字段是 `&'static XxxMapper`，而构造 `AppState`
//! 又必须先连上真库 —— 既换不掉依赖，也写不了不连库的测试。
//!
//! 现在 `XxxSvc::new(Arc<dyn XxxMapperTrait>)` 可以直接塞假实现，全程不碰数据库。

use std::sync::{Arc, Mutex};

use auth_center::mapper::{RoleMapperTrait, UserMapperTrait};
use auth_center::pojo::role_pojo::{RoleCondition, RoleDto, RoleVo};
use auth_center::pojo::user_pojo::{UserCondition, UserDto, UserVo};
use auth_center::svc::{RoleSvc, UserSvc};
use auth_center::util::paged_struct::{PageData, PageInfo};
use sea_orm::DbErr;

// ==================== 用户：带钩子（密码哈希） ====================

/// 记录 Service 递进来的 DTO，用来断言钩子到底改了什么
#[derive(Default)]
struct UserRecorder {
    dto: Mutex<Vec<UserDto>>,
}

struct FakeUserMapper {
    recorder: Arc<UserRecorder>,
}

#[async_trait::async_trait]
impl UserMapperTrait for FakeUserMapper {
    async fn list(&self, _condition: UserCondition) -> Result<Vec<UserVo>, DbErr> {
        Ok(vec![])
    }

    async fn page(&self, _condition: UserCondition) -> Result<PageData<UserVo>, DbErr> {
        Ok(PageData::new(PageInfo::from(1, 20, 0), vec![]))
    }

    async fn get_by_id(&self, _rec_id: i64) -> Result<Option<UserVo>, DbErr> {
        Ok(None)
    }

    async fn save(&self, dto: UserDto) -> Result<i64, DbErr> {
        self.recorder.dto.lock().unwrap().push(dto);
        Ok(123)
    }

    async fn update_by_id(&self, dto: UserDto) -> Result<u64, DbErr> {
        self.recorder.dto.lock().unwrap().push(dto);
        Ok(1)
    }

    async fn delete_by_ids(&self, _dto: UserDto) -> Result<u64, DbErr> {
        Ok(0)
    }

    async fn remove_by_ids(&self, _dto: UserDto) -> Result<u64, DbErr> {
        Ok(0)
    }
}

/// 构造一个完全脱离数据库的 UserSvc
fn user_svc() -> (UserSvc, Arc<UserRecorder>) {
    let recorder = Arc::new(UserRecorder::default());
    let mapper: Arc<dyn UserMapperTrait> = Arc::new(FakeUserMapper {
        recorder: recorder.clone(),
    });
    (UserSvc::new(mapper), recorder)
}

#[tokio::test]
async fn before_save_hook_replaces_plaintext_password_with_hash() {
    let (svc, recorder) = user_svc();

    let dto = UserDto {
        user_name: Some("alice".into()),
        password: Some("plain-password".into()),
        ..Default::default()
    };
    let rec_id = svc.save(dto).await.expect("save 应成功");

    assert_eq!(rec_id, 123, "应把 mapper 返回的主键透传出去");

    let recorded = recorder.dto.lock().unwrap();
    let stored = recorded[0]
        .password
        .as_deref()
        .expect("钩子应写入哈希后的密码");

    assert_ne!(stored, "plain-password", "落库的密码不能是明文");
    assert!(
        stored.starts_with("$argon2"),
        "应是 argon2 PHC 哈希串，实际：{stored}"
    );
    assert!(
        auth_center::auth::password::verify_password("plain-password", stored),
        "哈希必须能用原密码校验通过"
    );
}

#[tokio::test]
async fn before_update_hook_rehashes_only_when_password_present() {
    let (svc, recorder) = user_svc();

    // 场景一：改别的字段、没传密码 —— 不能凭空造一个密码覆盖原值
    svc.update_by_id(UserDto {
        rec_id: Some(7),
        real_name: Some("爱丽丝".into()),
        ..Default::default()
    })
    .await
    .expect("update 应成功");

    // 场景二：传了密码 —— 必须重新哈希
    svc.update_by_id(UserDto {
        rec_id: Some(7),
        password: Some("new-password".into()),
        ..Default::default()
    })
    .await
    .expect("update 应成功");

    let recorded = recorder.dto.lock().unwrap();
    assert_eq!(recorded[0].password, None, "没传密码时不应写入 password");
    assert_eq!(recorded[0].real_name.as_deref(), Some("爱丽丝"));
    assert!(
        recorded[1]
            .password
            .as_deref()
            .is_some_and(|p| p.starts_with("$argon2")),
        "传了密码时应被哈希"
    );
}

#[tokio::test]
async fn empty_password_is_rejected() {
    let (svc, _recorder) = user_svc();

    let dto = UserDto {
        user_name: Some("bob".into()),
        password: Some(String::new()),
        ..Default::default()
    };

    assert!(
        svc.save(dto).await.is_err(),
        "空密码应被拒绝，而不是存一个空哈希"
    );
}

// ==================== 角色：无业务逻辑，默认钩子必须是空操作 ====================

struct FakeRoleMapper {
    saved: Mutex<Vec<RoleDto>>,
}

#[async_trait::async_trait]
impl RoleMapperTrait for FakeRoleMapper {
    async fn list(&self, _condition: RoleCondition) -> Result<Vec<RoleVo>, DbErr> {
        Ok(vec![])
    }

    async fn page(&self, _condition: RoleCondition) -> Result<PageData<RoleVo>, DbErr> {
        Ok(PageData::new(PageInfo::from(1, 20, 0), vec![]))
    }

    async fn get_by_id(&self, _rec_id: i64) -> Result<Option<RoleVo>, DbErr> {
        Ok(None)
    }

    async fn save(&self, dto: RoleDto) -> Result<i64, DbErr> {
        self.saved.lock().unwrap().push(dto);
        Ok(9)
    }

    async fn update_by_id(&self, dto: RoleDto) -> Result<u64, DbErr> {
        self.saved.lock().unwrap().push(dto);
        Ok(1)
    }

    async fn delete_by_ids(&self, _dto: RoleDto) -> Result<u64, DbErr> {
        Ok(0)
    }

    async fn remove_by_ids(&self, _dto: RoleDto) -> Result<u64, DbErr> {
        Ok(0)
    }
}

#[tokio::test]
async fn service_without_hook_block_passes_dto_through_unchanged() {
    // `RoleSvc` 的宏调用没写钩子块 —— 全部落在 `ServiceHooks` 的默认空实现上，
    // 行为与引入钩子机制之前完全一致。
    let fake = Arc::new(FakeRoleMapper {
        saved: Mutex::new(vec![]),
    });
    let mapper: Arc<dyn RoleMapperTrait> = fake.clone();
    let svc = RoleSvc::new(mapper);

    let rec_id = svc
        .save(RoleDto {
            code: Some("ADMIN".into()),
            name: Some("管理员".into()),
            role_type: Some(1),
            ..Default::default()
        })
        .await
        .expect("save 应成功");

    assert_eq!(rec_id, 9);

    let saved = fake.saved.lock().unwrap();
    assert_eq!(saved[0].code.as_deref(), Some("ADMIN"), "DTO 应原样抵达 mapper");
    assert_eq!(saved[0].name.as_deref(), Some("管理员"));
    assert_eq!(saved[0].role_type, Some(1));
    assert_eq!(saved[0].rec_id, None, "默认钩子不应改动任何字段");
}
