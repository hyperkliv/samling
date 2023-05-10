use cornucopia_async::{GenericClient, Params};
use deadpool_postgres::Object;

use super::{CreatePrice, CreatePriceList, Price, PriceList, UpdatePrice, UpdatePriceList};
use crate::{
    auth::User,
    cornucopia::{
        queries::{
            group::ensure_superuser_access,
            price::{
                delete_price, insert_price, select_prices, update_price, DeletePriceParams,
                InsertPriceParams, PriceRow, PriceRowBorrowed, SelectPricesParams,
                UpdatePriceParams,
            },
            pricelist::{
                delete_pricelist, get_pricelist, insert_pricelist, list_pricelist_summaries,
                list_pricelists, update_pricelist, InsertPricelistParams,
                ListPricelistSummariesParams, ListPricelistsParams, PriceListRow,
                PriceListRowBorrowed, PriceListSummaryRow, PriceListSummaryRowBorrowed,
                UpdatePricelistParams,
            },
        },
        types::public::Pricetype,
    },
    entity_ref::{ExternalIdEntity, Id, RefTarget},
    organizations::Organization,
    Error, PriceListSummary, PriceType, Ref, RequestMetaData, Result, SlugEntity, Style,
};

#[derive(Clone)]
pub struct PricesRepo;

impl PricesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Price>> {
        select_prices()
            .params(
                client,
                &SelectPricesParams {
                    organization_id: organization_id.into(),
                    ids: None::<Vec<i32>>,
                    external_ids: None::<Vec<String>>,
                },
            )
            .map(handle_price_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Price>,
    ) -> Result<Option<Price>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if slug.is_some() {
            Err(Error::SlugReferenceUnsupported("price"))
        } else if let Some(res) = select_prices()
            .params(
                client,
                &SelectPricesParams {
                    organization_id: organization_id.into(),
                    ids: id.map(|val| vec![val]),
                    external_ids: external_id.map(|val| vec![val]),
                },
            )
            .map(handle_price_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Price>,
    ) -> Result<Price> {
        if let Some(price) = self.find(client, organization_id, ref_).await? {
            Ok(price)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        price: CreatePrice,
    ) -> Result<Price> {
        let tx = client.transaction().await?;
        price
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let style_id = Style::get_id(&tx, organization_id, &price.style).await?;
        let list_id = PriceList::get_id(&tx, organization_id, &price.list).await?;

        let inserted_id: Id<Price> = insert_price()
            .params(
                &tx,
                &InsertPriceParams {
                    uom: price.uom,
                    r#type: price.r#type.into(),
                    currency: price.currency,
                    style_id: style_id.into(),
                    list_id: list_id.into(),
                    external_id: price.external_id.as_deref(),
                    organization_id: organization_id.into(),
                    created_by: created_by.into(),
                    amount: price.amount,
                    start: price.start,
                    end: price.end,
                },
            )
            .one()
            .await?
            .into();

        let price = self.get(&tx, organization_id, &inserted_id.into()).await?;
        tx.commit().await?;
        Ok(price)
    }

    pub async fn update(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        ref_: &Ref<Price>,
        price: UpdatePrice,
    ) -> Result<Price> {
        let tx = client.transaction().await?;
        let existing = self.get(&tx, organization_id, ref_).await?;
        if price.external_id.is_some() && existing.external_id != price.external_id {
            price
                .ensure_available_external_id(&tx, organization_id)
                .await?;
        }
        let style_id = if let Some(style_ref) = price.style {
            Style::get_id(&tx, organization_id, &style_ref).await?
        } else {
            existing.style.id
        };
        let list_id = if let Some(list_ref) = price.list {
            PriceList::get_id(&tx, organization_id, &list_ref).await?
        } else {
            existing.list.id
        };
        let num_updated = update_price()
            .params(
                &tx,
                &UpdatePriceParams {
                    uom: price.uom,
                    r#type: price.r#type.map(|t| t.into()),
                    currency: price.currency,
                    style_id: Some(style_id.into()),
                    list_id: Some(list_id.into()),
                    external_id: price.external_id.as_deref(),
                    id: existing.id.into(),
                    amount: price.amount,
                    start: price.start,
                    end: price.end,
                },
            )
            .await?;
        debug_assert_eq!(num_updated, 1);

        let price = self.get(&tx, organization_id, ref_).await?;
        tx.commit().await?;
        Ok(price)
    }

    pub async fn upsert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        ref_: &Ref<Price>,
        mut price: CreatePrice,
    ) -> Result<(bool, Price)> {
        price.update_external_id_from_ref(ref_);
        if Price::lookup_id(client, organization_id, ref_)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, ref_, price.to_owned().into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, price.to_owned())
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Price>,
    ) -> Result<()> {
        let id = Price::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_price()
            .params(
                client,
                &DeletePriceParams {
                    organization_id: organization_id.into(),
                    id: id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_price_row(row: PriceRowBorrowed) -> Result<Price> {
    let row: PriceRow = row.into();
    Ok(Price {
        id: row.id.into(),
        r#type: row.r#type.into(),
        uom: row.uom,
        currency: row.currency,
        amount: row.amount,
        start: row.start,
        end: row.end,
        style: serde_path_to_error::deserialize(row.style)?,
        list: serde_path_to_error::deserialize(row.list)?,
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
    })
}

#[derive(Clone)]
pub struct PriceListsRepo;

impl PriceListsRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
    ) -> Result<Vec<PriceList>> {
        list_pricelists()
            .params(
                client,
                &ListPricelistsParams {
                    organization_id: metadata.organization_id().into(),
                    requester_id: metadata.user_id().into(),
                },
            )
            .map(handle_pricelist_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn list_summaries(
        &self,
        client: &impl GenericClient,
        metadata: RequestMetaData,
    ) -> Result<Vec<PriceListSummary>> {
        list_pricelist_summaries()
            .params(
                client,
                &ListPricelistSummariesParams {
                    organization_id: metadata.organization_id().into(),
                    requester_id: metadata.user_id().into(),
                },
            )
            .map(handle_pricelist_summary_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<PriceList>,
    ) -> Result<Option<PriceList>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if slug.is_some() {
            Err(Error::SlugReferenceUnsupported("pricelist"))
        } else if let Some(res) = get_pricelist()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug,
            )
            .map(handle_pricelist_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<PriceList>,
    ) -> Result<PriceList> {
        if let Some(pricelist) = self.find(client, organization_id, ref_).await? {
            Ok(pricelist)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        pricelist: CreatePriceList,
    ) -> Result<PriceList> {
        let tx = client.transaction().await?;
        let slug = pricelist
            .check_or_generate_slug(&tx, organization_id, "")
            .await?;
        pricelist
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let inserted_id: Id<PriceList> = insert_pricelist()
            .params(
                &tx,
                &InsertPricelistParams {
                    name: pricelist.name,
                    slug: slug.as_ref(),
                    external_id: pricelist.external_id.as_deref(),
                    organization_id: organization_id.into(),
                    created_by: created_by.into(),
                },
            )
            .one()
            .await?
            .into();

        let pricelist = self.get(&tx, organization_id, &inserted_id.into()).await?;
        tx.commit().await?;
        ensure_superuser_access()
            .bind(client, &organization_id.into())
            .one()
            .await?;
        Ok(pricelist)
    }

    pub async fn update(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        ref_: &Ref<PriceList>,
        pricelist: UpdatePriceList,
    ) -> Result<PriceList> {
        let existing = self.get(client, organization_id, ref_).await?;
        let tx = client.transaction().await?;
        if pricelist.slug.is_some() && Some(existing.slug) != pricelist.slug {
            pricelist
                .ensure_available_slug(&tx, organization_id)
                .await?;
        }
        if pricelist.external_id.is_some() && existing.external_id != pricelist.external_id {
            pricelist
                .ensure_available_external_id(&tx, organization_id)
                .await?;
        }
        let num_updated = update_pricelist()
            .params(
                &tx,
                &UpdatePricelistParams {
                    name: pricelist.name,
                    slug: pricelist.slug.as_deref(),
                    external_id: pricelist.external_id.as_deref(),
                    id: existing.id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_updated, 1);

        let pricelist = self.get(&tx, organization_id, ref_).await?;
        tx.commit().await?;
        Ok(pricelist)
    }

    pub async fn upsert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        ref_: &Ref<PriceList>,
        mut pricelist: CreatePriceList,
    ) -> Result<(bool, PriceList)> {
        pricelist.update_slug_from_ref(ref_);
        pricelist.update_external_id_from_ref(ref_);
        if PriceList::lookup_id(client, organization_id, ref_)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, ref_, pricelist.to_owned().into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, pricelist.to_owned())
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<PriceList>,
    ) -> Result<()> {
        let id = PriceList::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_pricelist()
            .bind(client, &organization_id.into(), (&id).into())
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_pricelist_row(row: PriceListRowBorrowed) -> Result<PriceList> {
    let row: PriceListRow = row.into();
    Ok(PriceList {
        id: row.id.into(),
        name: row.name,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
    })
}

fn handle_pricelist_summary_row(row: PriceListSummaryRowBorrowed) -> Result<PriceListSummary> {
    let row: PriceListSummaryRow = row.into();
    Ok(PriceListSummary {
        id: row.id.into(),
        name: row.name,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
    })
}

impl From<Pricetype> for PriceType {
    fn from(pt: Pricetype) -> Self {
        match pt {
            Pricetype::Unit => PriceType::Unit,
            Pricetype::Retail => PriceType::Retail,
        }
    }
}

impl From<PriceType> for Pricetype {
    fn from(pt: PriceType) -> Self {
        match pt {
            PriceType::Unit => Pricetype::Unit,
            PriceType::Retail => Pricetype::Retail,
        }
    }
}
