use cornucopia_async::GenericClient;
use futures::{future::try_join3, Stream, StreamExt};

use crate::{
    cornucopia::queries::admin::{
        select_category_filter_choices, select_status_filter_choices, select_style_filter_choices,
        select_user_org_data, EntityFilterChoiceRow, SelectUserOrgDataStmt,
        StringFilterChoiceRowBorrowed,
    },
    Error,
    // cornucopia::queries::admin::{
    //     select_attribute_org_data, select_attributetype_org_data, select_category_filter_choices,
    //     select_category_org_data, select_collection_org_data, select_collection_pricelist_org_data,
    //     select_color_org_data, select_group_collection_org_data, select_group_org_data,
    //     select_group_pricelist_org_data, select_group_user_org_data, select_image_org_data,
    //     select_new_collection_color_org_data, select_new_collection_style_org_data,
    //     select_organization_org_data, select_price_org_data, select_pricelist_org_data,
    //     select_size_collection_org_data, select_size_org_data, select_status_filter_choices,
    //     select_style_attribute_org_data, select_style_category_org_data,
    //     select_style_filter_choices, select_style_org_data, select_user_org_data,
    //     select_user_organization_org_data, EntityFilterChoiceRow, StringFilterChoiceRowBorrowed, UserTableDataBorrowed, UserTableData, SelectUserOrgDataStmt,
    // },
    I18nString,
    Id,
    ItemFilterChoices,
    Organization,
    RefType,
    Result,
};

use super::{EntityFilterChoice, OrganizationDataRow, StringFilterChoice};

pub struct OrganizationDataRepo<C: GenericClient> {
    client: C,
    organization_id: i32,
    user: SelectUserOrgDataStmt,
}

impl<C: GenericClient> OrganizationDataRepo<C> {
    pub(crate) fn new(client: C, organization_id: Id<Organization>) -> Self {
        let mut user = select_user_org_data();
        Self {
            client,
            organization_id: organization_id.into(),
            user,
        }
    }
}

impl<C: GenericClient> OrganizationDataRepo<C> {
    pub async fn stream_organization_data(
        &mut self,
    ) -> Result<impl Stream<Item = Result<OrganizationDataRow>> + '_> {
        let x = self
            .user
            .bind(&self.client, &self.organization_id)
            .iter()
            .await?
            .map(|res| match res {
                Ok(val) => Ok(OrganizationDataRow::User(val)),
                Err(err) => Err(Error::from(err)),
            });

        Ok(x)
    }
    // let user = select_user_org_data().bind(client, &id).all().await?;
    // let organization = select_organization_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let group = select_group_org_data().bind(client, &id).all().await?;
    // let attributetype = select_attributetype_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let attribute = select_attribute_org_data().bind(client, &id).all().await?;
    // let category = select_category_org_data().bind(client, &id).all().await?;
    // let collection = select_collection_org_data().bind(client, &id).all().await?;
    // let style = select_style_org_data().bind(client, &id).all().await?;
    // let color = select_color_org_data().bind(client, &id).all().await?;
    // let size = select_size_org_data().bind(client, &id).all().await?;
    // let image = select_image_org_data().bind(client, &id).all().await?;
    // let pricelist = select_pricelist_org_data().bind(client, &id).all().await?;
    // let price = select_price_org_data().bind(client, &id).all().await?;
    // let user_organization = select_user_organization_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let collection_pricelist = select_collection_pricelist_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let group_collection = select_group_collection_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let group_pricelist = select_group_pricelist_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let group_user = select_group_user_org_data().bind(client, &id).all().await?;
    // let new_collection_style = select_new_collection_style_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let new_collection_color = select_new_collection_color_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let size_collection = select_size_collection_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let style_attribute = select_style_attribute_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // let style_category = select_style_category_org_data()
    //     .bind(client, &id)
    //     .all()
    //     .await?;
    // Ok(OrganizationData {
    //     user,
    //     organization,
    //     group,
    //     attributetype,
    //     attribute,
    //     category,
    //     collection,
    //     style,
    //     color,
    //     size,
    //     image,
    //     pricelist,
    //     price,
    //     user_organization,
    //     collection_pricelist,
    //     group_collection,
    //     group_pricelist,
    //     group_user,
    //     new_collection_style,
    //     new_collection_color,
    //     size_collection,
    //     style_attribute,
    //     style_category,
    // })
}

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
        let status_future = status_query
            .bind(client, organization_id.inner())
            .map(|row| row.into())
            .all();
        let category_future = category_query.bind(client, organization_id.inner()).all();
        let style_future = style_query.bind(client, organization_id.inner()).all();
        let (status, category_res, style_res) =
            try_join3(status_future, category_future, style_future).await?;
        let category = category_res
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        let style = style_res
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        Ok(ItemFilterChoices {
            status,
            category,
            style,
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
