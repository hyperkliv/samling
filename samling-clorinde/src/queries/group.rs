// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct SelectGroupsParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct SelectGroupSummariesParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct GetGroupIdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub organization_id: i32,
    pub id: Option<i32>,
    pub external_id: Option<T1>,
    pub slug: Option<T2>,
}
#[derive(Debug)]
pub struct InsertGroupParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub slug: T1,
    pub external_id: Option<T2>,
    pub name: T3,
    pub description: T4,
    pub organization_id: i32,
    pub created_by: i32,
}
#[derive(Debug)]
pub struct UpdateGroupParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub slug: Option<T1>,
    pub external_id: Option<T2>,
    pub name: Option<T3>,
    pub description: Option<T4>,
    pub id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteGroupParams {
    pub organization_id: i32,
    pub id: i32,
}
#[derive(Debug)]
pub struct ReplaceGroupUsersParams<T1: crate::ArraySql<Item = i32>> {
    pub group_id: i32,
    pub user_ids: T1,
}
#[derive(Debug)]
pub struct ReplaceGroupCollectionsParams<T1: crate::ArraySql<Item = i32>> {
    pub group_id: i32,
    pub collection_ids: T1,
}
#[derive(Debug)]
pub struct ReplaceGroupPricelistsParams<T1: crate::ArraySql<Item = i32>> {
    pub group_id: i32,
    pub pricelist_ids: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GroupRow {
    pub id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub organization_id: i32,
    pub name: String,
    pub description: String,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub users: serde_json::Value,
    pub collections: serde_json::Value,
    pub price_lists: serde_json::Value,
}
pub struct GroupRowBorrowed<'a> {
    pub id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub organization_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub created_by: Option<i32>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub users: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub collections: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub price_lists: postgres_types::Json<&'a serde_json::value::RawValue>,
}
impl<'a> From<GroupRowBorrowed<'a>> for GroupRow {
    fn from(
        GroupRowBorrowed {
            id,
            slug,
            external_id,
            organization_id,
            name,
            description,
            created_by,
            created_at,
            updated_at,
            users,
            collections,
            price_lists,
        }: GroupRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            organization_id,
            name: name.into(),
            description: description.into(),
            created_by,
            created_at,
            updated_at,
            users: serde_json::from_str(users.0.get()).unwrap(),
            collections: serde_json::from_str(collections.0.get()).unwrap(),
            price_lists: serde_json::from_str(price_lists.0.get()).unwrap(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GroupSummaryRow {
    pub id: i32,
    pub slug: String,
    pub external_id: Option<String>,
    pub name: String,
    pub description: String,
    pub num_users: i64,
    pub num_collections: i64,
    pub num_price_lists: i64,
}
pub struct GroupSummaryRowBorrowed<'a> {
    pub id: i32,
    pub slug: &'a str,
    pub external_id: Option<&'a str>,
    pub name: &'a str,
    pub description: &'a str,
    pub num_users: i64,
    pub num_collections: i64,
    pub num_price_lists: i64,
}
impl<'a> From<GroupSummaryRowBorrowed<'a>> for GroupSummaryRow {
    fn from(
        GroupSummaryRowBorrowed {
            id,
            slug,
            external_id,
            name,
            description,
            num_users,
            num_collections,
            num_price_lists,
        }: GroupSummaryRowBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            slug: slug.into(),
            external_id: external_id.map(|v| v.into()),
            name: name.into(),
            description: description.into(),
            num_users,
            num_collections,
            num_price_lists,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GroupRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> GroupRowBorrowed,
    mapper: fn(GroupRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> GroupRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GroupRowBorrowed) -> R) -> GroupRowQuery<'a, C, R, N> {
        GroupRowQuery {
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
pub struct GroupSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
    client: &'a C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'a mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> GroupSummaryRowBorrowed,
    mapper: fn(GroupSummaryRowBorrowed) -> T,
}
impl<'a, C, T: 'a, const N: usize> GroupSummaryRowQuery<'a, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GroupSummaryRowBorrowed) -> R,
    ) -> GroupSummaryRowQuery<'a, C, R, N> {
        GroupSummaryRowQuery {
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
pub fn select_groups() -> SelectGroupsStmt {
    SelectGroupsStmt(crate::client::async_::Stmt::new(
        "SELECT
    \"group\".*,
    coalesce(\"users\".json_data, '[]'::json) AS \"users\",
    coalesce(collections.json_data, '[]'::json) AS collections,
    coalesce(pricelists.json_data, '[]'::json) AS price_lists
FROM \"group\"
LEFT JOIN LATERAL (
    SELECT
        group_user.group_id,
        json_agg(json_build_object(
            'id',
            \"user\".id,
            'name',
            \"user\".name,
            'email',
            \"user\".email,
            'last_sign_in',
            \"user\".last_sign_in,
            'profile_image',
            \"user\".profile_image
        )) FILTER (WHERE \"user\".id IS NOT NULL) AS json_data
    FROM group_user
    INNER JOIN \"user\" ON \"user\".id = group_user.user_id
    WHERE group_user.group_id = \"group\".id
    GROUP BY group_user.group_id
) AS \"users\" ON \"users\".group_id = \"group\".id
LEFT JOIN LATERAL (
    SELECT
        group_collection.group_id,
        json_agg(
            json_build_object(
                'id',
                collection.id,
                'acronym',
                collection.acronym,
                'name',
                collection.name,
                'image_url',
                collection.image_url,
                'external_id',
                collection.external_id,
                'slug',
                collection.slug,
                'created_by',
                collection.created_by,
                'created_at',
                collection.created_at,
                'updated_at',
                collection.updated_at,
                'pricing',
                coalesce(pricing.json_data, '[]'::json),
                'num_styles',
                coalesce(stats.num_styles, 0),
                'num_colors',
                coalesce(stats.num_colors, 0),
                'num_sizes',
                coalesce(stats.num_sizes, 0)
            )
            ORDER BY collection.id DESC
        ) FILTER (WHERE collection.id IS NOT NULL) AS json_data
    FROM group_collection
    INNER JOIN collection ON collection.id = group_collection.collection_id
    LEFT JOIN (
        SELECT
            size_collection.collection_id,
            count(DISTINCT size.id) AS num_sizes,
            count(DISTINCT color.id) AS num_colors,
            count(DISTINCT color.style_id) AS num_styles
        FROM size_collection
        INNER JOIN size ON size.id = size_collection.size_id
        INNER JOIN color ON size.color_id = color.id
        GROUP BY size_collection.collection_id
    ) AS stats ON stats.collection_id = group_collection.collection_id
    LEFT JOIN LATERAL (
        SELECT
            collection_pricelist.collection_id,
            json_agg(
                json_build_object(
                    'list',
                    json_build_object(
                        'id',
                        pricelist.id,
                        'external_id',
                        pricelist.external_id,
                        'slug',
                        pricelist.slug,
                        'name',
                        pricelist.name
                    ),
                    'date',
                    collection_pricelist.price_date
                )
            ) FILTER (WHERE collection_pricelist.collection_id IS NOT NULL) AS json_data
        FROM collection_pricelist
        INNER JOIN pricelist
            ON pricelist.id = collection_pricelist.pricelist_id
        WHERE collection_pricelist.collection_id = collection.id
        GROUP BY collection_pricelist.collection_id
    ) AS pricing ON pricing.collection_id = collection.id
    WHERE group_collection.group_id = \"group\".id
    GROUP BY group_collection.group_id
) AS collections ON collections.group_id = \"group\".id
LEFT JOIN LATERAL (
    SELECT
        group_pricelist.group_id,
        json_agg(
            json_build_object(
                'id',
                pricelist.id,
                'name',
                pricelist.name,
                'slug',
                pricelist.slug,
                'external_id',
                pricelist.external_id
            )
        ) FILTER (WHERE pricelist.id IS NOT NULL) AS json_data
    FROM group_pricelist
    INNER JOIN pricelist ON pricelist.id = group_pricelist.pricelist_id
    WHERE group_pricelist.group_id = \"group\".id
    GROUP BY group_pricelist.group_id
) AS pricelists ON pricelists.group_id = \"group\".id
WHERE
    \"group\".organization_id = $1
    AND ($2::int IS NULL OR \"group\".id = $2)
    AND ($3::text IS NULL OR \"group\".external_id = $3)
    AND ($4::text IS NULL OR \"group\".slug = $4)
ORDER BY
    \"group\".updated_at DESC",
    ))
}
pub struct SelectGroupsStmt(crate::client::async_::Stmt);
impl SelectGroupsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> GroupRowQuery<'a, C, GroupRow, 4> {
        GroupRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| GroupRowBorrowed {
                id: row.get(0),
                slug: row.get(1),
                external_id: row.get(2),
                organization_id: row.get(3),
                name: row.get(4),
                description: row.get(5),
                created_by: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                users: row.get(9),
                collections: row.get(10),
                price_lists: row.get(11),
            },
            mapper: |it| <GroupRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        SelectGroupsParams<T1, T2>,
        GroupRowQuery<'a, C, GroupRow, 4>,
        C,
    > for SelectGroupsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectGroupsParams<T1, T2>,
    ) -> GroupRowQuery<'a, C, GroupRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn select_group_summaries() -> SelectGroupSummariesStmt {
    SelectGroupSummariesStmt(crate::client::async_::Stmt::new(
        "SELECT
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
    FROM group_user WHERE group_user.group_id = \"group\".id GROUP BY group_user.group_id
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
    \"group\".organization_id = $1
    AND ($2::int IS NULL OR \"group\".id = $2)
    AND ($3::text IS NULL OR \"group\".external_id = $3)
    AND ($4::text IS NULL OR \"group\".slug = $4)
ORDER BY
    \"group\".updated_at DESC",
    ))
}
pub struct SelectGroupSummariesStmt(crate::client::async_::Stmt);
impl SelectGroupSummariesStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
        id: &'a Option<i32>,
        external_id: &'a Option<T1>,
        slug: &'a Option<T2>,
    ) -> GroupSummaryRowQuery<'a, C, GroupSummaryRow, 4> {
        GroupSummaryRowQuery {
            client,
            params: [organization_id, id, external_id, slug],
            stmt: &mut self.0,
            extractor: |row| GroupSummaryRowBorrowed {
                id: row.get(0),
                slug: row.get(1),
                external_id: row.get(2),
                name: row.get(3),
                description: row.get(4),
                num_users: row.get(5),
                num_collections: row.get(6),
                num_price_lists: row.get(7),
            },
            mapper: |it| <GroupSummaryRow>::from(it),
        }
    }
}
impl<'a, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        SelectGroupSummariesParams<T1, T2>,
        GroupSummaryRowQuery<'a, C, GroupSummaryRow, 4>,
        C,
    > for SelectGroupSummariesStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a SelectGroupSummariesParams<T1, T2>,
    ) -> GroupSummaryRowQuery<'a, C, GroupSummaryRow, 4> {
        self.bind(
            client,
            &params.organization_id,
            &params.id,
            &params.external_id,
            &params.slug,
        )
    }
}
pub fn get_group_id() -> GetGroupIdStmt {
    GetGroupIdStmt(crate::client::async_::Stmt::new(
        "SELECT \"group\".id
FROM
    \"group\"
WHERE
    \"group\".organization_id = $1
    AND ($2::int IS NULL OR \"group\".id = $2)
    AND ($3::text IS NULL OR \"group\".external_id = $3)
    AND ($4::text IS NULL OR \"group\".slug = $4)",
    ))
}
pub struct GetGroupIdStmt(crate::client::async_::Stmt);
impl GetGroupIdStmt {
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
    crate::client::async_::Params<'a, GetGroupIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
    for GetGroupIdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GetGroupIdParams<T1, T2>,
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
pub fn insert_group() -> InsertGroupStmt {
    InsertGroupStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"group\" (
    slug,
    external_id,
    name,
    description,
    organization_id,
    created_by)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6)
RETURNING
id",
    ))
}
pub struct InsertGroupStmt(crate::client::async_::Stmt);
impl InsertGroupStmt {
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
        slug: &'a T1,
        external_id: &'a Option<T2>,
        name: &'a T3,
        description: &'a T4,
        organization_id: &'a i32,
        created_by: &'a i32,
    ) -> I32Query<'a, C, i32, 6> {
        I32Query {
            client,
            params: [
                slug,
                external_id,
                name,
                description,
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
        T4: crate::StringSql,
    >
    crate::client::async_::Params<'a, InsertGroupParams<T1, T2, T3, T4>, I32Query<'a, C, i32, 6>, C>
    for InsertGroupStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertGroupParams<T1, T2, T3, T4>,
    ) -> I32Query<'a, C, i32, 6> {
        self.bind(
            client,
            &params.slug,
            &params.external_id,
            &params.name,
            &params.description,
            &params.organization_id,
            &params.created_by,
        )
    }
}
pub fn update_group() -> UpdateGroupStmt {
    UpdateGroupStmt(crate::client::async_::Stmt::new(
        "UPDATE \"group\"
SET
    slug = coalesce($1, slug),
    external_id = coalesce($2, external_id),
    name = coalesce($3, name),
    description = coalesce($4, description)
WHERE
    id = $5",
    ))
}
pub struct UpdateGroupStmt(crate::client::async_::Stmt);
impl UpdateGroupStmt {
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
        slug: &'a Option<T1>,
        external_id: &'a Option<T2>,
        name: &'a Option<T3>,
        description: &'a Option<T4>,
        id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[slug, external_id, name, description, id])
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
        UpdateGroupParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateGroupStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateGroupParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.slug,
            &params.external_id,
            &params.name,
            &params.description,
            &params.id,
        ))
    }
}
pub fn delete_group() -> DeleteGroupStmt {
    DeleteGroupStmt(crate::client::async_::Stmt::new(
        "DELETE FROM \"group\"
WHERE
    organization_id = $1
    AND id = $2",
    ))
}
pub struct DeleteGroupStmt(crate::client::async_::Stmt);
impl DeleteGroupStmt {
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
        DeleteGroupParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteGroupStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a DeleteGroupParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.organization_id, &params.id))
    }
}
pub fn replace_group_users() -> ReplaceGroupUsersStmt {
    ReplaceGroupUsersStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    replace_group_users($1, $2)",
    ))
}
pub struct ReplaceGroupUsersStmt(crate::client::async_::Stmt);
impl ReplaceGroupUsersStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        group_id: &'a i32,
        user_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [group_id, user_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, ReplaceGroupUsersParams<T1>, I32Query<'a, C, i32, 2>, C>
    for ReplaceGroupUsersStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a ReplaceGroupUsersParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.group_id, &params.user_ids)
    }
}
pub fn replace_group_collections() -> ReplaceGroupCollectionsStmt {
    ReplaceGroupCollectionsStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    replace_group_collections($1, $2)",
    ))
}
pub struct ReplaceGroupCollectionsStmt(crate::client::async_::Stmt);
impl ReplaceGroupCollectionsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        group_id: &'a i32,
        collection_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [group_id, collection_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, ReplaceGroupCollectionsParams<T1>, I32Query<'a, C, i32, 2>, C>
    for ReplaceGroupCollectionsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a ReplaceGroupCollectionsParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.group_id, &params.collection_ids)
    }
}
pub fn replace_group_pricelists() -> ReplaceGroupPricelistsStmt {
    ReplaceGroupPricelistsStmt(crate::client::async_::Stmt::new(
        "SELECT *
FROM
    replace_group_pricelists($1, $2)",
    ))
}
pub struct ReplaceGroupPricelistsStmt(crate::client::async_::Stmt);
impl ReplaceGroupPricelistsStmt {
    pub fn bind<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'a mut self,
        client: &'a C,
        group_id: &'a i32,
        pricelist_ids: &'a T1,
    ) -> I32Query<'a, C, i32, 2> {
        I32Query {
            client,
            params: [group_id, pricelist_ids],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
impl<'a, C: GenericClient, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<'a, ReplaceGroupPricelistsParams<T1>, I32Query<'a, C, i32, 2>, C>
    for ReplaceGroupPricelistsStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a ReplaceGroupPricelistsParams<T1>,
    ) -> I32Query<'a, C, i32, 2> {
        self.bind(client, &params.group_id, &params.pricelist_ids)
    }
}
pub fn ensure_superuser_access() -> EnsureSuperuserAccessStmt {
    EnsureSuperuserAccessStmt(crate::client::async_::Stmt::new(
        "SELECT ensure_superuser_access($1)",
    ))
}
pub struct EnsureSuperuserAccessStmt(crate::client::async_::Stmt);
impl EnsureSuperuserAccessStmt {
    pub fn bind<'a, C: GenericClient>(
        &'a mut self,
        client: &'a C,
        organization_id: &'a i32,
    ) -> I32Query<'a, C, i32, 1> {
        I32Query {
            client,
            params: [organization_id],
            stmt: &mut self.0,
            extractor: |row| row.get(0),
            mapper: |it| it,
        }
    }
}
