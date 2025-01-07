#[derive(Debug)]
pub struct InsertOrganizationParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub logo_url: Option<T2>,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateOrganizationParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: Option<T1>,
    pub logo_url: Option<T2>,
    pub id: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationRow {
    pub id: i32,
    pub name: String,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub logo_url: Option<String>,
}
pub struct OrganizationRowBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub logo_url: Option<&'a str>,
}
impl<'a> From<OrganizationRowBorrowed<'a>> for OrganizationRow {
    fn from(
        OrganizationRowBorrowed {
            id,
            name,
            created_by,
            created_at,
            updated_at,
            logo_url,
        }: OrganizationRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            created_by,
            created_at,
            updated_at,
            logo_url: logo_url.map(|v| v.into()),
        }
    }
} // This file was generated with `clorinde`. Do not modify.

use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct OrganizationRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> OrganizationRowBorrowed,
    mapper: fn(OrganizationRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> OrganizationRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(OrganizationRowBorrowed) -> R,
    ) -> OrganizationRowQuery<'a, C, R, N> {
        OrganizationRowQuery {
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
pub fn get_organization() -> GetOrganizationStmt {
    GetOrganizationStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    organization
WHERE
    organization.id = $1",
    ))
}
pub struct GetOrganizationStmt(crate::client::async_::Stmt);
impl GetOrganizationStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        id: &'a i32,
    ) -> OrganizationRowQuery<'a, C, OrganizationRow, 1> {
        OrganizationRowQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| OrganizationRowBorrowed {
                id: row.get(0),
                name: row.get(1),
                created_by: row.get(2),
                created_at: row.get(3),
                updated_at: row.get(4),
                logo_url: row.get(5),
            },
            mapper: |it| <OrganizationRow>::from(it),
        }
    }
}
pub fn get_organization_id() -> GetOrganizationIdStmt {
    GetOrganizationIdStmt(crate::client::async_::Stmt::new(
        "SELECT organization.id
FROM
    organization
WHERE
    organization.id = $1",
    ))
}
pub struct GetOrganizationIdStmt(crate::client::async_::Stmt);
impl GetOrganizationIdStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        id: &'a i32,
    ) -> I32Query<'a, C, i32, 1> {
        I32Query {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
pub fn insert_organization() -> InsertOrganizationStmt {
    InsertOrganizationStmt(crate::client::async_::Stmt::new(
        "INSERT INTO organization (
    name,
    logo_url,
    created_by)
VALUES (
    $1,
    $2,
    $3)
RETURNING
*",
    ))
}
pub struct InsertOrganizationStmt(crate::client::async_::Stmt);
impl InsertOrganizationStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        name: &'a T1,
        logo_url: &'a Option<T2>,
        created_by: &'a i32,
    ) -> OrganizationRowQuery<'a, C, OrganizationRow, 3> {
        OrganizationRowQuery {
            client,
            params: [name, logo_url, created_by],
            stmt: &mut self.0,
            extractor: |row| OrganizationRowBorrowed {
                id: row.get(0),
                name: row.get(1),
                created_by: row.get(2),
                created_at: row.get(3),
                updated_at: row.get(4),
                logo_url: row.get(5),
            },
            mapper: |it| <OrganizationRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        InsertOrganizationParams<T1, T2>,
        OrganizationRowQuery<'a, C, OrganizationRow, 3>,
        C,
    > for InsertOrganizationStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertOrganizationParams<T1, T2>,
    ) -> OrganizationRowQuery<'a, C, OrganizationRow, 3> {
        self.bind(client, &params.name, &params.logo_url, &params.created_by)
    }
}
pub fn update_organization() -> UpdateOrganizationStmt {
    UpdateOrganizationStmt(crate::client::async_::Stmt::new(
        "UPDATE
organization
SET
    name = coalesce($1, name),
    logo_url = coalesce($2, logo_url)
WHERE
    id = $3
RETURNING
*",
    ))
}
pub struct UpdateOrganizationStmt(crate::client::async_::Stmt);
impl UpdateOrganizationStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        name: &'a Option<T1>,
        logo_url: &'a Option<T2>,
        id: &'a i32,
    ) -> OrganizationRowQuery<'a, C, OrganizationRow, 3> {
        OrganizationRowQuery {
            client,
            params: [name, logo_url, id],
            stmt: &mut self.0,
            extractor: |row| OrganizationRowBorrowed {
                id: row.get(0),
                name: row.get(1),
                created_by: row.get(2),
                created_at: row.get(3),
                updated_at: row.get(4),
                logo_url: row.get(5),
            },
            mapper: |it| <OrganizationRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        UpdateOrganizationParams<T1, T2>,
        OrganizationRowQuery<'a, C, OrganizationRow, 3>,
        C,
    > for UpdateOrganizationStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateOrganizationParams<T1, T2>,
    ) -> OrganizationRowQuery<'a, C, OrganizationRow, 3> {
        self.bind(client, &params.name, &params.logo_url, &params.id)
    }
}
pub fn delete_organization() -> DeleteOrganizationStmt {
    DeleteOrganizationStmt(crate::client::async_::Stmt::new(
        "DELETE FROM organization
WHERE id = $1
RETURNING
id",
    ))
}
pub struct DeleteOrganizationStmt(crate::client::async_::Stmt);
impl DeleteOrganizationStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        id: &'a i32,
    ) -> I32Query<'a, C, i32, 1> {
        I32Query {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
pub fn list_user_organizations() -> ListUserOrganizationsStmt {
    ListUserOrganizationsStmt(crate::client::async_::Stmt::new(
        "SELECT organization.*
FROM
    organization
INNER JOIN
    user_organization ON organization.id = user_organization.organization_id
WHERE
    user_organization.user_id = $1",
    ))
}
pub struct ListUserOrganizationsStmt(crate::client::async_::Stmt);
impl ListUserOrganizationsStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        user_id: &'a i32,
    ) -> OrganizationRowQuery<'a, C, OrganizationRow, 1> {
        OrganizationRowQuery {
            client,
            params: [user_id],
            stmt: &mut self.0,
            extractor: |row| OrganizationRowBorrowed {
                id: row.get(0),
                name: row.get(1),
                created_by: row.get(2),
                created_at: row.get(3),
                updated_at: row.get(4),
                logo_url: row.get(5),
            },
            mapper: |it| <OrganizationRow>::from(it),
        }
    }
}
