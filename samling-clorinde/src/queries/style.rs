// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct SelectStylesParams<T1: crate::ArraySql<Item = i32>> {
    pub organization_id: i32,
    pub ids: Option<T1>,
}
#[derive(Debug)]
pub struct SelectCollectionStylesNestedParams<
    T1: crate::StringSql,
    T2: crate::ArraySql<Item = T1>,
    T3: crate::ArraySql<Item = i32>,
    T4: crate::ArraySql<Item = i32>,
> {
    pub collection_id: i32,
    pub organization_id: i32,
    pub statuses: Option<T2>,
    pub pricelist_ids_to_display: Option<T3>,
    pub ids: Option<T4>,
}
#[derive(Debug)]
pub struct SelectNestedStyleSummariesParams<
    T1: crate::ArraySql<Item = i32>,
    T2: crate::StringSql,
    T3: crate::ArraySql<Item = T2>,
    T4: crate::ArraySql<Item = i32>,
    T5: crate::ArraySql<Item = i32>,
> {
    pub attributes: Option<T1>,
    pub organization_id: i32,
    pub statuses: Option<T3>,
    pub ids: Option<T4>,
    pub categories: Option<T5>,
}
#[derive(Debug)]
pub struct GetStyleIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetStyleRefsParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertStyleParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::JsonSql,
    T5: crate::JsonSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
> {
    pub organization_id: i32,
    pub slug: T1,
    pub external_id: Option<T2>,
    pub number: T3,
    pub name: T4,
    pub description: T5,
    pub core: Option<bool>,
    pub country_of_origin: Option<T6>,
    pub tariff_no: Option<T7>,
    pub net_weight: rust_decimal::Decimal,
    pub gross_weight: rust_decimal::Decimal,
    pub unit_volume: rust_decimal::Decimal,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateStyleParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::JsonSql,
    T5: crate::JsonSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
> {
    pub slug: Option<T1>,
    pub external_id: Option<T2>,
    pub number: Option<T3>,
    pub name: Option<T4>,
    pub description: Option<T5>,
    pub core: Option<bool>,
    pub country_of_origin: Option<T6>,
    pub tariff_no: Option<T7>,
    pub net_weight: Option<rust_decimal::Decimal>,
    pub gross_weight: Option<rust_decimal::Decimal>,
    pub unit_volume: Option<rust_decimal::Decimal>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteStyleParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StyleRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub number: String,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub description: serde_json::Value,
    pub core: Option<bool>,
    pub country_of_origin: Option<String>,
    pub tariff_no: Option<String>,
    pub unit_volume: rust_decimal::Decimal,
    pub gross_weight: rust_decimal::Decimal,
    pub net_weight: rust_decimal::Decimal,
    pub categories: serde_json::Value,
    pub attributes: serde_json::Value,
}
pub struct StyleRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub number: &'a str,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub description: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub core: Option<bool>,
    pub country_of_origin: Option<&'a str>,
    pub tariff_no: Option<&'a str>,
    pub unit_volume: rust_decimal::Decimal,
    pub gross_weight: rust_decimal::Decimal,
    pub net_weight: rust_decimal::Decimal,
    pub categories: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub attributes: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<StyleRowBorrowed<'a>> for StyleRow {
    fn from(
        StyleRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            number,
            name,
            created_by,
            created_at,
            updated_at,
            description,
            core,
            country_of_origin,
            tariff_no,
            unit_volume,
            gross_weight,
            net_weight,
            categories,
            attributes,
        }: StyleRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            number: number.into(),
            name: serde_json::from_str(name.0.get()).unwrap(),
            created_by,
            created_at,
            updated_at,
            description: serde_json::from_str(description.0.get()).unwrap(),
            core,
            country_of_origin: country_of_origin.map(|v| v.into()),
            tariff_no: tariff_no.map(|v| v.into()),
            unit_volume,
            gross_weight,
            net_weight,
            categories: serde_json::from_str(categories.0.get()).unwrap(),
            attributes: serde_json::from_str(attributes.0.get()).unwrap(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NestedStyleRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub number: String,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub description: serde_json::Value,
    pub core: Option<bool>,
    pub country_of_origin: Option<String>,
    pub tariff_no: Option<String>,
    pub unit_volume: rust_decimal::Decimal,
    pub gross_weight: rust_decimal::Decimal,
    pub net_weight: rust_decimal::Decimal,
    pub is_new: Option<bool>,
    pub colors: serde_json::Value,
    pub prices: serde_json::Value,
    pub attributes: serde_json::Value,
    pub categories: serde_json::Value,
}
pub struct NestedStyleRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub number: &'a str,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub description: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub core: Option<bool>,
    pub country_of_origin: Option<&'a str>,
    pub tariff_no: Option<&'a str>,
    pub unit_volume: rust_decimal::Decimal,
    pub gross_weight: rust_decimal::Decimal,
    pub net_weight: rust_decimal::Decimal,
    pub is_new: Option<bool>,
    pub colors: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub prices: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub attributes: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub categories: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<NestedStyleRowBorrowed<'a>> for NestedStyleRow {
    fn from(
        NestedStyleRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            number,
            name,
            created_by,
            created_at,
            updated_at,
            description,
            core,
            country_of_origin,
            tariff_no,
            unit_volume,
            gross_weight,
            net_weight,
            is_new,
            colors,
            prices,
            attributes,
            categories,
        }: NestedStyleRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            number: number.into(),
            name: serde_json::from_str(name.0.get()).unwrap(),
            created_by,
            created_at,
            updated_at,
            description: serde_json::from_str(description.0.get()).unwrap(),
            core,
            country_of_origin: country_of_origin.map(|v| v.into()),
            tariff_no: tariff_no.map(|v| v.into()),
            unit_volume,
            gross_weight,
            net_weight,
            is_new,
            colors: serde_json::from_str(colors.0.get()).unwrap(),
            prices: serde_json::from_str(prices.0.get()).unwrap(),
            attributes: serde_json::from_str(attributes.0.get()).unwrap(),
            categories: serde_json::from_str(categories.0.get()).unwrap(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NestedStyleSummaryRow {
    pub id: i32,
    pub name: serde_json::Value,
    pub number: String,
    pub colors: serde_json::Value,
}
pub struct NestedStyleSummaryRowBorrowed<'a> {
    pub id: i32,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub number: &'a str,
    pub colors: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<NestedStyleSummaryRowBorrowed<'a>> for NestedStyleSummaryRow {
    fn from(
        NestedStyleSummaryRowBorrowed {
            id,
            name,
            number,
            colors,
        }: NestedStyleSummaryRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: serde_json::from_str(name.0.get()).unwrap(),
            number: number.into(),
            colors: serde_json::from_str(colors.0.get()).unwrap(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct StyleRefs {
    pub id: i32,
    pub external_id: Option<String>,
    pub slug: String,
}
pub struct StyleRefsBorrowed<'a> {
    pub id: i32,
    pub external_id: Option<&'a str>,
    pub slug: &'a str,
}
impl<'a> From<StyleRefsBorrowed<'a>> for StyleRefs {
    fn from(
        StyleRefsBorrowed {
            id,
            external_id,
            slug,
        }: StyleRefsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            external_id: external_id.map(|v| v.into()),
            slug: slug.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct StyleRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> StyleRowBorrowed,
    mapper: fn(StyleRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> StyleRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(StyleRowBorrowed) -> R) -> StyleRowQuery<'a, C, R, N> {
        StyleRowQuery {
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
pub struct NestedStyleRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> NestedStyleRowBorrowed,
    mapper: fn(NestedStyleRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> NestedStyleRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(NestedStyleRowBorrowed) -> R,
    ) -> NestedStyleRowQuery<'a, C, R, N> {
        NestedStyleRowQuery {
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
pub struct NestedStyleSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> NestedStyleSummaryRowBorrowed,
    mapper: fn(NestedStyleSummaryRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> NestedStyleSummaryRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(NestedStyleSummaryRowBorrowed) -> R,
    ) -> NestedStyleSummaryRowQuery<'a, C, R, N> {
        NestedStyleSummaryRowQuery {
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
pub struct StyleRefsQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> StyleRefsBorrowed,
    mapper: fn(StyleRefsBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> StyleRefsQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(StyleRefsBorrowed) -> R) -> StyleRefsQuery<'a, C, R, N> {
        StyleRefsQuery {
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
pub fn select_styles() -> SelectStylesStmt {
    SelectStylesStmt(crate::client::async_::Stmt::new(
        "SELECT
    style.*,
    coalesce(joined_categories.json_data, '[]') AS \"categories\",
    coalesce(joined_attributes.json_data, '[]') AS \"attributes\"
FROM
    style
LEFT JOIN (
    SELECT
        style_attribute.style_id,
        json_agg(
            json_build_object(
                'id',
                \"attribute\".id,
                'title',
                \"attribute\".title,
                'description',
                \"attribute\".description,
                'slug',
                \"attribute\".slug,
                'external_id',
                \"attribute\".external_id,
                'type',
                json_build_object(
                    'id',
                    attributetype.id,
                    'name',
                    attributetype.name,
                    'slug',
                    attributetype.slug,
                    'external_id',
                    attributetype.external_id
                )
            )
            ORDER BY
                attributetype.name,
                \"attribute\".title
        ) FILTER (
            WHERE
            \"attribute\".id IS NOT NULL
        ) AS json_data
    FROM
        attribute
    INNER JOIN attributetype ON attributetype.id = \"attribute\".type_id
    INNER JOIN style_attribute ON \"attribute\".id = style_attribute.attribute_id
    WHERE
        \"attribute\".organization_id = $1
    GROUP BY
        style_attribute.style_id
    ) AS joined_attributes ON joined_attributes.style_id = style.id
LEFT JOIN (
    SELECT
        style_category.style_id,
        json_agg(category.*) FILTER (
            WHERE
            category.id IS NOT NULL
        ) AS \"json_data\"
    FROM
        category
    INNER JOIN style_category ON category.id = style_category.category_id
    WHERE
        category.organization_id = $1
    GROUP BY
        style_category.style_id
    ) AS joined_categories ON joined_categories.style_id = style.id
WHERE
    style.organization_id = $1
    AND $2::int[] IS NULL OR style.id = any($2)
ORDER BY
    style.number",
    ))
}
pub struct SelectStylesStmt(crate::client::async_::Stmt);
impl SelectStylesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        ids: &'a Option<T1>,
    ) -> StyleRowQuery<'a, C, StyleRow, 2> {
        StyleRowQuery {
            client,
            params: [organization_id, ids],
            stmt: &mut self.0,
            extractor: |row| StyleRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                number: row.get(4),
                name: row.get(5),
                created_by: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                description: row.get(9),
                core: row.get(10),
                country_of_origin: row.get(11),
                tariff_no: row.get(12),
                unit_volume: row.get(13),
                gross_weight: row.get(14),
                net_weight: row.get(15),
                categories: row.get(16),
                attributes: row.get(17),
            },
            mapper: |it| <StyleRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, SelectStylesParams<T1>, StyleRowQuery<'a, C, StyleRow, 2>, C>
    for SelectStylesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectStylesParams<T1>,
    ) -> StyleRowQuery<'a, C, StyleRow, 2> {
        self.bind(client, &params.organization_id, &params.ids)
    }
}
pub fn select_collection_styles_nested() -> SelectCollectionStylesNestedStmt {
    SelectCollectionStylesNestedStmt(crate::client::async_::Stmt::new(
        "WITH new_styles AS (
    SELECT
        new_collection_style.style_id,
        new_collection_style.is_new
    FROM
        new_collection_style
    WHERE
        new_collection_style.collection_id = $1
),

new_colors AS (
    SELECT
        new_collection_color.color_id,
        new_collection_color.is_new
    FROM
        new_collection_color
    WHERE
        new_collection_color.collection_id = $1
)

SELECT
    style.*,
    new_styles.is_new,
    coalesce(joined_colors.json_data, '[]') AS \"colors\",
    coalesce(joined_prices.json_data, '[]') AS \"prices\",
    coalesce(joined_attributes.json_data, '[]') AS \"attributes\",
    coalesce(joined_categories.json_data, '[]') AS \"categories\"
FROM
    style
LEFT JOIN new_styles ON new_styles.style_id = style.id
INNER JOIN (
        SELECT
            color.style_id,
            json_agg(
                json_build_object(
                    'id',
                    color.id,
                    'number',
                    color.number,
                    'name',
                    color.name,
                    'slug',
                    color.slug,
                    'external_id',
                    color.external_id,
                    'sizes', coalesce(joined_sizes.json_data, '[]'),
                    'images', coalesce(joined_images.json_data, '[]'),
                    'is_new',
                    new_colors.is_new
                )
                ORDER BY
                    color.number
            ) FILTER (
                WHERE
                color.id IS NOT NULL
            ) AS json_data
        FROM
            color
        LEFT JOIN new_colors ON new_colors.color_id = color.id
        INNER JOIN (
            SELECT
                size.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        size.id,
                        'number',
                        size.number,
                        'position',
                        size.position,
                        'name',
                        size.name,
                        'service_item',
                        size.service_item,
                        'delivery_period',
                        size.delivery_period,
                        'ean_code',
                        size.ean_code,
                        'status',
                        size.status,
                        'slug',
                        size.slug,
                        'external_id',
                        size.external_id
                    )
                    ORDER BY
                        size.number
                ) FILTER (
                    WHERE
                    size.id IS NOT NULL
                ) AS json_data
            FROM
                size
            INNER JOIN color ON size.color_id = color.id
            INNER JOIN style ON color.style_id = style.id
            INNER JOIN size_collection ON size_collection.size_id = size.id
            WHERE
                size.organization_id = $2
                AND size_collection.collection_id = $1
                AND (
                    $3::text[] IS NULL OR size.status = any($3)
                )
            GROUP BY
                size.color_id
            ) AS joined_sizes ON joined_sizes.color_id = color.id
        LEFT JOIN (
            SELECT
                image.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        image.id,
                        'url',
                        image.url,
                        'external_id',
                        image.external_id
                    )
                    ORDER BY
                        image.position ASC,
                        image.uploaded_at DESC
                ) AS json_data
            FROM
                image
            WHERE
                image.organization_id = $2
            GROUP BY
                image.color_id
            ) AS joined_images ON joined_images.color_id = color.id
        WHERE
            color.organization_id = $2
        GROUP BY
            color.style_id
    ) AS joined_colors ON joined_colors.style_id = style.id
LEFT JOIN (
        SELECT
            price.style_id,
            json_agg(
                json_build_object(
                    'id',
                    price.id,
                    'type',
                    price.type,
                    'uom',
                    price.uom,
                    'currency',
                    price.currency,
                    'amount',
                    price.amount,
                    'start',
                    price.start,
                    'end',
                    price.end,
                    'list',
                    json_build_object(
                        'id',
                        pricelist.id,
                        'slug',
                        pricelist.slug,
                        'external_id',
                        pricelist.external_id,
                        'name',
                        pricelist.name
                    )
                )
                ORDER BY
                    price.type,
                    pricelist.name,
                    price.\"start\"
            ) FILTER (
                WHERE
                price.id IS NOT NULL
            ) AS json_data
        FROM
            price
        INNER JOIN pricelist ON pricelist.id = price.list_id
        INNER JOIN collection_pricelist ON
            collection_pricelist.pricelist_id = pricelist.id
        WHERE
            price.organization_id = $2
            AND (
                $4::int[] IS NULL
                OR pricelist.id = any($4)
            )
            AND collection_pricelist.collection_id = $1
            AND (
                collection_pricelist.price_date
                BETWEEN price.\"start\" AND price.\"end\"
            )
        GROUP BY
            price.style_id
    ) AS joined_prices ON joined_prices.style_id = style.id
LEFT JOIN (
        SELECT
            style_attribute.style_id,
            json_agg(
                json_build_object(
                    'id',
                    \"attribute\".id,
                    'title',
                    \"attribute\".title,
                    'description',
                    \"attribute\".description,
                    'slug',
                    \"attribute\".slug,
                    'external_id',
                    \"attribute\".external_id,
                    'type',
                    json_build_object(
                        'id',
                        attributetype.id,
                        'name',
                        attributetype.name,
                        'slug',
                        attributetype.slug,
                        'external_id',
                        attributetype.external_id
                    )
                )
                ORDER BY
                    attributetype.name,
                    \"attribute\".title
            ) FILTER (
                WHERE
                \"attribute\".id IS NOT NULL
            ) AS json_data
        FROM
            attribute
        INNER JOIN attributetype ON attributetype.id = \"attribute\".type_id
        INNER JOIN style_attribute ON \"attribute\".id = style_attribute.attribute_id
        WHERE
            \"attribute\".organization_id = $2
        GROUP BY
            style_attribute.style_id
    ) AS joined_attributes ON joined_attributes.style_id = style.id
LEFT JOIN (
        SELECT
            style_category.style_id,
            json_agg(
                json_build_object(
                    'id',
                    category.id,
                    'slug',
                    category.slug,
                    'name',
                    category.name,
                    'external_id',
                    category.external_id
                )
            ) FILTER (
                WHERE
                category.id IS NOT NULL
            ) AS \"json_data\"
        FROM
            category
        INNER JOIN style_category ON category.id = style_category.category_id
        WHERE
            category.organization_id = $2
        GROUP BY
            style_category.style_id
    ) AS joined_categories ON joined_categories.style_id = style.id
WHERE
    ($5::int[] IS NULL OR style.id = any($5))
ORDER BY
    style.number",
    ))
}
pub struct SelectCollectionStylesNestedStmt(crate::client::async_::Stmt);
impl SelectCollectionStylesNestedStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = T1>,
        T3: crate::ArraySql<Item = i32>,
        T4: crate::ArraySql<Item = i32>,
    >(
        &'a mut self,
        client: &'a C,
        collection_id: &'a i32,
        organization_id: &'a i32,
        statuses: &'a Option<T2>,
        pricelist_ids_to_display: &'a Option<T3>,
        ids: &'a Option<T4>,
    ) -> NestedStyleRowQuery<'a, C, NestedStyleRow, 5> {
        NestedStyleRowQuery {
            client,
            params: [
                collection_id,
                organization_id,
                statuses,
                pricelist_ids_to_display,
                ids,
            ],
            stmt: &mut self.0,
            extractor: |row| NestedStyleRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                number: row.get(4),
                name: row.get(5),
                created_by: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                description: row.get(9),
                core: row.get(10),
                country_of_origin: row.get(11),
                tariff_no: row.get(12),
                unit_volume: row.get(13),
                gross_weight: row.get(14),
                net_weight: row.get(15),
                is_new: row.get(16),
                colors: row.get(17),
                prices: row.get(18),
                attributes: row.get(19),
                categories: row.get(20),
            },
            mapper: |it| <NestedStyleRow>::from(it),
        }
    }
}
impl<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = T1>,
        T3: crate::ArraySql<Item = i32>,
        T4: crate::ArraySql<Item = i32>,
    >
    crate::client::async_::Params<
        'a,
        SelectCollectionStylesNestedParams<T1, T2, T3, T4>,
        NestedStyleRowQuery<'a, C, NestedStyleRow, 5>,
        C,
    > for SelectCollectionStylesNestedStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectCollectionStylesNestedParams<T1, T2, T3, T4>,
    ) -> NestedStyleRowQuery<'a, C, NestedStyleRow, 5> {
        self.bind(
            client,
            &params.collection_id,
            &params.organization_id,
            &params.statuses,
            &params.pricelist_ids_to_display,
            &params.ids,
        )
    }
}
pub fn select_nested_style_summaries() -> SelectNestedStyleSummariesStmt {
    SelectNestedStyleSummariesStmt(crate::client::async_::Stmt::new(
        "WITH attribute_matches AS (
    SELECT styles_matching_attributes($1)
)

SELECT
    style.id,
    style.name,
    style.number,
    coalesce(joined_colors.json_data, '[]') AS \"colors\"
FROM
    style
INNER JOIN (
        SELECT
            color.style_id,
            json_agg(
                json_build_object(
                    'id',
                    color.id,
                    'number',
                    color.number,
                    'name',
                    color.name,
                    'sizes', coalesce(joined_sizes.json_data, '[]'),
                    'primary_image', primary_image.json_data
                )
                ORDER BY
                    color.number
            ) FILTER (
                WHERE
                color.id IS NOT NULL
            ) AS json_data
        FROM
            color
        INNER JOIN (
            SELECT
                size.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        size.id,
                        'number',
                        size.number,
                        'name',
                        size.name
                    )
                    ORDER BY
                        size.number
                ) FILTER (
                    WHERE
                    size.id IS NOT NULL
                ) AS json_data
            FROM
                size
            INNER JOIN color ON size.color_id = color.id
            WHERE
                size.organization_id = $2
                AND ($3::text[] IS NULL OR size.status = any($3))
            GROUP BY
                size.color_id
            ) AS joined_sizes ON joined_sizes.color_id = color.id
        LEFT JOIN (
            -- We utilize distinct here because there might be multiple rows with
            -- `position = 1` for a given `color_id`.
            SELECT DISTINCT ON (image.color_id)
                image.color_id,
                json_build_object(
                    'id',
                    image.id,
                    'url',
                    image.url,
                    'external_id',
                    image.external_id
                ) AS json_data
            FROM
                image
            WHERE
                image.organization_id = $2
                AND image.position = 1
            ) AS primary_image ON primary_image.color_id = color.id
        WHERE
            color.organization_id = $2
        GROUP BY
            color.style_id
    ) AS joined_colors ON joined_colors.style_id = style.id
LEFT JOIN (
    SELECT
        style_category.style_id,
        array_agg(style_category.category_id) AS category_ids
    FROM style_category
    GROUP BY style_category.style_id
) AS style_categories ON style_categories.style_id = style.id
WHERE
    style.organization_id = $2
    AND ($4::int[] IS NULL OR style.id = any($4))
    AND ($5::int[] IS NULL OR style_categories.category_ids && $5)
    AND ($1::int[] IS NULL OR style.id IN (SELECT * FROM attribute_matches))
ORDER BY
    style.number",
    ))
}
pub struct SelectNestedStyleSummariesStmt(crate::client::async_::Stmt);
impl SelectNestedStyleSummariesStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::ArraySql<Item = i32>,
        T2: crate::StringSql,
        T3: crate::ArraySql<Item = T2>,
        T4: crate::ArraySql<Item = i32>,
        T5: crate::ArraySql<Item = i32>,
    >(
        &'a mut self,
        client: &'a C,
        attributes: &'a Option<T1>,
        organization_id: &'a i32,
        statuses: &'a Option<T3>,
        ids: &'a Option<T4>,
        categories: &'a Option<T5>,
    ) -> NestedStyleSummaryRowQuery<'a, C, NestedStyleSummaryRow, 5> {
        NestedStyleSummaryRowQuery {
            client,
            params: [attributes, organization_id, statuses, ids, categories],
            stmt: &mut self.0,
            extractor: |row| NestedStyleSummaryRowBorrowed {
                id: row.get(0),
                name: row.get(1),
                number: row.get(2),
                colors: row.get(3),
            },
            mapper: |it| <NestedStyleSummaryRow>::from(it),
        }
    }
}
impl<
        'a,
        C: GenericClient,
        T1: crate::ArraySql<Item = i32>,
        T2: crate::StringSql,
        T3: crate::ArraySql<Item = T2>,
        T4: crate::ArraySql<Item = i32>,
        T5: crate::ArraySql<Item = i32>,
    >
    crate::client::async_::Params<
        'a,
        SelectNestedStyleSummariesParams<T1, T2, T3, T4, T5>,
        NestedStyleSummaryRowQuery<'a, C, NestedStyleSummaryRow, 5>,
        C,
    > for SelectNestedStyleSummariesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectNestedStyleSummariesParams<T1, T2, T3, T4, T5>,
    ) -> NestedStyleSummaryRowQuery<'a, C, NestedStyleSummaryRow, 5> {
        self.bind(
            client,
            &params.attributes,
            &params.organization_id,
            &params.statuses,
            &params.ids,
            &params.categories,
        )
    }
}
pub fn get_style_id() -> GetStyleIdStmt {
    GetStyleIdStmt(crate::client::async_::Stmt::new(
        "SELECT style.id
FROM
    style
WHERE
    style.organization_id = $1
    AND (
        (style.id = coalesce($2, -1))
        OR (
            style.external_id = coalesce($3, '___NON_EXISTING___')
            OR (style.slug = coalesce($4, '___NON_EXISTING___'))
        )
    )",
    ))
}
pub struct GetStyleIdStmt(crate::client::async_::Stmt);
impl GetStyleIdStmt {
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
    crate::client::async_::Params<'a, GetStyleIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetStyleIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetStyleIdParams<T1, T2>,
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
pub fn get_style_refs() -> GetStyleRefsStmt {
    GetStyleRefsStmt(crate::client::async_::Stmt::new(
        "SELECT
    style.id,
    style.external_id,
    style.slug
FROM
    style
WHERE
    style.organization_id = $1
    AND (
        (style.id = coalesce($2, -1))
        OR (
            style.external_id = coalesce($3, '___NON_EXISTING___')
            OR (style.slug = coalesce($4, '___NON_EXISTING___'))
        )
    )",
    ))
}
pub struct GetStyleRefsStmt(crate::client::async_::Stmt);
impl GetStyleRefsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> StyleRefsQuery<'a, C, StyleRefs, 4> {
        StyleRefsQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| StyleRefsBorrowed {
                id: row.get(0),
                external_id: row.get(1),
                slug: row.get(2),
            },
            mapper: |it| <StyleRefs>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        GetStyleRefsParams<T1, T2>,
        StyleRefsQuery<'a, C, StyleRefs, 4>,
        C,
    > for GetStyleRefsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetStyleRefsParams<T1, T2>,
    ) -> StyleRefsQuery<'a, C, StyleRefs, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn insert_style() -> InsertStyleStmt {
    InsertStyleStmt(crate::client::async_::Stmt::new(
        "INSERT INTO
style (
    organization_id,
    slug,
    external_id,
    number,
    name,
    description,
    core,
    country_of_origin,
    tariff_no,
    net_weight,
    gross_weight,
    unit_volume,
    created_by
)
VALUES
(
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11,
    $12,
    $13
)
RETURNING
id",
    ))
}
pub struct InsertStyleStmt(crate::client::async_::Stmt);
impl InsertStyleStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::JsonSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        slug: &'a T1,
        external_id: &'a Option<T2>,
        number: &'a T3,
        name: &'a T4,
        description: &'a T5,
        core: &'a Option<bool>,
        country_of_origin: &'a Option<T6>,
        tariff_no: &'a Option<T7>,
        net_weight: &'a rust_decimal::Decimal,
        gross_weight: &'a rust_decimal::Decimal,
        unit_volume: &'a rust_decimal::Decimal,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 13> {
        I32Query {
            client,
            params: [
                organization_id,
                slug,
                external_id,
                number,
                name,
                description,
                core,
                country_of_origin,
                tariff_no,
                net_weight,
                gross_weight,
                unit_volume,
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
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::JsonSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        InsertStyleParams<T1, T2, T3, T4, T5, T6, T7>,
        I32Query<'a, C, i32, 13>,
        C,
    > for InsertStyleStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertStyleParams<T1, T2, T3, T4, T5, T6, T7>,
    ) -> I32Query<'a, C, i32, 13> {
        self.bind(
            client,
            &params.organization_id,
            &params.slug,
            &params.external_id,
            &params.number,
            &params.name,
            &params.description,
            &params.core,
            &params.country_of_origin,
            &params.tariff_no,
            &params.net_weight,
            &params.gross_weight,
            &params.unit_volume,
            &params.created_by,
        )
    }
}
pub fn update_style() -> UpdateStyleStmt {
    UpdateStyleStmt(crate::client::async_::Stmt::new(
        "UPDATE
style
SET
    slug = coalesce($1, slug),
    external_id = coalesce($2, external_id),
    number = coalesce($3, number),
    name = coalesce($4, name),
    description = coalesce($5, description),
    core = coalesce($6, core),
    country_of_origin = coalesce($7, country_of_origin),
    tariff_no = coalesce($8, tariff_no),
    net_weight = coalesce($9, net_weight),
    gross_weight = coalesce($10, gross_weight),
    unit_volume = coalesce($11, unit_volume)
WHERE
    id = $12
RETURNING
id",
    ))
}
pub struct UpdateStyleStmt(crate::client::async_::Stmt);
impl UpdateStyleStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::JsonSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        slug: &'a Option<T1>,
        external_id: &'a Option<T2>,
        number: &'a Option<T3>,
        name: &'a Option<T4>,
        description: &'a Option<T5>,
        core: &'a Option<bool>,
        country_of_origin: &'a Option<T6>,
        tariff_no: &'a Option<T7>,
        net_weight: &'a Option<rust_decimal::Decimal>,
        gross_weight: &'a Option<rust_decimal::Decimal>,
        unit_volume: &'a Option<rust_decimal::Decimal>,
        id: &'a i32,
    ) -> I32Query<'a, C, i32, 12> {
        I32Query {
            client,
            params: [
                slug,
                external_id,
                number,
                name,
                description,
                core,
                country_of_origin,
                tariff_no,
                net_weight,
                gross_weight,
                unit_volume,
                id,
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
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::JsonSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        UpdateStyleParams<T1, T2, T3, T4, T5, T6, T7>,
        I32Query<'a, C, i32, 12>,
        C,
    > for UpdateStyleStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateStyleParams<T1, T2, T3, T4, T5, T6, T7>,
    ) -> I32Query<'a, C, i32, 12> {
        self.bind(
            client,
            &params.slug,
            &params.external_id,
            &params.number,
            &params.name,
            &params.description,
            &params.core,
            &params.country_of_origin,
            &params.tariff_no,
            &params.net_weight,
            &params.gross_weight,
            &params.unit_volume,
            &params.id,
        )
    }
}
pub fn delete_style() -> DeleteStyleStmt {
    DeleteStyleStmt(crate::client::async_::Stmt::new(
        "DELETE FROM
style
WHERE
    organization_id = $1
    AND id = $2
RETURNING
id",
    ))
}
pub struct DeleteStyleStmt(crate::client::async_::Stmt);
impl DeleteStyleStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a i32,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [organization_id, id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient>
    crate::client::async_::Params<'a, DeleteStyleParams, I32Query<'a, C, i32, 2>, C>
    for DeleteStyleStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteStyleParams,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.organization_id, &params.id)
    }
}
