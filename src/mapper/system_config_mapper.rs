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
use crate::{pojo::system_config_pojo::*, AppState};
use sea_orm::Condition;

/// Trait defining the interface for system config-related database operations
#[async_trait::async_trait]
pub trait SystemConfigMapperTrait {
    async fn list(&self, condition: SystemConfigCondition) -> Result<Vec<SystemConfigVo>, DbErr>;
    async fn page(&self, condition: SystemConfigCondition) -> Result<PageData<SystemConfigVo>, DbErr>;
    async fn get_by_id(&self, rec_id: i64) -> Result<Option<SystemConfigVo>, DbErr>;
    async fn save(&self, system_config_dto: SystemConfigDto) -> Result<i64, DbErr>;
    async fn update_by_id(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr>;
    async fn delete_by_ids(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr>;
    async fn remove_by_ids(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr>;
}

/// Implementation of SystemConfigMapperTrait
pub struct SystemConfigMapper {
    state: Arc<AppState>,
}

impl SystemConfigMapper {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn get_instance(state: Arc<AppState>) -> &'static SystemConfigMapper {
        static INSTANCE: OnceCell<SystemConfigMapper> = OnceCell::new();
        INSTANCE.get_or_init(|| SystemConfigMapper::new(state))
    }

    fn build_query_wrapper(&self, condition: &SystemConfigCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(system_config::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(system_config::Column::Id.is_in(ids.clone()));
        };
        if let Some(key) = &condition.config_key {
            query_wrapper = query_wrapper.add(system_config::Column::ConfigKey.eq(key));
        };
        if let Some(config_type) = &condition.config_type {
            query_wrapper = query_wrapper.add(system_config::Column::ConfigType.eq(config_type));
        };
        query_wrapper
    }

    async fn convert_page_data(
        &self,
        condition: &SystemConfigCondition,
        list: Vec<SystemConfigVo>,
        total: u64,
    ) -> Result<PageData<SystemConfigVo>, DbErr> {
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
impl SystemConfigMapperTrait for SystemConfigMapper {
    async fn list(&self, condition: SystemConfigCondition) -> Result<Vec<SystemConfigVo>, DbErr> {
        let system_config = SystemConfig::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<SystemConfigVo>()
            .all(&self.state.mysql_pool)
            .await?;

        Ok(system_config)
    }

    async fn page(&self, condition: SystemConfigCondition) -> Result<PageData<SystemConfigVo>, DbErr> {
        let system_config = SystemConfig::find()
            .filter(self.build_query_wrapper(&condition))
            .apply_if(condition.get_size(), QuerySelect::limit)
            .apply_if(condition.get_offset(), QuerySelect::offset::<u64>)
            .into_model::<SystemConfigVo>()
            .all(&self.state.mysql_pool)
            .await?;
        let total = SystemConfig::find()
            .filter(self.build_query_wrapper(&condition))
            .count(&self.state.mysql_pool)
            .await?;
        self.convert_page_data(&condition, system_config, total).await
    }

    async fn get_by_id(&self, rec_id: i64) -> Result<Option<SystemConfigVo>, DbErr> {
        let system_config_opt = SystemConfig::find_by_id(rec_id)
            .into_model::<SystemConfigVo>()
            .one(&self.state.mysql_pool)
            .await?;
        Ok(system_config_opt)
    }

    async fn save(&self, system_config_dto: SystemConfigDto) -> Result<i64, DbErr> {
        let system_config_dtoc = system_config_dto.into_json_with_snake_key();
        info!("system_config_json is {:?}", system_config_dtoc);
        let mut system_config_actmod = system_config::ActiveModel::from_json(system_config_dtoc)?;
        system_config_actmod.set(system_config::Column::CreateBy, sea_orm::Value::BigInt(Some(0)));
        system_config_actmod.set(system_config::Column::UpdateBy, sea_orm::Value::BigInt(Some(0)));
        let inserted_result = SystemConfig::insert(system_config_actmod).exec(&self.state.mysql_pool).await?;
        Ok(inserted_result.last_insert_id)
    }

    async fn update_by_id(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr> {
        let system_config_dto_json = system_config_dto.into_json_with_snake_key();
        info!("system_config_json is {:?}", system_config_dto);
        let system_config_actmod = system_config::ActiveModel::from_json(system_config_dto_json)?;
        let update_result = SystemConfig::update_many()
            .set(system_config_actmod)
            .filter(system_config::Column::Id.eq(system_config_dto.rec_id))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn delete_by_ids(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr> {
        info!("system_config_json is {:?}", system_config_dto);
        let update_result = SystemConfig::update_many()
            .col_expr(system_config::Column::IsDel, Expr::value(-1))
            .filter(system_config::Column::Id.is_in(system_config_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }

    async fn remove_by_ids(&self, system_config_dto: SystemConfigDto) -> Result<u64, DbErr> {
        info!("system_config_json is {:?}", system_config_dto);
        let update_result = SystemConfig::delete_many()
            .filter(system_config::Column::Id.is_in(system_config_dto.rec_ids.unwrap()))
            .exec(&self.state.mysql_pool)
            .await?;
        Ok(update_result.rows_affected)
    }
} 