// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetAttributeParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetAttributeIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertAttributeParams<
    T1: crate::JsonSql,
    T2: crate::JsonSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub title: T1,
    pub description: T2,
    pub type_id: i32,
    pub slug: T3,
    pub external_id: Option<T4>,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateAttributeParams<
    T1: crate::JsonSql,
    T2: crate::JsonSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub type_id: Option<i32>,
    pub title: Option<T1>,
    pub description: Option<T2>,
    pub slug: Option<T3>,
    pub external_id: Option<T4>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteAttributeParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug)]
pub struct AssociateStyleAttributesParams<T1: crate::ArraySql<Item = i32>> {
    pub style_id: i32,
    pub attribute_ids: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeRow {
    pub id: i32,
    pub organization_id: i32,
    pub type_id: i32,
    pub title: serde_json::Value,
    pub description: serde_json::Value,
    pub slug: String,
    pub external_id: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub r#type: serde_json::Value,
}
pub struct AttributeRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub type_id: i32,
    pub title: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub description: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub r#type: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<AttributeRowBorrowed<'a>> for AttributeRow {
    fn from(
        AttributeRowBorrowed {
            id,
            organization_id,
            type_id,
            title,
            description,
            slug,
            external_id,
            created_by,
            created_at,
            updated_at,
            r#type,
        }: AttributeRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            type_id,
            title: serde_json::from_str(title.0.get()).unwrap(),
            description: serde_json::from_str(description.0.get()).unwrap(),
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            created_by,
            created_at,
            updated_at,
            r#type: serde_json::from_str(r#type.0.get()).unwrap(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct AttributeRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> AttributeRowBorrowed,
    mapper: fn(AttributeRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> AttributeRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AttributeRowBorrowed) -> R) -> AttributeRowQuery<'a, C, R, N> {
        AttributeRowQuery {
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
pub fn list_attributes() -> ListAttributesStmt {
    ListAttributesStmt(crate::client::async_::Stmt::new(
        "SELECT
    attribute.*,
    to_jsonb(attributetype.*) AS \"type\"
FROM
    attribute
INNER JOIN attributetype
    ON attributetype.id = attribute.type_id
WHERE
    attribute.organization_id = $1
ORDER BY
    attribute.updated_at DESC",
    ))
}
pub struct ListAttributesStmt(crate::client::async_::Stmt);
impl ListAttributesStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> AttributeRowQuery<'a, C, AttributeRow, 1> {
        AttributeRowQuery {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| AttributeRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                type_id: row.get(2),
                title: row.get(3),
                description: row.get(4),
                slug: row.get(5),
                external_id: row.get(6),
                created_by: row.get(7),
                created_at: row.get(8),
                updated_at: row.get(9),
                r#type: row.get(10),
            },
            mapper: |it| <AttributeRow>::from(it),
        }
    }
}
pub fn get_attribute() -> GetAttributeStmt {
    GetAttributeStmt(crate::client::async_::Stmt::new(
        "SELECT
    attribute.*,
    to_jsonb(attributetype.*) AS \"type\"
FROM
    attribute
INNER JOIN attributetype
    ON attributetype.id = attribute.type_id
WHERE
    attribute.organization_id = $1
    AND ((attribute.id = coalesce($2, -1))
        OR (
            attribute.external_id = coalesce($3, '___NON_EXISTING___')
        )
        OR (attribute.slug = coalesce($4, '___NON_EXISTING___')))",
    ))
}
pub struct GetAttributeStmt(crate::client::async_::Stmt);
impl GetAttributeStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> AttributeRowQuery<'a, C, AttributeRow, 4> {
        AttributeRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| AttributeRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                type_id: row.get(2),
                title: row.get(3),
                description: row.get(4),
                slug: row.get(5),
                external_id: row.get(6),
                created_by: row.get(7),
                created_at: row.get(8),
                updated_at: row.get(9),
                r#type: row.get(10),
            },
            mapper: |it| <AttributeRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        GetAttributeParams<T1, T2>,
        AttributeRowQuery<'a, C, AttributeRow, 4>,
        C,
    > for GetAttributeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetAttributeParams<T1, T2>,
    ) -> AttributeRowQuery<'a, C, AttributeRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn get_attribute_id() -> GetAttributeIdStmt {
    GetAttributeIdStmt(crate::client::async_::Stmt::new(
        "SELECT attribute.id
FROM
    attribute
WHERE
    attribute.organization_id = $1
    AND ((attribute.id = coalesce($2, -1))
        OR (
            attribute.external_id = coalesce($3, '___NON_EXISTING___')
        )
        OR (attribute.slug = coalesce($4, '___NON_EXISTING___')))",
    ))
}
pub struct GetAttributeIdStmt(crate::client::async_::Stmt);
impl GetAttributeIdStmt {
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
    crate::client::async_::Params<'a, GetAttributeIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetAttributeIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetAttributeIdParams<T1, T2>,
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
pub fn insert_attribute() -> InsertAttributeStmt {
    InsertAttributeStmt(crate::client::async_::Stmt::new(
        "INSERT INTO attribute (
    title,
    description,
    type_id,
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
pub struct InsertAttributeStmt(crate::client::async_::Stmt);
impl InsertAttributeStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::JsonSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        title: &'a T1,
        description: &'a T2,
        type_id: &'a i32,
        slug: &'a T3,
        external_id: &'a Option<T4>,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 7> {
        I32Query {
            client,
            params: [
                title,
                description,
                type_id,
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
    >
    crate::client::async_::Params<
        'a,
        InsertAttributeParams<T1, T2, T3, T4>,
        I32Query<'a, C, i32, 7>,
        C,
    > for InsertAttributeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertAttributeParams<T1, T2, T3, T4>,
    ) -> I32Query<'a, C, i32, 7> {
        self.bind(
            client,
            &params.title,
            &params.description,
            &params.type_id,
            &params.slug,
            &params.external_id,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_attribute() -> UpdateAttributeStmt {
    UpdateAttributeStmt(crate::client::async_::Stmt::new(
        "UPDATE
attribute
SET
    type_id = coalesce($1, type_id),
    title = coalesce($2, title),
    description = coalesce($3, description),
    slug = coalesce($4, slug),
    external_id = coalesce($5, external_id)
WHERE
    id = $6",
    ))
}
pub struct UpdateAttributeStmt(crate::client::async_::Stmt);
impl UpdateAttributeStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::JsonSql,
        T2: crate::JsonSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        type_id: &'a Option<i32>,
        title: &'a Option<T1>,
        description: &'a Option<T2>,
        slug: &'a Option<T3>,
        external_id: &'a Option<T4>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[type_id, title, description, slug, external_id, id])
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
    >
    crate::client::async_::Params<
        'a,
        UpdateAttributeParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateAttributeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateAttributeParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.type_id,
            &params.title,
            &params.description,
            &params.slug,
            &params.external_id,
            &params.id,
        ))
    }
}
pub fn delete_attribute() -> DeleteAttributeStmt {
    DeleteAttributeStmt(crate::client::async_::Stmt::new(
        "DELETE FROM attribute
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeleteAttributeStmt(crate::client::async_::Stmt);
impl DeleteAttributeStmt {
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
        DeleteAttributeParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteAttributeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteAttributeParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
pub fn associate_style_attributes() -> AssociateStyleAttributesStmt {
    AssociateStyleAttributesStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    associate_style_attributes($1, $2)",
    ))
}
pub struct AssociateStyleAttributesStmt(crate::client::async_::Stmt);
impl AssociateStyleAttributesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        style_id: &'a i32,
        attribute_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [style_id, attribute_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        AssociateStyleAttributesParams<T1>,
        I32Query<'a, C, i32, 2>,
        C,
    > for AssociateStyleAttributesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a AssociateStyleAttributesParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.style_id, &params.attribute_ids)
    }
}
