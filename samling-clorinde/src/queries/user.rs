// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct SelectUsersParams<
    T1: crate::StringSql,
    T2: crate::ArraySql<Item = i32>,
    T3: crate::ArraySql<Item = i32>,
> {
    pub organization_id: Option<i32>,
    pub id: Option<i32>,
    pub email: Option<T1>,
    pub roles: Option<T2>,
    pub groups: Option<T3>,
}
#[derive(Clone, Copy, Debug)]
pub struct GetOrgUserIdParams {
    pub organization_id: i32,
    pub user_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct GetRoleIdsParams {
    pub user_id: i32,
    pub organization_id: i32,
}
#[derive(Debug)]
pub struct InsertUserParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub name: T1,
    pub email: T2,
    pub password_hash: Option<T3>,
    pub profile_image: Option<T4>,
}
#[derive(Debug)]
pub struct UpdateUserParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub name: Option<T1>,
    pub email: Option<T2>,
    pub password_hash: Option<T3>,
    pub profile_image: Option<T4>,
    pub id: i32,
}
#[derive(Debug)]
pub struct UpsertUserOrganizationParams<T1: crate::ArraySql<Item = i32>> {
    pub user_id: i32,
    pub organization_id: i32,
    pub role_ids: Option<T1>,
}
#[derive(Debug)]
pub struct ReplaceUserGroupsParams<T1: crate::ArraySql<Item = i32>> {
    pub user_id: i32,
    pub group_ids: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserRow {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub created_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub profile_image: Option<String>,
    pub last_sign_in: Option<crate::types::time::TimestampTz>,
    pub organizations: serde_json::Value,
    pub groups: serde_json::Value,
}
pub struct UserRowBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: Option<&'a str>,
    pub created_at: crate::types::time::TimestampTz,
    pub updated_at: crate::types::time::TimestampTz,
    pub profile_image: Option<&'a str>,
    pub last_sign_in: Option<crate::types::time::TimestampTz>,
    pub organizations: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub groups: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<UserRowBorrowed<'a>> for UserRow {
    fn from(
        UserRowBorrowed {
            id,
            name,
            email,
            password_hash,
            created_at,
            updated_at,
            profile_image,
            last_sign_in,
            organizations,
            groups,
        }: UserRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            email: email.into(),
            password_hash: password_hash.map(|v| v.into()),
            created_at,
            updated_at,
            profile_image: profile_image.map(|v| v.into()),
            last_sign_in,
            organizations: serde_json::from_str(organizations.0.get()).unwrap(),
            groups: serde_json::from_str(groups.0.get()).unwrap(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UserRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> UserRowBorrowed,
    mapper: fn(UserRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> UserRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UserRowBorrowed) -> R) -> UserRowQuery<'a, C, R, N> {
        UserRowQuery {
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
pub struct Veci32Query<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> crate::ArrayIterator<'_, i32>,
    mapper: fn(crate::ArrayIterator<'_, i32>) -> T,
}
impl<'a, C, T: 'a, const N: usize> Veci32Query<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(crate::ArrayIterator<'_, i32>) -> R,
    ) -> Veci32Query<'a, C, R, N> {
        Veci32Query {
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
pub fn select_users() -> SelectUsersStmt {
    SelectUsersStmt(crate::client::async_::Stmt::new(
        "WITH group_summaries AS (
    SELECT
        \"group\".id,
        \"group\".slug,
        \"group\".external_id,
        \"group\".name,
        \"group\".description,
        coalesce(user_counts.n, 0) AS num_users,
        coalesce(collection_counts.n, 0) AS num_collections,
        coalesce(pricelist_counts.n, 0) AS num_price_lists
    FROM \"group\"
    LEFT JOIN LATERAL (
        SELECT
            group_user.group_id,
            count(*) AS n
        FROM group_user
        WHERE group_user.group_id = \"group\".id
        GROUP BY group_user.group_id
    ) AS user_counts ON user_counts.group_id = \"group\".id
    LEFT JOIN LATERAL (
        SELECT
            group_collection.group_id,
            count(*) AS n
        FROM
            group_collection
        WHERE group_collection.group_id = \"group\".id GROUP BY group_collection.group_id
    ) AS collection_counts ON collection_counts.group_id = \"group\".id
    LEFT JOIN LATERAL (
        SELECT
            group_pricelist.group_id,
            count(*) AS n
        FROM
            group_pricelist
        WHERE group_pricelist.group_id = \"group\".id GROUP BY group_pricelist.group_id
    ) AS pricelist_counts ON pricelist_counts.group_id = \"group\".id
    WHERE
        ($1::int IS NULL OR \"group\".organization_id = $1)
)

SELECT
    \"user\".*,
    coalesce(organizations.json_data, '[]'::jsonb) AS organizations,
    coalesce(\"groups\".json_data, '[]'::jsonb) AS \"groups\"
FROM
    \"user\"
LEFT JOIN (
    SELECT
        user_organization.user_id,
        array_agg(organization.id) AS ids,
        jsonb_agg(
            jsonb_build_object(
                'organization',
                jsonb_build_object(
                    'id',
                    organization.id,
                    'name',
                    organization.name,
                    'logo_url',
                    organization.logo_url
                ),
                'role_ids',
                user_organization.role_ids
            )
        ) AS json_data
    FROM user_organization
    INNER JOIN
        organization ON organization.id = user_organization.organization_id
    GROUP BY user_organization.user_id
) AS organizations ON organizations.user_id = \"user\".id
LEFT JOIN (
    SELECT
        group_user.user_id,
        array_agg(group_user.group_id) AS group_ids,
        jsonb_agg(group_summaries.*) AS json_data
    FROM group_user
    INNER JOIN group_summaries
        ON group_summaries.id = group_user.group_id
    GROUP BY group_user.user_id
) AS \"groups\" ON \"groups\".user_id = \"user\".id
WHERE
    ($1::int IS NULL OR $1 = any(organizations.ids))
    AND ($2::int IS NULL OR \"user\".id = $2)
    AND ($3::text IS NULL OR lower(\"user\".email) = lower($3))
    AND (
        $4::int[] IS NULL
        OR jsonb_path_query_first(
            organizations.json_data,
            '$[*] ? (@.organization.id == $organization_id).role_ids',
            jsonb_build_object('organization_id', $1)
        ) @> to_jsonb($4)
    )
    AND ($5::int[] IS NULL OR \"groups\".group_ids @> $5)",
    ))
}
pub struct SelectUsersStmt(crate::client::async_::Stmt);
impl SelectUsersStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = i32>,
        T3: crate::ArraySql<Item = i32>,
    >(
        &'a mut self,
        client: &'a C,
        organization_id: &'a Option<i32>,
        id: &'a Option<i32>,
        email: &'a Option<T1>,
        roles: &'a Option<T2>,
        groups: &'a Option<T3>,
    ) -> UserRowQuery<'a, C, UserRow, 5> {
        UserRowQuery {
            client,
            params: [organization_id, id, email, roles, groups],
            stmt: &mut self.0,
            extractor: |row| UserRowBorrowed {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password_hash: row.get(3),
                created_at: row.get(4),
                updated_at: row.get(5),
                profile_image: row.get(6),
                last_sign_in: row.get(7),
                organizations: row.get(8),
                groups: row.get(9),
            },
            mapper: |it| <UserRow>::from(it),
        }
    }
}
impl<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = i32>,
        T3: crate::ArraySql<Item = i32>,
    >
    crate::client::async_::Params<
        'a,
        SelectUsersParams<T1, T2, T3>,
        UserRowQuery<'a, C, UserRow, 5>,
        C,
    > for SelectUsersStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectUsersParams<T1, T2, T3>,
    ) -> UserRowQuery<'a, C, UserRow, 5> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.email,
            &params.roles,
            &params.groups,
        )
    }
}
pub fn get_org_user_id() -> GetOrgUserIdStmt {
    GetOrgUserIdStmt(crate::client::async_::Stmt::new(
        "SELECT \"user\".id
FROM
    \"user\"
INNER JOIN user_organization
    ON user_organization.user_id = \"user\".id
WHERE
    user_organization.organization_id = $1
    AND \"user\".id = $2",
    ))
}
pub struct GetOrgUserIdStmt(crate::client::async_::Stmt);
impl GetOrgUserIdStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        user_id: &'a i32,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [organization_id, user_id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient>
    crate::client::async_::Params<'a, GetOrgUserIdParams, I32Query<'a, C, i32, 2>, C>
    for GetOrgUserIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetOrgUserIdParams,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.organization_id, &params.user_id)
    }
}
pub fn get_role_ids() -> GetRoleIdsStmt {
    GetRoleIdsStmt(crate::client::async_::Stmt::new(
        "SELECT user_organization.role_ids
FROM
    user_organization
WHERE
    user_organization.user_id = $1
    AND user_organization.organization_id = $2",
    ))
}
pub struct GetRoleIdsStmt(crate::client::async_::Stmt);
impl GetRoleIdsStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        user_id: &'a i32,
        organization_id: &'a i32,
    ) -> Veci32Query<'a, C, Vec<i32>, 2> {
        Veci32Query {
            client,
            params: [user_id, organization_id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it.map(|v| v).collect(),
        }
    }
}
impl<'a, C: GenericClient>
    crate::client::async_::Params<'a, GetRoleIdsParams, Veci32Query<'a, C, Vec<i32>, 2>, C>
    for GetRoleIdsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetRoleIdsParams,
    ) -> Veci32Query<'a, C, Vec<i32>, 2> {
        self.bind(client, &params.user_id, &params.organization_id)
    }
}
pub fn insert_user() -> InsertUserStmt {
    InsertUserStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"user\" (
    name,
    email,
    password_hash,
    profile_image)
VALUES (
    $1,
    $2,
    $3,
    $4)
RETURNING
id",
    ))
}
pub struct InsertUserStmt(crate::client::async_::Stmt);
impl InsertUserStmt {
    pub fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        name: &'a T1,
        email: &'a T2,
        password_hash: &'a Option<T3>,
        profile_image: &'a Option<T4>,
    ) -> I32Query<'a, C, i32, 4> {
        I32Query {
            client,
            params: [name, email, password_hash, profile_image],
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
        T4: crate::StringSql,
    >
    crate::client::async_::Params<'a, InsertUserParams<T1, T2, T3, T4>, I32Query<'a, C, i32, 4>, C>
    for InsertUserStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertUserParams<T1, T2, T3, T4>,
    ) -> I32Query<'a, C, i32, 4> {
        self.bind(
            client,
            &params.name,
            &params.email,
            &params.password_hash,
            &params.profile_image,
        )
    }
}
pub fn update_user() -> UpdateUserStmt {
    UpdateUserStmt(crate::client::async_::Stmt::new(
        "UPDATE
\"user\"
SET
    name = coalesce($1, name),
    email = coalesce($2, email),
    password_hash = coalesce($3, password_hash),
    profile_image = coalesce($4, profile_image)
WHERE
    id = $5",
    ))
}
pub struct UpdateUserStmt(crate::client::async_::Stmt);
impl UpdateUserStmt {
    pub async fn bind<
        'a,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'a mut self,
        client: &'a C,
        name: &'a Option<T1>,
        email: &'a Option<T2>,
        password_hash: &'a Option<T3>,
        profile_image: &'a Option<T4>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[name, email, password_hash, profile_image, id])
            .await
    }
}
impl<
        'a,
        C: GenericClient + Send + Sync,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
    >
    crate::client::async_::Params<
        'a,
        UpdateUserParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateUserStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateUserParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.name,
            &params.email,
            &params.password_hash,
            &params.profile_image,
            &params.id,
        ))
    }
}
pub fn delete_user() -> DeleteUserStmt {
    DeleteUserStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"user\"
WHERE id = $1",
    ))
}
pub struct DeleteUserStmt(crate::client::async_::Stmt);
impl DeleteUserStmt {
    pub async fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn upsert_user_organization() -> UpsertUserOrganizationStmt {
    UpsertUserOrganizationStmt(crate::client::async_::Stmt::new(
        "INSERT INTO user_organization (
    user_id,
    organization_id,
    role_ids
)
VALUES (
    $1,
    $2,
    $3
)
ON CONFLICT ON CONSTRAINT user_organization_uq
DO UPDATE SET role_ids = coalesce(excluded.role_ids, user_organization.role_ids)",
    ))
}
pub struct UpsertUserOrganizationStmt(crate::client::async_::Stmt);
impl UpsertUserOrganizationStmt {
    pub async fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        user_id: &'a i32,
        organization_id: &'a i32,
        role_ids: &'a Option<T1>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[user_id, organization_id, role_ids])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        UpsertUserOrganizationParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpsertUserOrganizationStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpsertUserOrganizationParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.user_id,
            &params.organization_id,
            &params.role_ids,
        ))
    }
}
pub fn get_user_password_hash() -> GetUserPasswordHashStmt {
    GetUserPasswordHashStmt(crate::client::async_::Stmt::new(
        "SELECT \"user\".password_hash
FROM
    \"user\"
WHERE
    \"user\".id = $1",
    ))
}
pub struct GetUserPasswordHashStmt(crate::client::async_::Stmt);
impl GetUserPasswordHashStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        user_id: &'a i32,
    ) -> StringQuery<'a, C, String, 1> {
        StringQuery {
            client,
            params: [user_id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it.into(),
        }
    }
}
pub fn update_last_sign_in() -> UpdateLastSignInStmt {
    UpdateLastSignInStmt(crate::client::async_::Stmt::new(
        "UPDATE \"user\" SET last_sign_in = now() WHERE id = $1",
    ))
}
pub struct UpdateLastSignInStmt(crate::client::async_::Stmt);
impl UpdateLastSignInStmt {
    pub async fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[id]).await
    }
}
pub fn replace_user_groups() -> ReplaceUserGroupsStmt {
    ReplaceUserGroupsStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    replace_user_groups($1, $2)",
    ))
}
pub struct ReplaceUserGroupsStmt(crate::client::async_::Stmt);
impl ReplaceUserGroupsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        user_id: &'a i32,
        group_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [user_id, group_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, ReplaceUserGroupsParams<T1>, I32Query<'a, C, i32, 2>, C>
    for ReplaceUserGroupsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a ReplaceUserGroupsParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.user_id, &params.group_ids)
    }
}
