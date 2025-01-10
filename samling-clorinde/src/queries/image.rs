// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetImageIdParams<T1: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
}
#[derive(Debug)]
pub struct GetImageUrlByExternalChecksumParams<T1: crate::StringSql> {
    pub organization_id: i32,
    pub external_checksum: T1,
}
#[derive(Debug)]
pub struct GetImageParams<T1: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
}
#[derive(Debug)]
pub struct InsertImageParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub color_id: i32,
    pub external_id: Option<T1>,
    pub url: T2,
    pub organization_id: i32,
    pub uploaded_by: i32,
    pub external_checksum: Option<T3>,
    pub position: i32,
}
#[derive(Debug)]
pub struct UpdateImageParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub color_id: i32,
    pub external_id: Option<T1>,
    pub url: Option<T2>,
    pub external_checksum: Option<T3>,
    pub position: Option<i32>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteImageParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ImageRow {
    pub id: i32,
    pub organization_id: i32,
    pub url: String,
    pub external_id: Option<String>,
    pub external_checksum: Option<String>,
    pub position: i32,
    pub color_id: i32,
    pub uploaded_by: Option<i32>,
    pub uploaded_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub color: serde_json::Value,
}
pub struct ImageRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub url: &'a str,
    pub external_id: Option<&'a str>,
    pub external_checksum: Option<&'a str>,
    pub position: i32,
    pub color_id: i32,
    pub uploaded_by: Option<i32>,
    pub uploaded_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub color: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<ImageRowBorrowed<'a>> for ImageRow {
    fn from(
        ImageRowBorrowed {
            id,
            organization_id,
            url,
            external_id,
            external_checksum,
            position,
            color_id,
            uploaded_by,
            uploaded_at,
            updated_at,
            color,
        }: ImageRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            organization_id,
            url: url.into(),
            external_id: external_id.map(|v| v.into()),
            external_checksum: external_checksum.map(|v| v.into()),
            position,
            color_id,
            uploaded_by,
            uploaded_at,
            updated_at,
            color: serde_json::from_str(color.0.get()).unwrap(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct ImageRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> ImageRowBorrowed,
    mapper: fn(ImageRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> ImageRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ImageRowBorrowed) -> R) -> ImageRowQuery<'a, C, R, N> {
        ImageRowQuery {
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
pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> &str,
    mapper: fn(&str) -> T,
}
impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'a, C, R, N> {
        StringQuery {
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
pub fn list_images() -> ListImagesStmt {
    ListImagesStmt(crate::client::async_::Stmt::new(
        "SELECT
    image.*,
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
    image
INNER JOIN color ON image.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    image.organization_id = $1
ORDER BY
    image.id",
    ))
}
pub struct ListImagesStmt(crate::client::async_::Stmt);
impl ListImagesStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> ImageRowQuery<'a, C, ImageRow, 1> {
        ImageRowQuery {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| ImageRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                url: row.get(2),
                external_id: row.get(3),
                external_checksum: row.get(4),
                position: row.get(5),
                color_id: row.get(6),
                uploaded_by: row.get(7),
                uploaded_at: row.get(8),
                updated_at: row.get(9),
                color: row.get(10),
            },
            mapper: |it| <ImageRow>::from(it),
        }
    }
}
pub fn get_image_id() -> GetImageIdStmt {
    GetImageIdStmt(crate::client::async_::Stmt::new(
        "SELECT image.id
FROM
    image
WHERE
    image.organization_id = $1
    AND (
        image.id = coalesce($2, -1)
        OR image.external_id = coalesce($3, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetImageIdStmt(crate::client::async_::Stmt);
impl GetImageIdStmt {
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
    crate::client::async_::Params<'a, GetImageIdParams<T1>, I32Query<'a, C, i32, 3>, C>
    for GetImageIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetImageIdParams<T1>,
    ) -> I32Query<'a, C, i32, 3> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
        )
    }
}
pub fn get_image_url_by_external_checksum() -> GetImageUrlByExternalChecksumStmt {
    GetImageUrlByExternalChecksumStmt(crate::client::async_::Stmt::new(
        "SELECT image.url
FROM
    image
WHERE
    image.organization_id = $1
    AND image.external_checksum = $2",
    ))
}
pub struct GetImageUrlByExternalChecksumStmt(crate::client::async_::Stmt);
impl GetImageUrlByExternalChecksumStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        external_checksum: &'a T1,
    ) -> StringQuery<'a, C, String, 2> {
        StringQuery {
            client,
            params: [organization_id, external_checksum],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        GetImageUrlByExternalChecksumParams<T1>,
        StringQuery<'a, C, String, 2>,
        C,
    > for GetImageUrlByExternalChecksumStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetImageUrlByExternalChecksumParams<T1>,
    ) -> StringQuery<'a, C, String, 2> {
        self.bind(client, &params.organization_id, &params.external_checksum)
    }
}
pub fn get_image() -> GetImageStmt {
    GetImageStmt(crate::client::async_::Stmt::new(
        "SELECT
    image.*,
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
    image
INNER JOIN color ON image.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    image.organization_id = $1
    AND (
        image.id = coalesce($2, -1)
        OR image.external_id = coalesce($3, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetImageStmt(crate::client::async_::Stmt);
impl GetImageStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
    ) -> ImageRowQuery<'a, C, ImageRow, 3> {
        ImageRowQuery {
            client,
            params: [organization_id, id, external_id],
            stmt: &mut self.0,
            extractor: |row| ImageRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                url: row.get(2),
                external_id: row.get(3),
                external_checksum: row.get(4),
                position: row.get(5),
                color_id: row.get(6),
                uploaded_by: row.get(7),
                uploaded_at: row.get(8),
                updated_at: row.get(9),
                color: row.get(10),
            },
            mapper: |it| <ImageRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<'a, GetImageParams<T1>, ImageRowQuery<'a, C, ImageRow, 3>, C>
    for GetImageStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetImageParams<T1>,
    ) -> ImageRowQuery<'a, C, ImageRow, 3> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
        )
    }
}
pub fn insert_image() -> InsertImageStmt {
    InsertImageStmt(crate::client::async_::Stmt::new(
        "INSERT INTO image (
    color_id,
    external_id,
    url,
    organization_id,
    uploaded_by,
    external_checksum,
    position)
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
pub struct InsertImageStmt(crate::client::async_::Stmt);
impl InsertImageStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        color_id: &'a i32,
        external_id: &'a Option<T1>,
        url: &'a T2,
        organization_id: &'a i32,
        uploaded_by: &'a i32,
        external_checksum: &'a Option<T3>,
        position: &'a i32,
    ) -> I32Query<'a, C, i32, 7> {
        I32Query {
            client,
            params: [
                color_id,
                external_id,
                url,
                organization_id,
                uploaded_by,
                external_checksum,
                position,
            ],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<'a, InsertImageParams<T1, T2, T3>, I32Query<'a, C, i32, 7>, C>
    for InsertImageStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertImageParams<T1, T2, T3>,
    ) -> I32Query<'a, C, i32, 7> {
        self.bind(
            client,
            &params.color_id,
            &params.external_id,
            &params.url,
            &params.organization_id,
            &params.uploaded_by,
            &params.external_checksum,
            &params.position,
        )
    }
}
pub fn update_image() -> UpdateImageStmt {
    UpdateImageStmt(crate::client::async_::Stmt::new(
        "UPDATE
image
SET
    color_id = coalesce($1, color_id),
    external_id = coalesce($2, external_id),
    url = coalesce($3, url),
    external_checksum = coalesce($4, external_checksum),
    position = coalesce($5, position)
WHERE
    id = $6",
    ))
}
pub struct UpdateImageStmt(crate::client::async_::Stmt);
impl UpdateImageStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        color_id: &'a i32,
        external_id: &'a Option<T1>,
        url: &'a Option<T2>,
        external_checksum: &'a Option<T3>,
        position: &'a Option<i32>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[color_id, external_id, url, external_checksum, position, id],
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
        UpdateImageParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateImageStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateImageParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.color_id,
            &params.external_id,
            &params.url,
            &params.external_checksum,
            &params.position,
            &params.id,
        ))
    }
}
pub fn delete_image() -> DeleteImageStmt {
    DeleteImageStmt(crate::client::async_::Stmt::new(
        "DELETE FROM image
WHERE organization_id = $1
      AND id = $2",
    ))
}
pub struct DeleteImageStmt(crate::client::async_::Stmt);
impl DeleteImageStmt {
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
        DeleteImageParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteImageStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteImageParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
