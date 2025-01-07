// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GetCategoryIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetCategoryParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertCategoryParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::JsonSql> {
    pub slug: T1,
    pub external_id: Option<T2>,
    pub name: T3,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateCategoryParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::JsonSql> {
    pub slug: Option<T1>,
    pub external_id: Option<T2>,
    pub name: Option<T3>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteCategoryParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug)]
pub struct AssociateStyleCategoriesParams<T1: crate::ArraySql<Item = i32>> {
    pub style_id: i32,
    pub category_ids: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryRow {
    pub id: i32,
    pub organization_id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub name: serde_json::Value,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct CategoryRowBorrowed<'a> {
    pub id: i32,
    pub organization_id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<CategoryRowBorrowed<'a>> for CategoryRow {
    fn from(
        CategoryRowBorrowed {
            id,
            organization_id,
            slug,
            external_id,
            name,
            created_by,
            created_at,
            updated_at,
        }: CategoryRowBorrowed<'a>,
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
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CategoryRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> CategoryRowBorrowed,
    mapper: fn(CategoryRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> CategoryRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(CategoryRowBorrowed) -> R) -> CategoryRowQuery<'a, C, R, N> {
        CategoryRowQuery {
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
pub fn list_categories() -> ListCategoriesStmt {
    ListCategoriesStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    category
WHERE
    category.organization_id = $1",
    ))
}
pub struct ListCategoriesStmt(crate::client::async_::Stmt);
impl ListCategoriesStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> CategoryRowQuery<'a, C, CategoryRow, 1> {
        CategoryRowQuery {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| CategoryRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                name: row.get(4),
                created_by: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            },
            mapper: |it| <CategoryRow>::from(it),
        }
    }
}
pub fn get_category_id() -> GetCategoryIdStmt {
    GetCategoryIdStmt(crate::client::async_::Stmt::new(
        "SELECT category.id
FROM
    category
WHERE
    category.organization_id = $1
    AND (
        category.id = coalesce($2, -1)
        OR category.external_id = coalesce($3, '___NON_EXISTING___')
        OR category.slug = coalesce($4, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetCategoryIdStmt(crate::client::async_::Stmt);
impl GetCategoryIdStmt {
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
    crate::client::async_::Params<'a, GetCategoryIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetCategoryIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetCategoryIdParams<T1, T2>,
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
pub fn get_category() -> GetCategoryStmt {
    GetCategoryStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    category
WHERE
    category.organization_id = $1
    AND (
        category.id = coalesce($2, -1)
        OR category.external_id = coalesce($3, '___NON_EXISTING___')
        OR category.slug = coalesce($4, '___NON_EXISTING___')
    )",
    ))
}
pub struct GetCategoryStmt(crate::client::async_::Stmt);
impl GetCategoryStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> CategoryRowQuery<'a, C, CategoryRow, 4> {
        CategoryRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| CategoryRowBorrowed {
                id: row.get(0),
                organization_id: row.get(1),
                slug: row.get(2),
                external_id: row.get(3),
                name: row.get(4),
                created_by: row.get(5),
                created_at: row.get(6),
                updated_at: row.get(7),
            },
            mapper: |it| <CategoryRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        GetCategoryParams<T1, T2>,
        CategoryRowQuery<'a, C, CategoryRow, 4>,
        C,
    > for GetCategoryStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetCategoryParams<T1, T2>,
    ) -> CategoryRowQuery<'a, C, CategoryRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn insert_category() -> InsertCategoryStmt {
    InsertCategoryStmt(crate::client::async_::Stmt::new(
        "INSERT INTO category (
    slug,
    external_id,
    name,
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
pub struct InsertCategoryStmt(crate::client::async_::Stmt);
impl InsertCategoryStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::JsonSql,
    >(
        &'a mut self,
        client: &'a C,
        slug: &'a T1,
        external_id: &'a Option<T2>,
        name: &'a T3,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 5> {
        I32Query {
            client,
            params: [slug, external_id, name, organization_id, created_by],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::JsonSql>
    crate::client::async_::Params<'a, InsertCategoryParams<T1, T2, T3>, I32Query<'a, C, i32, 5>, C>
    for InsertCategoryStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertCategoryParams<T1, T2, T3>,
    ) -> I32Query<'a, C, i32, 5> {
        self.bind(
            client,
            &params.slug,
            &params.external_id,
            &params.name,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_category() -> UpdateCategoryStmt {
    UpdateCategoryStmt(crate::client::async_::Stmt::new(
        "UPDATE category
SET
    slug = coalesce($1, slug),
    external_id = coalesce($2, external_id),
    name = coalesce($3, name)
WHERE
    category.id = $4",
    ))
}
pub struct UpdateCategoryStmt(crate::client::async_::Stmt);
impl UpdateCategoryStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::JsonSql,
    >(
        &'a mut self,
        client: &'a C,
        slug: &'a Option<T1>,
        external_id: &'a Option<T2>,
        name: &'a Option<T3>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[slug, external_id, name, id]).await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::JsonSql,
    >
    crate::client::async_::Params<
        'a,
        UpdateCategoryParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateCategoryStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateCategoryParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.slug,
            &params.external_id,
            &params.name,
            &params.id,
        ))
    }
}
pub fn delete_category() -> DeleteCategoryStmt {
    DeleteCategoryStmt(crate::client::async_::Stmt::new(
        "DELETE FROM category
WHERE
    organization_id = $1
    AND id = $2",
    ))
}
pub struct DeleteCategoryStmt(crate::client::async_::Stmt);
impl DeleteCategoryStmt {
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
        DeleteCategoryParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteCategoryStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteCategoryParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
pub fn associate_style_categories() -> AssociateStyleCategoriesStmt {
    AssociateStyleCategoriesStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    associate_style_categories($1, $2)",
    ))
}
pub struct AssociateStyleCategoriesStmt(crate::client::async_::Stmt);
impl AssociateStyleCategoriesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        style_id: &'a i32,
        category_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [style_id, category_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        AssociateStyleCategoriesParams<T1>,
        I32Query<'a, C, i32, 2>,
        C,
    > for AssociateStyleCategoriesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a AssociateStyleCategoriesParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.style_id, &params.category_ids)
    }
}
