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
use crate::{pojo::organization_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for organization-related database operations
#[async_trait::async_trait]
pub trait OrganizationMapperTrait {
    async fn list(&self, condition: OrganizationCondition) -> Result<Vec<OrganizationVo>, DbErr>;
    async fn page(&self, condition: OrganizationCondition) -> Result<PageData<OrganizationVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<OrganizationVo>, DbErr>;
    async fn save(&self, organization_dto: OrganizationDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr>;
}

/// Implementation of OrganizationMapperTrait
pub struct OrganizationMapper {
    state: Arc<AppState>,
}

impl OrganizationMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static OrganizationMapper {
        static INSTANCE: OnceCell<OrganizationMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| OrganizationMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &OrganizationCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(organization::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(organization::Column::Id.is_in(ids.clone()));
        };
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(organization::Column::Code.eq(code));
        };
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(organization::Column::ParentCode.eq(parent_code));
        };
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(organization::Column::Name.eq(name));
        };
        if let Some(r#type) = &condition.r#type {
            query_wrapper = query_wrapper.add(organization::Column::Type.eq(r#type));
        };
        if let Some(contacts) = &condition.contacts {
            query_wrapper = query_wrapper.add(organization::Column::Contacts.eq(contacts));
        };
        if let Some(cellphone) = &condition.cellphone {
            query_wrapper = query_wrapper.add(organization::Column::Cellphone.eq(cellphone));
        };
        if let Some(email) = &condition.email {
            query_wrapper = query_wrapper.add(organization::Column::Email.eq(email));
        };
        if let Some(uscc) = &condition.uscc {
            query_wrapper = query_wrapper.add(organization::Column::Uscc.eq(uscc));
        };
        if let Some(status) = &condition.status {
            query_wrapper = query_wrapper.add(organization::Column::Status.eq(status));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &OrganizationCondition,
        list: Vec<OrganizationVo>,
        total: u64,
    ) -> Result<PageData<OrganizationVo>, DbErr> {
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
impl OrganizationMapperTrait for OrganizationMapper {
    async fn list(&self, condition: OrganizationCondition) -> Result<Vec<OrganizationVo>, DbErr> {
        let organization = Organization::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<OrganizationVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(organization)
    }

    async fn page(&self, condition: OrganizationCondition) -> Result<PageData<OrganizationVo>, DbErr> {
        let organization = Organization::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<OrganizationVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = Organization::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, organization, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<OrganizationVo>, DbErr> {
        let organization_opt = Organization::find_by_id(rec_id)
            .into_model::<OrganizationVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(organization_opt)
    }

    async fn save(&self, organization_dto: OrganizationDto) -> Result<i64, DbErr> {
        let organization_dtoc = organization_dto.into_json_with_snake_key();
        info!("organization_json is {:?}", organization_dtoc);
        let mut organization_actmod = organization::ActiveModel::from_json(organization_dtoc)?;
        organization_actmod.set(organization::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        organization_actmod.set(organization::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = Organization::insert(organization_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr> {
        let organization_dto_json = organization_dto.into_json_with_snake_key();
        info!("organization_json is {:?}", organization_dto);
        let organization_actmod = organization::ActiveModel::from_json(organization_dto_json)?;
        let update_result = Organization::update_many()
            .set(organization_actmod)
            .filter(organization::Column::Id.eq(organization_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr> {
        info!("organization_json is {:?}", organization_dto);
        let update_result = Organization::update_many()
            .col_expr(organization::Column::IsDel, Expr::value(-1))
            .filter(organization::Column::Id.is_in(organization_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, organization_dto: OrganizationDto) -> Result<u64, DbErr> {
        info!("organization_json is {:?}", organization_dto);
        let update_result = Organization::delete_many()
            .filter(organization::Column::Id.is_in(organization_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 