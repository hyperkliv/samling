#[derive(Debug)]
pub struct GetAttributeTypeParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetAttributeTypeIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertAttributeTypeParams<T1: crate::JsonSql, T2: crate::StringSql, T3: crate::StringSql>
{
    pub name: T1,
    pub slug: T2,
    pub external_id: Option<T3>,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateAttributeTypeParams<T1: crate::JsonSql, T2: crate::StringSql, T3: crate::StringSql>
{
    pub name: Option<T1>,
    pub slug: Option<T2>,
    pub external_id: Option<T3>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteAttributeTypeParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeTypeRow {
    pub id: i32,
    pub organization_id: i32,
    pub name: serde_json::Value,
    pub slug: String,
    pub external_id: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct AttributeTypeRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<AttributeTypeRowBorrowed<'a>> for AttributeTypeRow {
    fn from(
        AttributeTypeRowBorrowed {
            id,
            organization_id,
            name,
            slug,
            external_id,
            created_by,
            created_at,
            updated_at,
        }: AttributeTypeRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            name: serde_json::from_str(name.0.get()).unwrap(),
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            created_by,
            created_at,
            updated_at,
        }
    }
} // This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct AttributeTypeRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> AttributeTypeRowBorrowed,
    mapper: fn(AttributeTypeRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> AttributeTypeRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(AttributeTypeRowBorrowed) -> R,
    ) -> AttributeTypeRowQuery<'a, C, R, N> {
        AttributeTypeRowQuery {
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
pub fn list_attribute_types() -> ListAttributeTypesStmt {
    ListAttributeTypesStmt(crate::client::async_::Stmt::new(
        "SELECT attributetype.*
FROM
    attributetype
WHERE
    attributetype.organization_id = $1
ORDER BY
    attributetype.updated_at DESC",
    ))
}
pub struct ListAttributeTypesStmt(crate::client::async_::Stmt);
impl ListAttributeTypesStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> AttributeTypeRowQuery<'a, C, AttributeTypeRow, 1> {
        AttributeTypeRowQuery {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| AttributeTypeRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                name: row.get(2),
                slug: row.get(3),
                external_id: row.get(4),
                created_by: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            },
            mapper: |it| <AttributeTypeRow>::from(it),
        }
    }
}
pub fn get_attribute_type() -> GetAttributeTypeStmt {
    GetAttributeTypeStmt(crate::client::async_::Stmt::new(
        "SELECT attributetype.*
FROM
    attributetype
WHERE
    attributetype.organization_id = $1
    AND ((attributetype.id = coalesce($2, -1))
        OR (
            attributetype.external_id = coalesce(
                $3, '___NON_EXISTING___'
            )
        )
        OR (attributetype.slug = coalesce($4, '___NON_EXISTING___')))",
    ))
}
pub struct GetAttributeTypeStmt(crate::client::async_::Stmt);
impl GetAttributeTypeStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> AttributeTypeRowQuery<'a, C, AttributeTypeRow, 4> {
        AttributeTypeRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| AttributeTypeRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                name: row.get(2),
                slug: row.get(3),
                external_id: row.get(4),
                created_by: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            },
            mapper: |it| <AttributeTypeRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        GetAttributeTypeParams<T1, T2>,
        AttributeTypeRowQuery<'a, C, AttributeTypeRow, 4>,
        C,
    > for GetAttributeTypeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetAttributeTypeParams<T1, T2>,
    ) -> AttributeTypeRowQuery<'a, C, AttributeTypeRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn get_attribute_type_id() -> GetAttributeTypeIdStmt {
    GetAttributeTypeIdStmt(crate::client::async_::Stmt::new(
        "SELECT attributetype.id
FROM
    attributetype
WHERE
    attributetype.organization_id = $1
    AND ((attributetype.id = coalesce($2, -1))
        OR (
            attributetype.external_id = coalesce(
                $3, '___NON_EXISTING___'
            )
        )
        OR (attributetype.slug = coalesce($4, '___NON_EXISTING___')))",
    ))
}
pub struct GetAttributeTypeIdStmt(crate::client::async_::Stmt);
impl GetAttributeTypeIdStmt {
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
    crate::client::async_::Params<'a, GetAttributeTypeIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetAttributeTypeIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetAttributeTypeIdParams<T1, T2>,
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
pub fn insert_attribute_type() -> InsertAttributeTypeStmt {
    InsertAttributeTypeStmt(crate::client::async_::Stmt::new(
        "INSERT INTO attributetype (
    name,
    slug,
    external_id,
    organization_id,
    created_by)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5)
RETURNING
id",
    ))
}
pub struct InsertAttributeTypeStmt(crate::client::async_::Stmt);
impl InsertAttributeTypeStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        name: &'a T1,
        slug: &'a T2,
        external_id: &'a Option<T3>,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 5> {
        I32Query {
            client,
            params: [name, slug, external_id, organization_id, created_by],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::JsonSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'a,
        InsertAttributeTypeParams<T1, T2, T3>,
        I32Query<'a, C, i32, 5>,
        C,
    > for InsertAttributeTypeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertAttributeTypeParams<T1, T2, T3>,
    ) -> I32Query<'a, C, i32, 5> {
        self.bind(
            client,
            &params.name,
            &params.slug,
            &params.external_id,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_attribute_type() -> UpdateAttributeTypeStmt {
    UpdateAttributeTypeStmt(crate::client::async_::Stmt::new(
        "UPDATE
attributetype
SET
    name = coalesce($1, name),
    slug = coalesce($2, slug),
    external_id = coalesce($3, external_id)
WHERE
    id = $4",
    ))
}
pub struct UpdateAttributeTypeStmt(crate::client::async_::Stmt);
impl UpdateAttributeTypeStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        name: &'a Option<T1>,
        slug: &'a Option<T2>,
        external_id: &'a Option<T3>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[name, slug, external_id, id]).await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::JsonSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        UpdateAttributeTypeParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAttributeTypeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateAttributeTypeParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.name,
            &params.slug,
            &params.external_id,
            &params.id,
        ))
    }
}
pub fn delete_attribute_type() -> DeleteAttributeTypeStmt {
    DeleteAttributeTypeStmt(crate::client::async_::Stmt::new(
        "DELETE FROM attributetype
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeleteAttributeTypeStmt(crate::client::async_::Stmt);
impl DeleteAttributeTypeStmt {
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
        DeleteAttributeTypeParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteAttributeTypeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteAttributeTypeParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
