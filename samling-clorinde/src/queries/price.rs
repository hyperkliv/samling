#[derive(Debug)]
pub struct SelectPricesParams<
    T1: crate::ArraySql<Item = i32>,
    T2: crate::StringSql,
    T3: crate::ArraySql<Item = T2>,
> {
    pub organization_id: i32,
    pub ids: Option<T1>,
    pub external_ids: Option<T3>,
}
#[derive(Debug)]
pub struct GetPriceIdParams<T1: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
}
#[derive(Debug)]
pub struct InsertPriceParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub r#type: crate::types::Pricetype,
    pub uom: Option<T1>,
    pub currency: T2,
    pub amount: rust_decimal::Decimal,
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub style_id: i32,
    pub list_id: i32,
    pub external_id: Option<T3>,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdatePriceParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub r#type: Option<crate::types::Pricetype>,
    pub uom: Option<T1>,
    pub currency: Option<T2>,
    pub amount: Option<rust_decimal::Decimal>,
    pub start: Option<chrono::NaiveDate>,
    pub end: Option<chrono::NaiveDate>,
    pub style_id: Option<i32>,
    pub list_id: Option<i32>,
    pub external_id: Option<T3>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeletePriceParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PriceRow {
    pub id: i32,
    pub organization_id: i32,
    pub r#type: crate::types::Pricetype,
    pub currency: String,
    pub uom: Option<String>,
    pub list_id: i32,
    pub external_id: Option<String>,
    pub style_id: i32,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub amount: rust_decimal::Decimal,
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub style: serde_json::Value,
    pub list: serde_json::Value,
}
pub struct PriceRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub r#type: crate::types::Pricetype,
    pub currency: &'a str,
    pub uom: Option<&'a str>,
    pub list_id: i32,
    pub external_id: Option<&'a str>,
    pub style_id: i32,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub amount: rust_decimal::Decimal,
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
    pub style: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub list: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<PriceRowBorrowed<'a>> for PriceRow {
    fn from(
        PriceRowBorrowed {
            id,
            organization_id,
            r#type,
            currency,
            uom,
            list_id,
            external_id,
            style_id,
            created_by,
            created_at,
            updated_at,
            amount,
            start,
            end,
            style,
            list,
        }: PriceRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            r#type,
            currency: currency.into(),
            uom: uom.map(|v| v.into()),
            list_id,
            external_id: external_id.map(|v| v.into()),
            style_id,
            created_by,
            created_at,
            updated_at,
            amount,
            start,
            end,
            style: serde_json::from_str(style.0.get()).unwrap(),
            list: serde_json::from_str(list.0.get()).unwrap(),
        }
    }
} // This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct PriceRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> PriceRowBorrowed,
    mapper: fn(PriceRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> PriceRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(PriceRowBorrowed) -> R) -> PriceRowQuery<'a, C, R, N> {
        PriceRowQuery {
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
pub fn select_prices() -> SelectPricesStmt {
    SelectPricesStmt(crate::client::async_::Stmt::new(
        "SELECT
    price.*,
    jsonb_build_object(
        'id',
        style.id,
        'external_id',
        style.external_id,
        'slug',
        style.slug,
        'number',
        style.number,
        'name',
        style.name
    ) AS \"style\",
    jsonb_build_object(
        'id',
        pricelist.id,
        'external_id',
        pricelist.external_id,
        'slug',
        pricelist.slug,
        'name',
        pricelist.name
    ) AS \"list\"
FROM
    price
INNER JOIN style ON style.id = price.style_id
INNER JOIN pricelist ON pricelist.id = price.list_id
WHERE
    price.organization_id = $1
    AND ($2::int[] IS NULL OR price.id = any($2))
    AND ($3::text[] IS NULL OR price.external_id = any($3))
ORDER BY
    price.updated_at DESC",
    ))
}
pub struct SelectPricesStmt(crate::client::async_::Stmt);
impl SelectPricesStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::ArraySql<Item = i32>,
        T2: crate::StringSql,
        T3: crate::ArraySql<Item = T2>,
    >(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        ids: &'a Option<T1>,
        external_ids: &'a Option<T3>,
    ) -> PriceRowQuery<'a, C, PriceRow, 3> {
        PriceRowQuery {
            client,
            params: [organization_id, ids, external_ids],
            stmt: &mut self.0,
            extractor: |row| PriceRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                r#type: row.get(2),
                currency: row.get(3),
                uom: row.get(4),
                list_id: row.get(5),
                external_id: row.get(6),
                style_id: row.get(7),
                created_by: row.get(8),
                created_at: row.get(9),
                updated_at: row.get(10),
                amount: row.get(11),
                start: row.get(12),
                end: row.get(13),
                style: row.get(14),
                list: row.get(15),
            },
            mapper: |it| <PriceRow>::from(it),
        }
    }
}
impl<
        'a,
        C: GenericClient,
        T1: crate::ArraySql<Item = i32>,
        T2: crate::StringSql,
        T3: crate::ArraySql<Item = T2>,
    >
    crate::client::async_::Params<
        'a,
        SelectPricesParams<T1, T2, T3>,
        PriceRowQuery<'a, C, PriceRow, 3>,
        C,
    > for SelectPricesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectPricesParams<T1, T2, T3>,
    ) -> PriceRowQuery<'a, C, PriceRow, 3> {
        self.bind(
            client,
            &params.organization_id,
            &params.ids,
            &params.external_ids,
        )
    }
}
pub fn get_price_id() -> GetPriceIdStmt {
    GetPriceIdStmt(crate::client::async_::Stmt::new(
        "SELECT price.id
FROM
    price
WHERE
    price.organization_id = $1
    AND ($2::int IS NULL OR price.id = $2)
    AND ($3::text IS NULL OR price.external_id = $3)",
    ))
}
pub struct GetPriceIdStmt(crate::client::async_::Stmt);
impl GetPriceIdStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
    ) -> I32Query<'a, C, i32, 3> {
        I32Query {
            client,
            params: [organization_id, id, external_id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<'a, GetPriceIdParams<T1>, I32Query<'a, C, i32, 3>, C>
    for GetPriceIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetPriceIdParams<T1>,
    ) -> I32Query<'a, C, i32, 3> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
        )
    }
}
pub fn insert_price() -> InsertPriceStmt {
    InsertPriceStmt(crate::client::async_::Stmt::new(
        "INSERT INTO price (
    type,
    uom,
    currency,
    amount,
    \"start\",
    \"end\",
    style_id,
    list_id,
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
    $7,
    $8,
    $9,
    $10,
    $11)
RETURNING
id",
    ))
}
pub struct InsertPriceStmt(crate::client::async_::Stmt);
impl InsertPriceStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        r#type: &'a crate::types::Pricetype,
        uom: &'a Option<T1>,
        currency: &'a T2,
        amount: &'a rust_decimal::Decimal,
        start: &'a chrono::NaiveDate,
        end: &'a chrono::NaiveDate,
        style_id: &'a i32,
        list_id: &'a i32,
        external_id: &'a Option<T3>,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 11> {
        I32Query {
            client,
            params: [
                r#type,
                uom,
                currency,
                amount,
                start,
                end,
                style_id,
                list_id,
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
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<'a, InsertPriceParams<T1, T2, T3>, I32Query<'a, C, i32, 11>, C>
    for InsertPriceStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertPriceParams<T1, T2, T3>,
    ) -> I32Query<'a, C, i32, 11> {
        self.bind(
            client,
            &params.r#type,
            &params.uom,
            &params.currency,
            &params.amount,
            &params.start,
            &params.end,
            &params.style_id,
            &params.list_id,
            &params.external_id,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_price() -> UpdatePriceStmt {
    UpdatePriceStmt(crate::client::async_::Stmt::new(
        "UPDATE
price
SET
    type = coalesce($1, type),
    uom = coalesce($2, uom),
    currency = coalesce($3, currency),
    amount = coalesce($4, amount),
    \"start\" = coalesce($5, \"start\"),
    \"end\" = coalesce($6, \"end\"),
    style_id = coalesce($7, style_id),
    list_id = coalesce($8, list_id),
    external_id = coalesce($9, external_id)
WHERE
    id = $10",
    ))
}
pub struct UpdatePriceStmt(crate::client::async_::Stmt);
impl UpdatePriceStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        r#type: &'a Option<crate::types::Pricetype>,
        uom: &'a Option<T1>,
        currency: &'a Option<T2>,
        amount: &'a Option<rust_decimal::Decimal>,
        start: &'a Option<chrono::NaiveDate>,
        end: &'a Option<chrono::NaiveDate>,
        style_id: &'a Option<i32>,
        list_id: &'a Option<i32>,
        external_id: &'a Option<T3>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[
                    r#type,
                    uom,
                    currency,
                    amount,
                    start,
                    end,
                    style_id,
                    list_id,
                    external_id,
                    id,
                ],
            )
            .await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        UpdatePriceParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdatePriceStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdatePriceParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.r#type,
            &params.uom,
            &params.currency,
            &params.amount,
            &params.start,
            &params.end,
            &params.style_id,
            &params.list_id,
            &params.external_id,
            &params.id,
        ))
    }
}
pub fn delete_price() -> DeletePriceStmt {
    DeletePriceStmt(crate::client::async_::Stmt::new(
        "DELETE FROM price
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeletePriceStmt(crate::client::async_::Stmt);
impl DeletePriceStmt {
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
        DeletePriceParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeletePriceStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeletePriceParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
