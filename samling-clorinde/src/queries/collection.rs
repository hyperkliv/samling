#[derive(Debug)]
pub struct SelectCollectionsParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub requester_id: i32,
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct SelectCollectionSummariesParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub requester_id: i32,
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetCollectionIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertCollectionParams<
    T1: crate::JsonSql,
    T2: crate::JsonSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub acronym: T1,
    pub name: T2,
    pub image_url: Option<T3>,
    pub slug: T4,
    pub external_id: Option<T5>,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateCollectionParams<
    T1: crate::JsonSql,
    T2: crate::JsonSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub acronym: Option<T1>,
    pub name: Option<T2>,
    pub image_url: Option<T3>,
    pub slug: Option<T4>,
    pub external_id: Option<T5>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteCollectionParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug)]
pub struct AssociateCollectionSizesParams<T1: crate::ArraySql<Item = i32>> {
    pub collection_id: i32,
    pub size_ids: T1,
}
#[derive(Debug)]
pub struct ReplaceCollectionPricelistsParams<
    T1: crate::ArraySql<Item = crate::types::CollectionPricelistRelation>,
> {
    pub collection_id: i32,
    pub collection_pricelist_relations: T1,
}
#[derive(Debug)]
pub struct SetNewCollectionStylesParams<T1: crate::ArraySql<Item = i32>> {
    pub collection_id: i32,
    pub style_ids: T1,
}
#[derive(Debug)]
pub struct SetNewCollectionColorsParams<T1: crate::ArraySql<Item = i32>> {
    pub collection_id: i32,
    pub color_ids: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub image_url: Option<String>,
    pub acronym: serde_json::Value,
    pub pricing: serde_json::Value,
    pub sizes: serde_json::Value,
}
pub struct CollectionRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub image_url: Option<&'a str>,
    pub acronym: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub pricing: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub sizes: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<CollectionRowBorrowed<'a>> for CollectionRow {
    fn from(
        CollectionRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            name,
            created_by,
            created_at,
            updated_at,
            image_url,
            acronym,
            pricing,
            sizes,
        }: CollectionRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            name: serde_json::from_str(name.0.get()).unwrap(),
            created_by,
            created_at,
            updated_at,
            image_url: image_url.map(|v| v.into()),
            acronym: serde_json::from_str(acronym.0.get()).unwrap(),
            pricing: serde_json::from_str(pricing.0.get()).unwrap(),
            sizes: serde_json::from_str(sizes.0.get()).unwrap(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionSummaryRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub image_url: Option<String>,
    pub acronym: serde_json::Value,
    pub num_sizes: i64,
    pub num_colors: i64,
    pub num_styles: i64,
    pub pricing: serde_json::Value,
}
pub struct CollectionSummaryRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub image_url: Option<&'a str>,
    pub acronym: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub num_sizes: i64,
    pub num_colors: i64,
    pub num_styles: i64,
    pub pricing: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<CollectionSummaryRowBorrowed<'a>> for CollectionSummaryRow {
    fn from(
        CollectionSummaryRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            name,
            created_by,
            created_at,
            updated_at,
            image_url,
            acronym,
            num_sizes,
            num_colors,
            num_styles,
            pricing,
        }: CollectionSummaryRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            name: serde_json::from_str(name.0.get()).unwrap(),
            created_by,
            created_at,
            updated_at,
            image_url: image_url.map(|v| v.into()),
            acronym: serde_json::from_str(acronym.0.get()).unwrap(),
            num_sizes,
            num_colors,
            num_styles,
            pricing: serde_json::from_str(pricing.0.get()).unwrap(),
        }
    }
} // This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CollectionRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> CollectionRowBorrowed,
    mapper: fn(CollectionRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> CollectionRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(CollectionRowBorrowed) -> R) -> CollectionRowQuery<'a, C, R, N> {
        CollectionRowQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
            .into_stream();
        Ok(it)
    }
}
pub struct CollectionSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> CollectionSummaryRowBorrowed,
    mapper: fn(CollectionSummaryRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> CollectionSummaryRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CollectionSummaryRowBorrowed) -> R,
    ) -> CollectionSummaryRowQuery<'a, C, R, N> {
        CollectionSummaryRowQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
            .into_stream();
        Ok(it)
    }
}
pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
}
impl<'a, C, T: 'a, const N: usize> I32Query<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i32) -> R) -> I32Query<'a, C, R, N> {
        I32Query {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| (self.mapper)((self.extractor)(&row))))
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
            .into_stream();
        Ok(it)
    }
}
pub fn select_collections() -> SelectCollectionsStmt {
    SelectCollectionsStmt(crate::client::async_::Stmt::new(
        "SELECT
    collection.*,
    coalesce(pricing.json_data, '[]'::json) AS pricing,
    coalesce(sizes.json_data, '[]'::json) AS sizes
FROM
    collection
INNER JOIN (
    SELECT group_collection.collection_id
    FROM group_collection
    INNER JOIN group_user
        ON group_user.group_id = group_collection.group_id
    WHERE
        group_user.user_id = $1
    GROUP BY group_collection.collection_id
    UNION
    SELECT collection.id AS collection_id
    FROM collection
    WHERE collection.created_by = $1
) AS requester_collections ON requester_collections.collection_id = collection.id
LEFT JOIN (
    SELECT
        size_collection.collection_id,
        json_agg(
            size.* ORDER BY size_collection.position
        ) FILTER (WHERE size.id IS NOT NULL) AS json_data
    FROM size
    INNER JOIN size_collection ON size_collection.size_id = size.id
    GROUP BY size_collection.collection_id
) AS sizes ON sizes.collection_id = collection.id
LEFT JOIN LATERAL (
    SELECT
        collection_pricelist.collection_id,
        json_agg(
            json_build_object(
                'list',
                json_build_object(
                    'id',
                    pricelist.id,
                    'external_id',
                    pricelist.external_id,
                    'slug',
                    pricelist.slug,
                    'name',
                    pricelist.name
                ),
                'date',
                collection_pricelist.price_date
            )
        ) FILTER (WHERE collection_pricelist.collection_id IS NOT NULL) AS json_data
    FROM collection_pricelist
    INNER JOIN pricelist
        ON pricelist.id = collection_pricelist.pricelist_id
    WHERE collection_pricelist.collection_id = collection.id
    GROUP BY collection_pricelist.collection_id
) AS pricing ON pricing.collection_id = collection.id
WHERE
    collection.organization_id = $2
    AND ($3::int IS NULL OR collection.id = $3)
    AND ($4::text IS NULL OR collection.external_id = $4)
    AND ($5::text IS NULL OR collection.slug = $5)
ORDER BY
    collection.updated_at DESC",
    ))
}
pub struct SelectCollectionsStmt(crate::client::async_::Stmt);
impl SelectCollectionsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        requester_id: &'a i32,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> CollectionRowQuery<'a, C, CollectionRow, 5> {
        CollectionRowQuery {
            client,
            params: [requester_id, organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| CollectionRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                name: row.get(4),
                created_by: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
                image_url: row.get(8),
                acronym: row.get(9),
                pricing: row.get(10),
                sizes: row.get(11),
            },
            mapper: |it| <CollectionRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        SelectCollectionsParams<T1, T2>,
        CollectionRowQuery<'a, C, CollectionRow, 5>,
        C,
    > for SelectCollectionsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectCollectionsParams<T1, T2>,
    ) -> CollectionRowQuery<'a, C, CollectionRow, 5> {
        self.bind(
            client,
            &params.requester_id,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn select_collection_summaries() -> SelectCollectionSummariesStmt {
    SelectCollectionSummariesStmt(crate::client::async_::Stmt::new(
        "SELECT
    collection.*,
    coalesce(stats.num_sizes, 0) AS num_sizes,
    coalesce(stats.num_colors, 0) AS num_colors,
    coalesce(stats.num_styles, 0) AS num_styles,
    coalesce(pricing.json_data, '[]'::json) AS pricing
FROM
    collection
INNER JOIN (
    SELECT group_collection.collection_id
    FROM group_collection
    INNER JOIN group_user
        ON group_user.group_id = group_collection.group_id
    WHERE
        group_user.user_id = $1
    GROUP BY group_collection.collection_id
    UNION
    SELECT collection.id AS collection_id
    FROM collection
    WHERE collection.created_by = $1
) AS requester_collections ON requester_collections.collection_id = collection.id
LEFT JOIN (
    SELECT
        size_collection.collection_id,
        count(DISTINCT size.id) AS num_sizes,
        count(DISTINCT color.id) AS num_colors,
        count(DISTINCT color.style_id) AS num_styles
    FROM color
    INNER JOIN size ON size.color_id = color.id
    INNER JOIN size_collection ON size.id = size_collection.size_id
    GROUP BY size_collection.collection_id
) AS stats ON stats.collection_id = collection.id
LEFT JOIN LATERAL (
    SELECT
        collection_pricelist.collection_id,
        json_agg(
            json_build_object(
                'list',
                json_build_object(
                    'id',
                    pricelist.id,
                    'external_id',
                    pricelist.external_id,
                    'slug',
                    pricelist.slug,
                    'name',
                    pricelist.name
                ),
                'date',
                collection_pricelist.price_date
            )
        ) FILTER (WHERE collection_pricelist.collection_id IS NOT NULL) AS json_data,
        min(collection_pricelist.price_date) AS min_price_date
    FROM collection_pricelist
    INNER JOIN pricelist
        ON pricelist.id = collection_pricelist.pricelist_id
    WHERE collection_pricelist.collection_id = collection.id
    GROUP BY collection_pricelist.collection_id
) AS pricing ON pricing.collection_id = collection.id
WHERE
    collection.organization_id = $2
    AND ($3::int IS NULL OR collection.id = $3)
    AND ($4::text IS NULL OR collection.external_id = $4)
    AND ($5::text IS NULL OR collection.slug = $5)
ORDER BY
    pricing.min_price_date DESC, collection.name ASC",
    ))
}
pub struct SelectCollectionSummariesStmt(crate::client::async_::Stmt);
impl SelectCollectionSummariesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        requester_id: &'a i32,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> CollectionSummaryRowQuery<'a, C, CollectionSummaryRow, 5> {
        CollectionSummaryRowQuery {
            client,
            params: [requester_id, organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| CollectionSummaryRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                name: row.get(4),
                created_by: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
                image_url: row.get(8),
                acronym: row.get(9),
                num_sizes: row.get(10),
                num_colors: row.get(11),
                num_styles: row.get(12),
                pricing: row.get(13),
            },
            mapper: |it| <CollectionSummaryRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        SelectCollectionSummariesParams<T1, T2>,
        CollectionSummaryRowQuery<'a, C, CollectionSummaryRow, 5>,
        C,
    > for SelectCollectionSummariesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectCollectionSummariesParams<T1, T2>,
    ) -> CollectionSummaryRowQuery<'a, C, CollectionSummaryRow, 5> {
        self.bind(
            client,
            &params.requester_id,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn get_collection_id() -> GetCollectionIdStmt {
    GetCollectionIdStmt(crate::client::async_::Stmt::new(
        "SELECT collection.id
FROM
    collection
WHERE
    collection.organization_id = $1
    AND ($2::int IS NULL OR collection.id = $2)
    AND ($3::text IS NULL OR collection.external_id = $3)
    AND ($4::text IS NULL OR collection.slug = $4)",
    ))
}
pub struct GetCollectionIdStmt(crate::client::async_::Stmt);
impl GetCollectionIdStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> I32Query<'a, C, i32, 4> {
        I32Query {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<'a, GetCollectionIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetCollectionIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetCollectionIdParams<T1, T2>,
    ) -> I32Query<'a, C, i32, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn insert_collection() -> InsertCollectionStmt {
    InsertCollectionStmt(crate::client::async_::Stmt::new(
        "INSERT INTO collection (
    acronym,
    name,
    image_url,
    slug,
    external_id,
    organization_id,
    created_by)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7)
RETURNING
id",
    ))
}
pub struct InsertCollectionStmt(crate::client::async_::Stmt);
impl InsertCollectionStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::JsonSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        acronym: &'a T1,
        name: &'a T2,
        image_url: &'a Option<T3>,
        slug: &'a T4,
        external_id: &'a Option<T5>,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 7> {
        I32Query {
            client,
            params: [
                acronym,
                name,
                image_url,
                slug,
                external_id,
                organization_id,
                created_by,
            ],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::JsonSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        InsertCollectionParams<T1, T2, T3, T4, T5>,
        I32Query<'a, C, i32, 7>,
        C,
    > for InsertCollectionStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertCollectionParams<T1, T2, T3, T4, T5>,
    ) -> I32Query<'a, C, i32, 7> {
        self.bind(
            client,
            &params.acronym,
            &params.name,
            &params.image_url,
            &params.slug,
            &params.external_id,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_collection() -> UpdateCollectionStmt {
    UpdateCollectionStmt(crate::client::async_::Stmt::new(
        "UPDATE
collection
SET
    acronym = coalesce($1, acronym),
    name = coalesce($2, name),
    image_url = coalesce($3, image_url),
    slug = coalesce($4, slug),
    external_id = coalesce($5, external_id)
WHERE
    id = $6",
    ))
}
pub struct UpdateCollectionStmt(crate::client::async_::Stmt);
impl UpdateCollectionStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::JsonSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        acronym: &'a Option<T1>,
        name: &'a Option<T2>,
        image_url: &'a Option<T3>,
        slug: &'a Option<T4>,
        external_id: &'a Option<T5>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[acronym, name, image_url, slug, external_id, id])
            .await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::JsonSql,
        T2: crate::JsonSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        UpdateCollectionParams<T1, T2, T3, T4, T5>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateCollectionStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateCollectionParams<T1, T2, T3, T4, T5>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.acronym,
            &params.name,
            &params.image_url,
            &params.slug,
            &params.external_id,
            &params.id,
        ))
    }
}
pub fn delete_collection() -> DeleteCollectionStmt {
    DeleteCollectionStmt(crate::client::async_::Stmt::new(
        "DELETE FROM collection
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeleteCollectionStmt(crate::client::async_::Stmt);
impl DeleteCollectionStmt {
    pub async fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[organization_id, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        DeleteCollectionParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteCollectionStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteCollectionParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
pub fn associate_collection_sizes() -> AssociateCollectionSizesStmt {
    AssociateCollectionSizesStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    associate_collection_sizes($1, $2)",
    ))
}
pub struct AssociateCollectionSizesStmt(crate::client::async_::Stmt);
impl AssociateCollectionSizesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        collection_id: &'a i32,
        size_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [collection_id, size_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        AssociateCollectionSizesParams<T1>,
        I32Query<'a, C, i32, 2>,
        C,
    > for AssociateCollectionSizesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a AssociateCollectionSizesParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.collection_id, &params.size_ids)
    }
}
pub fn replace_collection_pricelists() -> ReplaceCollectionPricelistsStmt {
    ReplaceCollectionPricelistsStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    replace_collection_pricelists($1, $2)",
    ))
}
pub struct ReplaceCollectionPricelistsStmt(crate::client::async_::Stmt);
impl ReplaceCollectionPricelistsStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::ArraySql<Item = crate::types::CollectionPricelistRelation>,
    >(
        &'a mut self,
        client: &'a C,
        collection_id: &'a i32,
        collection_pricelist_relations: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [collection_id, collection_pricelist_relations],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<
        'a,
        C: GenericClient,
        T1: crate::ArraySql<Item = crate::types::CollectionPricelistRelation>,
    >
    crate::client::async_::Params<
        'a,
        ReplaceCollectionPricelistsParams<T1>,
        I32Query<'a, C, i32, 2>,
        C,
    > for ReplaceCollectionPricelistsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a ReplaceCollectionPricelistsParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(
            client,
            &params.collection_id,
            &params.collection_pricelist_relations,
        )
    }
}
pub fn set_new_collection_styles() -> SetNewCollectionStylesStmt {
    SetNewCollectionStylesStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    set_new_collection_styles($1, $2)",
    ))
}
pub struct SetNewCollectionStylesStmt(crate::client::async_::Stmt);
impl SetNewCollectionStylesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        collection_id: &'a i32,
        style_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [collection_id, style_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, SetNewCollectionStylesParams<T1>, I32Query<'a, C, i32, 2>, C>
    for SetNewCollectionStylesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SetNewCollectionStylesParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.collection_id, &params.style_ids)
    }
}
pub fn set_new_collection_colors() -> SetNewCollectionColorsStmt {
    SetNewCollectionColorsStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    set_new_collection_colors($1, $2)",
    ))
}
pub struct SetNewCollectionColorsStmt(crate::client::async_::Stmt);
impl SetNewCollectionColorsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        collection_id: &'a i32,
        color_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [collection_id, color_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, SetNewCollectionColorsParams<T1>, I32Query<'a, C, i32, 2>, C>
    for SetNewCollectionColorsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SetNewCollectionColorsParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.collection_id, &params.color_ids)
    }
}
