use cornucopia_async::GenericClient;
use futures::future::try_join4;

use crate::{
    cornucopia::queries::admin::{
        select_attribute_filter_choices, select_category_filter_choices,
        select_status_filter_choices, select_style_filter_choices, EntityFilterChoiceRow,
        StringFilterChoiceRowBorrowed,
    },
    I18nString, Id, ItemFilterChoices, Organization, RefType, Result,
};

use super::{EntityFilterChoice, StringFilterChoice};

#[derive(Clone)]
pub struct FiltersRepo;

impl FiltersRepo {
    pub async fn list_item_filters(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<ItemFilterChoices> {
        let mut status_query = select_status_filter_choices();
        let mut category_query = select_category_filter_choices();
        let mut style_query = select_style_filter_choices();
        let mut attribute_query = select_attribute_filter_choices();
        let status_future = status_query
            .bind(client, organization_id.inner())
            .map(|row| row.into())
            .all();
        let category_future = category_query.bind(client, organization_id.inner()).all();
        let style_future = style_query.bind(client, organization_id.inner()).all();
        let attribute_future = attribute_query.bind(client, organization_id.inner()).all();
        let (status, category_res, style_res, attribute_res) = try_join4(
            status_future,
            category_future,
            style_future,
            attribute_future,
        )
        .await?;
        let category = category_res
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        let style = style_res
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        let attribute = attribute_res
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(ItemFilterChoices {
            status,
            category,
            style,
            attribute,
        })
    }
}

impl<'a> From<StringFilterChoiceRowBorrowed<'a>> for StringFilterChoice {
    fn from(row: StringFilterChoiceRowBorrowed<'a>) -> Self {
        row.title.to_owned().into()
    }
}

impl TryFrom<EntityFilterChoiceRow> for EntityFilterChoice {
    type Error = crate::Error;
    fn try_from(row: EntityFilterChoiceRow) -> Result<Self> {
        let title: I18nString = serde_path_to_error::deserialize(row.title)?;
        let subtitle = row
            .subtitle
            .map_or_else(|| Ok(None), serde_path_to_error::deserialize)?;
        let image = row
            .image
            .map_or_else(|| Ok(None), serde_path_to_error::deserialize)?;
        Ok(Self {
            id: row.id,
            title,
            subtitle,
            image,
        })
    }
}
