use sea_orm::sea_query::Cond;
use sea_orm::{ColumnTrait, Condition};

use crate::entities::{prelude::*, *};
use crate::pojo::organization_pojo::*;

crate::define_mapper_trait!(OrganizationMapperTrait, OrganizationCondition, OrganizationVo, OrganizationDto);
crate::define_mapper_struct!(OrganizationMapper);

impl OrganizationMapper {
    fn build_query_wrapper(&self, condition: &OrganizationCondition) -> Condition {
        let mut query_wrapper = Cond::all().add(organization::Column::IsDel.eq(0));
        if let Some(ids) = &condition.ids {
            query_wrapper = query_wrapper.add(organization::Column::Id.is_in(ids.clone()));
        }
        if let Some(code) = &condition.code {
            query_wrapper = query_wrapper.add(organization::Column::Code.eq(code));
        }
        if let Some(parent_code) = &condition.parent_code {
            query_wrapper = query_wrapper.add(organization::Column::ParentCode.eq(parent_code));
        }
        if let Some(name) = &condition.name {
            query_wrapper = query_wrapper.add(organization::Column::Name.eq(name));
        }
        if let Some(r#type) = &condition.r#type {
            query_wrapper = query_wrapper.add(organization::Column::Type.eq(r#type));
        }
        if let Some(contacts) = &condition.contacts {
            query_wrapper = query_wrapper.add(organization::Column::Contacts.eq(contacts));
        }
        if let Some(cellphone) = &condition.cellphone {
            query_wrapper = query_wrapper.add(organization::Column::Cellphone.eq(cellphone));
        }
        if let Some(email) = &condition.email {
            query_wrapper = query_wrapper.add(organization::Column::Email.eq(email));
        }
        if let Some(uscc) = &condition.uscc {
            query_wrapper = query_wrapper.add(organization::Column::Uscc.eq(uscc));
        }
        if let Some(status) = &condition.status {
            query_wrapper = query_wrapper.add(organization::Column::Status.eq(status));
        }
        query_wrapper
    }
}

crate::impl_mapper!(OrganizationMapper, OrganizationMapperTrait, Organization, organization, OrganizationCondition, OrganizationVo, OrganizationDto);
