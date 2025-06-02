use anyhow::Result;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Cond;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
    QueryTrait,
};
use std::sync::Arc;
use tracing::info;
use once_cell::sync::OnceCell;

use crate::entities::{prelude::*, *};
use crate::util::paged_struct::{PageData, PageInfo, Pageable};
use crate::util::IntoJsonValue;
use crate::{pojo::user_info_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for user info-related database operations
#[async_trait::async_trait]
pub trait UserInfoMapperTrait {
    async fn list(&self, condition: UserInfoCondition) -> Result<Vec<UserInfoVo>, DbErr>;
    async fn page(&self, condition: UserInfoCondition) -> Result<PageData<UserInfoVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserInfoVo>, DbErr>;
    async fn save(&self, user_info_dto: UserInfoDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr>;
}

/// Implementation of UserInfoMapperTrait
pub struct UserInfoMapper {
    state: Arc<AppState>,
}

impl UserInfoMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static UserInfoMapper {
        static INSTANCE: OnceCell<UserInfoMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| UserInfoMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &UserInfoCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(user_info::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(user_info::Column::Id.is_in(ids.clone()));
        };
        if let Some(user_code) = &condition.user_code {
            query_wrapper = query_wrapper.add(user_info::Column::UserCode.eq(user_code));
        };
        if let Some(username) = &condition.username {
            query_wrapper = query_wrapper.add(user_info::Column::Username.eq(username));
        };
        if let Some(real_name) = &condition.real_name {
            query_wrapper = query_wrapper.add(user_info::Column::RealName.eq(real_name));
        };
        if let Some(nick_name) = &condition.nick_name {
            query_wrapper = query_wrapper.add(user_info::Column::NickName.eq(nick_name));
        };
        if let Some(cellphone) = &condition.cellphone {
            query_wrapper = query_wrapper.add(user_info::Column::Cellphone.eq(cellphone));
        };
        if let Some(gender) = &condition.gender {
            query_wrapper = query_wrapper.add(user_info::Column::Gender.eq(gender));
        };
        if let Some(user_type) = &condition.user_type {
            query_wrapper = query_wrapper.add(user_info::Column::UserType.eq(user_type));
        };
        if let Some(id_card_type) = &condition.id_card_type {
            query_wrapper = query_wrapper.add(user_info::Column::IdCardType.eq(id_card_type));
        };
        if let Some(id_card_no) = &condition.id_card_no {
            query_wrapper = query_wrapper.add(user_info::Column::IdCardNo.eq(id_card_no));
        };
        if let Some(email) = &condition.email {
            query_wrapper = query_wrapper.add(user_info::Column::Email.eq(email));
        };
        if let Some(qq) = &condition.qq {
            query_wrapper = query_wrapper.add(user_info::Column::Qq.eq(qq));
        };
        if let Some(wx_union_id) = &condition.wx_union_id {
            query_wrapper = query_wrapper.add(user_info::Column::WxUnionId.eq(wx_union_id));
        };
        if let Some(wx_open_id) = &condition.wx_open_id {
            query_wrapper = query_wrapper.add(user_info::Column::WxOpenId.eq(wx_open_id));
        };
        if let Some(wx_mini_open_id) = &condition.wx_mini_open_id {
            query_wrapper = query_wrapper.add(user_info::Column::WxMiniOpenId.eq(wx_mini_open_id));
        };
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(user_info::Column::ParentCode.eq(parent_code));
        };
        if let Some(path) = &condition.path {
            query_wrapper = query_wrapper.add(user_info::Column::Path.eq(path));
        };
        if let Some(status) = &condition.status {
            query_wrapper = query_wrapper.add(user_info::Column::Status.eq(status));
        };
        if let Some(org_code) = &condition.org_code {
            query_wrapper = query_wrapper.add(user_info::Column::OrgCode.eq(org_code));
        };
        if let Some(department_code) = &condition.department_code {
            query_wrapper = query_wrapper.add(user_info::Column::DepartmentCode.eq(department_code));
        };
        if let Some(position_code) = &condition.position_code {
            query_wrapper = query_wrapper.add(user_info::Column::PositionCode.eq(position_code));
        };


        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &UserInfoCondition,
        list: Vec<UserInfoVo>,
        total: u64,
    ) -> Result<PageData<UserInfoVo>, DbErr> {
        let page_info = PageInfo::from(
            condition.get_page().unwrap(),
            condition.get_size().unwrap(),
            total,
        );
        let page_data = PageData::new(page_info, list);
        Ok(page_data)
    }
}

#[async_trait::async_trait]
impl UserInfoMapperTrait for UserInfoMapper {
    async fn list(&self, condition: UserInfoCondition) -> Result<Vec<UserInfoVo>, DbErr> {
        let user_info = UserInfo::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserInfoVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(user_info)
    }

    async fn page(&self, condition: UserInfoCondition) -> Result<PageData<UserInfoVo>, DbErr> {
        let user_info = UserInfo::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<UserInfoVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = UserInfo::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, user_info, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<UserInfoVo>, DbErr> {
        let user_info_opt = UserInfo::find_by_id(rec_id)
            .into_model::<UserInfoVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(user_info_opt)
    }

    async fn save(&self, user_info_dto: UserInfoDto) -> Result<i64, DbErr> {
        let user_info_dtoc = user_info_dto.into_json_with_snake_key();
        info!("user_info_json is {:?}", user_info_dtoc);
        let mut user_info_actmod = user_info::ActiveModel::from_json(user_info_dtoc)?;
        user_info_actmod.set(user_info::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        user_info_actmod.set(user_info::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = UserInfo::insert(user_info_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr> {
        let user_info_dto_json = user_info_dto.into_json_with_snake_key();
        info!("user_info_json is {:?}", user_info_dto);
        let user_info_actmod = user_info::ActiveModel::from_json(user_info_dto_json)?;
        let update_result = UserInfo::update_many()
            .set(user_info_actmod)
            .filter(user_info::Column::Id.eq(user_info_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr> {
        info!("user_info_json is {:?}", user_info_dto);
        let update_result = UserInfo::update_many()
            .col_expr(user_info::Column::IsDel, Expr::value(-1))
            .filter(user_info::Column::Id.is_in(user_info_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, user_info_dto: UserInfoDto) -> Result<u64, DbErr> {
        info!("user_info_json is {:?}", user_info_dto);
        let update_result = UserInfo::delete_many()
            .filter(user_info::Column::Id.is_in(user_info_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 