#[derive(Debug)]
pub struct GetSizeIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetSizeParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertSizeParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::JsonSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
> {
    pub color_id: i32,
    pub slug: T1,
    pub external_id: Option<T2>,
    pub number: T3,
    pub name: T4,
    pub service_item: Option<bool>,
    pub delivery_period: Option<chrono::NaiveDate>,
    pub ean_code: Option<T5>,
    pub status: Option<T6>,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateSizeParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::JsonSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
> {
    pub color_id: i32,
    pub slug: Option<T1>,
    pub external_id: Option<T2>,
    pub number: Option<T3>,
    pub position: Option<i16>,
    pub name: Option<T4>,
    pub service_item: Option<bool>,
    pub delivery_period: Option<chrono::NaiveDate>,
    pub ean_code: Option<T5>,
    pub status: Option<T6>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteSizeParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SizeRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub color_id: i32,
    pub number: String,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub service_item: Option<bool>,
    pub delivery_period: Option<chrono::NaiveDate>,
    pub ean_code: Option<String>,
    pub status: Option<String>,
    pub position: i16,
    pub color: serde_json::Value,
}
pub struct SizeRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub color_id: i32,
    pub number: &'a str,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub service_item: Option<bool>,
    pub delivery_period: Option<chrono::NaiveDate>,
    pub ean_code: Option<&'a str>,
    pub status: Option<&'a str>,
    pub position: i16,
    pub color: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<SizeRowBorrowed<'a>> for SizeRow {
    fn from(
        SizeRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            color_id,
            number,
            name,
            created_by,
            created_at,
            updated_at,
            service_item,
            delivery_period,
            ean_code,
            status,
            position,
            color,
        }: SizeRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            color_id,
            number: number.into(),
            name: serde_json::from_str(name.0.get()).unwrap(),
            created_by,
            created_at,
            updated_at,
            service_item,
            delivery_period,
            ean_code: ean_code.map(|v| v.into()),
            status: status.map(|v| v.into()),
            position,
            color: serde_json::from_str(color.0.get()).unwrap(),
        }
    }
} // This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct SizeRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> SizeRowBorrowed,
    mapper: fn(SizeRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> SizeRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(SizeRowBorrowed) -> R) -> SizeRowQuery<'a, C, R, N> {
        SizeRowQuery {
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
pub fn list_sizes() -> ListSizesStmt {
    ListSizesStmt(crate::client::async_::Stmt::new(
        "SELECT
    size.*,
    jsonb_build_object(
        'id',
        color.id,
        'style',
        jsonb_build_object(
            'id',
            style.id,
            'number',
            style.number,
            'name',
            style.name,
            'slug',
            style.slug,
            'external_id',
            style.external_id
        ),
        'number',
        color.number,
        'name',
        color.name,
        'slug',
        color.slug,
        'external_id',
        color.external_id
    ) AS \"color\"
FROM
    size
INNER JOIN color ON size.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    size.organization_id = $1
ORDER BY
    size.id",
    ))
}
pub struct ListSizesStmt(crate::client::async_::Stmt);
impl ListSizesStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> SizeRowQuery<'a, C, SizeRow, 1> {
        SizeRowQuery {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| SizeRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                color_id: row.get(4),
                number: row.get(5),
                name: row.get(6),
                created_by: row.get(7),
                created_at: row.get(8),
                updated_at: row.get(9),
                service_item: row.get(10),
                delivery_period: row.get(11),
                ean_code: row.get(12),
                status: row.get(13),
                position: row.get(14),
                color: row.get(15),
            },
            mapper: |it| <SizeRow>::from(it),
        }
    }
}
pub fn get_size_id() -> GetSizeIdStmt {
    GetSizeIdStmt(crate::client::async_::Stmt::new(
        "SELECT size.id
FROM
    size
WHERE
    size.organization_id = $1
    AND (
        size.id = coalesce($2, -1)
        OR size.external_id = coalesce($3, '___NON_EXISTING___')
        OR size.slug = coalesce($4, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetSizeIdStmt(crate::client::async_::Stmt);
impl GetSizeIdStmt {
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
    crate::client::async_::Params<'a, GetSizeIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetSizeIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetSizeIdParams<T1, T2>,
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
pub fn get_size() -> GetSizeStmt {
    GetSizeStmt(crate::client::async_::Stmt::new(
        "SELECT
    size.*,
    jsonb_build_object(
        'id',
        color.id,
        'style',
        jsonb_build_object(
            'id',
            style.id,
            'number',
            style.number,
            'name',
            style.name,
            'slug',
            style.slug,
            'external_id',
            style.external_id
        ),
        'number',
        color.number,
        'name',
        color.name,
        'slug',
        color.slug,
        'external_id',
        color.external_id
    ) AS \"color\"
FROM
    size
INNER JOIN color ON size.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    size.organization_id = $1
    AND (
        size.id = coalesce($2, -1)
        OR size.external_id = coalesce($3, '___NON_EXISTING___')
        OR size.slug = coalesce($4, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetSizeStmt(crate::client::async_::Stmt);
impl GetSizeStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> SizeRowQuery<'a, C, SizeRow, 4> {
        SizeRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| SizeRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                color_id: row.get(4),
                number: row.get(5),
                name: row.get(6),
                created_by: row.get(7),
                created_at: row.get(8),
                updated_at: row.get(9),
                service_item: row.get(10),
                delivery_period: row.get(11),
                ean_code: row.get(12),
                status: row.get(13),
                position: row.get(14),
                color: row.get(15),
            },
            mapper: |it| <SizeRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<'a, GetSizeParams<T1, T2>, SizeRowQuery<'a, C, SizeRow, 4>, C>
    for GetSizeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetSizeParams<T1, T2>,
    ) -> SizeRowQuery<'a, C, SizeRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn insert_size() -> InsertSizeStmt {
    InsertSizeStmt(crate::client::async_::Stmt::new(
        "INSERT INTO size (
    color_id,
    slug,
    external_id,
    number,
    name,
    service_item,
    delivery_period,
    ean_code,
    status,
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
pub struct InsertSizeStmt(crate::client::async_::Stmt);
impl InsertSizeStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        color_id: &'a i32,
        slug: &'a T1,
        external_id: &'a Option<T2>,
        number: &'a T3,
        name: &'a T4,
        service_item: &'a Option<bool>,
        delivery_period: &'a Option<chrono::NaiveDate>,
        ean_code: &'a Option<T5>,
        status: &'a Option<T6>,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 11> {
        I32Query {
            client,
            params: [
                color_id,
                slug,
                external_id,
                number,
                name,
                service_item,
                delivery_period,
                ean_code,
                status,
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
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        InsertSizeParams<T1, T2, T3, T4, T5, T6>,
        I32Query<'a, C, i32, 11>,
        C,
    > for InsertSizeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertSizeParams<T1, T2, T3, T4, T5, T6>,
    ) -> I32Query<'a, C, i32, 11> {
        self.bind(
            client,
            &params.color_id,
            &params.slug,
            &params.external_id,
            &params.number,
            &params.name,
            &params.service_item,
            &params.delivery_period,
            &params.ean_code,
            &params.status,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_size() -> UpdateSizeStmt {
    UpdateSizeStmt(crate::client::async_::Stmt::new(
        "UPDATE
size
SET
    color_id = coalesce($1, color_id),
    slug = coalesce($2, slug),
    external_id = coalesce($3, external_id),
    number = coalesce($4, number),
    position = coalesce($5, position),
    name = coalesce($6, name),
    service_item = coalesce($7, service_item),
    delivery_period = coalesce($8, delivery_period),
    ean_code = coalesce($9, ean_code),
    status = coalesce($10, status)
WHERE
    id = $11",
    ))
}
pub struct UpdateSizeStmt(crate::client::async_::Stmt);
impl UpdateSizeStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        color_id: &'a i32,
        slug: &'a Option<T1>,
        external_id: &'a Option<T2>,
        number: &'a Option<T3>,
        position: &'a Option<i16>,
        name: &'a Option<T4>,
        service_item: &'a Option<bool>,
        delivery_period: &'a Option<chrono::NaiveDate>,
        ean_code: &'a Option<T5>,
        status: &'a Option<T6>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[
                    color_id,
                    slug,
                    external_id,
                    number,
                    position,
                    name,
                    service_item,
                    delivery_period,
                    ean_code,
                    status,
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
        T4: crate::JsonSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        UpdateSizeParams<T1, T2, T3, T4, T5, T6>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSizeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateSizeParams<T1, T2, T3, T4, T5, T6>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.color_id,
            &params.slug,
            &params.external_id,
            &params.number,
            &params.position,
            &params.name,
            &params.service_item,
            &params.delivery_period,
            &params.ean_code,
            &params.status,
            &params.id,
        ))
    }
}
pub fn delete_size() -> DeleteSizeStmt {
    DeleteSizeStmt(crate::client::async_::Stmt::new(
        "DELETE FROM size
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeleteSizeStmt(crate::client::async_::Stmt);
impl DeleteSizeStmt {
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
        DeleteSizeParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteSizeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteSizeParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
