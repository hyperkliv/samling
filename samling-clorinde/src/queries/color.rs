// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetColorParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetColorIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetColorRefsParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertColorParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::JsonSql,
> {
    pub style_id: i32,
    pub slug: T1,
    pub external_id: Option<T2>,
    pub number: T3,
    pub name: T4,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateColorParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::JsonSql,
> {
    pub style_id: i32,
    pub slug: Option<T1>,
    pub external_id: Option<T2>,
    pub number: Option<T3>,
    pub name: Option<T4>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteColorParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ColorRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub style_id: i32,
    pub number: String,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub style: serde_json::Value,
    pub images: serde_json::Value,
}
pub struct ColorRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub style_id: i32,
    pub number: &'a str,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub style: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub images: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<ColorRowBorrowed<'a>> for ColorRow {
    fn from(
        ColorRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            style_id,
            number,
            name,
            created_by,
            created_at,
            updated_at,
            style,
            images,
        }: ColorRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            style_id,
            number: number.into(),
            name: serde_json::from_str(name.0.get()).unwrap(),
            created_by,
            created_at,
            updated_at,
            style: serde_json::from_str(style.0.get()).unwrap(),
            images: serde_json::from_str(images.0.get()).unwrap(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ColorRefs {
    pub id: i32,
    pub external_id: Option<String>,
    pub slug: String,
}
pub struct ColorRefsBorrowed<'a> {
    pub id: i32,
    pub external_id: Option<&'a str>,
    pub slug: &'a str,
}
impl<'a> From<ColorRefsBorrowed<'a>> for ColorRefs {
    fn from(
        ColorRefsBorrowed {
            id,
            external_id,
            slug,
        }: ColorRefsBorrowed<'a>,
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
pub struct ColorRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> ColorRowBorrowed,
    mapper: fn(ColorRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> ColorRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ColorRowBorrowed) -> R) -> ColorRowQuery<'a, C, R, N> {
        ColorRowQuery {
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
pub struct ColorRefsQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> ColorRefsBorrowed,
    mapper: fn(ColorRefsBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> ColorRefsQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ColorRefsBorrowed) -> R) -> ColorRefsQuery<'a, C, R, N> {
        ColorRefsQuery {
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
pub fn list_colors() -> ListColorsStmt {
    ListColorsStmt(crate::client::async_::Stmt::new(
        "SELECT
    color.*,
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
    ) AS \"style\",
    coalesce(
        jsonb_agg(
            jsonb_build_object(
                'id',
                image.id,
                'external_id',
                image.external_id,
                'url',
                image.url
            )
            ORDER BY image.position ASC, image.uploaded_at DESC
        ) FILTER (WHERE image.id IS NOT NULL), '[]'::jsonb) AS \"images\"
FROM
    color
INNER JOIN style ON style.id = color.style_id
LEFT OUTER JOIN image ON image.color_id = color.id
WHERE
    color.organization_id = $1
GROUP BY
    style.id,
    color.id
ORDER BY
    style.number,
    color.number",
    ))
}
pub struct ListColorsStmt(crate::client::async_::Stmt);
impl ListColorsStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> ColorRowQuery<'a, C, ColorRow, 1> {
        ColorRowQuery {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| ColorRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                style_id: row.get(4),
                number: row.get(5),
                name: row.get(6),
                created_by: row.get(7),
                created_at: row.get(8),
                updated_at: row.get(9),
                style: row.get(10),
                images: row.get(11),
            },
            mapper: |it| <ColorRow>::from(it),
        }
    }
}
pub fn get_color() -> GetColorStmt {
    GetColorStmt(crate::client::async_::Stmt::new(
        "SELECT
    color.*,
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
    ) AS \"style\",
    coalesce(
        jsonb_agg(
            jsonb_build_object(
                'id',
                image.id,
                'external_id',
                image.external_id,
                'url',
                image.url
            )
            ORDER BY image.position ASC, image.uploaded_at DESC
        ) FILTER (WHERE image.id IS NOT NULL), '[]'::jsonb) AS \"images\"
FROM
    color
INNER JOIN style ON style.id = color.style_id
LEFT OUTER JOIN image ON image.color_id = color.id
WHERE
    color.organization_id = $1
    AND (
        color.id = coalesce($2, -1)
        OR color.external_id = coalesce($3, '___NON_EXISTING___')
        OR color.slug = coalesce($4, '___NON_EXISTING___')
    )
GROUP BY
    style.id,
    color.id",
    ))
}
pub struct GetColorStmt(crate::client::async_::Stmt);
impl GetColorStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> ColorRowQuery<'a, C, ColorRow, 4> {
        ColorRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| ColorRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                style_id: row.get(4),
                number: row.get(5),
                name: row.get(6),
                created_by: row.get(7),
                created_at: row.get(8),
                updated_at: row.get(9),
                style: row.get(10),
                images: row.get(11),
            },
            mapper: |it| <ColorRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<'a, GetColorParams<T1, T2>, ColorRowQuery<'a, C, ColorRow, 4>, C>
    for GetColorStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetColorParams<T1, T2>,
    ) -> ColorRowQuery<'a, C, ColorRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn get_color_id() -> GetColorIdStmt {
    GetColorIdStmt(crate::client::async_::Stmt::new(
        "SELECT color.id
FROM
    color
WHERE
    color.organization_id = $1
    AND (
        color.id = coalesce($2, -1)
        OR color.external_id = coalesce($3, '___NON_EXISTING___')
        OR color.slug = coalesce($4, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetColorIdStmt(crate::client::async_::Stmt);
impl GetColorIdStmt {
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
    crate::client::async_::Params<'a, GetColorIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetColorIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetColorIdParams<T1, T2>,
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
pub fn get_color_refs() -> GetColorRefsStmt {
    GetColorRefsStmt(crate::client::async_::Stmt::new(
        "SELECT
    color.id,
    color.external_id,
    color.slug
FROM
    color
WHERE
    color.organization_id = $1
    AND (
        color.id = coalesce($2, -1)
        OR color.external_id = coalesce($3, '___NON_EXISTING___')
        OR color.slug = coalesce($4, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetColorRefsStmt(crate::client::async_::Stmt);
impl GetColorRefsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> ColorRefsQuery<'a, C, ColorRefs, 4> {
        ColorRefsQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| ColorRefsBorrowed {
                id: row.get(0),
                external_id: row.get(1),
                slug: row.get(2),
            },
            mapper: |it| <ColorRefs>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        GetColorRefsParams<T1, T2>,
        ColorRefsQuery<'a, C, ColorRefs, 4>,
        C,
    > for GetColorRefsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetColorRefsParams<T1, T2>,
    ) -> ColorRefsQuery<'a, C, ColorRefs, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn insert_color() -> InsertColorStmt {
    InsertColorStmt(crate::client::async_::Stmt::new(
        "INSERT INTO color (
    style_id,
    slug,
    external_id,
    number,
    name,
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
pub struct InsertColorStmt(crate::client::async_::Stmt);
impl InsertColorStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
    >(
        &'a mut self,
        client: &'a C,
        style_id: &'a i32,
        slug: &'a T1,
        external_id: &'a Option<T2>,
        number: &'a T3,
        name: &'a T4,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 7> {
        I32Query {
            client,
            params: [
                style_id,
                slug,
                external_id,
                number,
                name,
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
    >
    crate::client::async_::Params<'a, InsertColorParams<T1, T2, T3, T4>, I32Query<'a, C, i32, 7>, C>
    for InsertColorStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertColorParams<T1, T2, T3, T4>,
    ) -> I32Query<'a, C, i32, 7> {
        self.bind(
            client,
            &params.style_id,
            &params.slug,
            &params.external_id,
            &params.number,
            &params.name,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_color() -> UpdateColorStmt {
    UpdateColorStmt(crate::client::async_::Stmt::new(
        "UPDATE
color
SET
    style_id = coalesce($1, style_id),
    slug = coalesce($2, slug),
    external_id = coalesce($3, external_id),
    number = coalesce($4, number),
    name = coalesce($5, name)
WHERE
    id = $6",
    ))
}
pub struct UpdateColorStmt(crate::client::async_::Stmt);
impl UpdateColorStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::JsonSql,
    >(
        &'a mut self,
        client: &'a C,
        style_id: &'a i32,
        slug: &'a Option<T1>,
        external_id: &'a Option<T2>,
        number: &'a Option<T3>,
        name: &'a Option<T4>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[style_id, slug, external_id, number, name, id])
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
    >
    crate::client::async_::Params<
        'a,
        UpdateColorParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateColorStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateColorParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.style_id,
            &params.slug,
            &params.external_id,
            &params.number,
            &params.name,
            &params.id,
        ))
    }
}
pub fn delete_color() -> DeleteColorStmt {
    DeleteColorStmt(crate::client::async_::Stmt::new(
        "DELETE FROM color
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeleteColorStmt(crate::client::async_::Stmt);
impl DeleteColorStmt {
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
        DeleteColorParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteColorStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteColorParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
