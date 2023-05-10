// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(Debug, postgres_types :: FromSql, Copy, Clone, PartialEq)]
        #[postgres(name = "collection_pricelist_relation")]
        pub struct CollectionPricelistRelation {
            #[postgres(name = "pricelist_id")]
            pub pricelist_id: i32,
            #[postgres(name = "price_date")]
            pub price_date: time::Date,
            #[postgres(name = "created_by")]
            pub created_by: i32,
        }
        impl<'a> postgres_types::ToSql for CollectionPricelistRelation {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CollectionPricelistRelation {
                    pricelist_id,
                    price_date,
                    created_by,
                } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "pricelist_id" => {
                            postgres_types::ToSql::to_sql(pricelist_id, field.type_(), out)
                        }
                        "price_date" => {
                            postgres_types::ToSql::to_sql(price_date, field.type_(), out)
                        }
                        "created_by" => {
                            postgres_types::ToSql::to_sql(created_by, field.type_(), out)
                        }
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "collection_pricelist_relation" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "pricelist_id" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            "price_date" => {
                                <time::Date as postgres_types::ToSql>::accepts(f.type_())
                            }
                            "created_by" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum Pricetype {
            Unit,
            Retail,
        }
        impl<'a> postgres_types::ToSql for Pricetype {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    Pricetype::Unit => "Unit",
                    Pricetype::Retail => "Retail",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "pricetype" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 2 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "Unit" => true,
                            "Retail" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for Pricetype {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<Pricetype, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "Unit" => Ok(Pricetype::Unit),
                    "Retail" => Ok(Pricetype::Retail),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "pricetype" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 2 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "Unit" => true,
                            "Retail" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod admin {
        #[derive(Debug, Clone, PartialEq)]
        pub struct EntityFilterChoiceRow {
            pub id: i32,
            pub title: serde_json::Value,
            pub subtitle: Option<serde_json::Value>,
            pub image: Option<serde_json::Value>,
        }
        pub struct EntityFilterChoiceRowBorrowed<'a> {
            pub id: i32,
            pub title: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub subtitle: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub image: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
        }
        impl<'a> From<EntityFilterChoiceRowBorrowed<'a>> for EntityFilterChoiceRow {
            fn from(
                EntityFilterChoiceRowBorrowed {
                    id,
                    title,
                    subtitle,
                    image,
                }: EntityFilterChoiceRowBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    title: serde_json::from_str(title.0.get()).unwrap(),
                    subtitle: subtitle.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    image: image.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct StringFilterChoiceRow {
            pub title: String,
        }
        pub struct StringFilterChoiceRowBorrowed<'a> {
            pub title: &'a str,
        }
        impl<'a> From<StringFilterChoiceRowBorrowed<'a>> for StringFilterChoiceRow {
            fn from(
                StringFilterChoiceRowBorrowed { title }: StringFilterChoiceRowBorrowed<'a>,
            ) -> Self {
                Self {
                    title: title.into(),
                }
            }
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct EntityFilterChoiceRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> EntityFilterChoiceRowBorrowed,
            mapper: fn(EntityFilterChoiceRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> EntityFilterChoiceRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(EntityFilterChoiceRowBorrowed) -> R,
            ) -> EntityFilterChoiceRowQuery<'a, C, R, N> {
                EntityFilterChoiceRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct StringFilterChoiceRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> StringFilterChoiceRowBorrowed,
            mapper: fn(StringFilterChoiceRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringFilterChoiceRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(StringFilterChoiceRowBorrowed) -> R,
            ) -> StringFilterChoiceRowQuery<'a, C, R, N> {
                StringFilterChoiceRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_style_filter_choices() -> SelectStyleFilterChoicesStmt {
            SelectStyleFilterChoicesStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    style.id,
    style.\"name\" AS title,
    jsonb_build_object('en', style.\"number\") AS subtitle,
    to_jsonb(main_image.json_data) AS image
FROM style
LEFT JOIN (
    SELECT
        color.style_id,
        row_number()
        OVER (PARTITION BY color.style_id ORDER BY image.uploaded_at DESC)
        AS rowno,
        jsonb_build_object(
            'id',
            image.id,
            'external_id',
            image.external_id,
            'url',
            image.url
        ) AS json_data
    FROM color
    INNER JOIN image ON image.color_id = color.id
    WHERE image.position = 1
) AS main_image ON main_image.style_id = style.id AND main_image.rowno = 1
WHERE style.organization_id = $1
ORDER BY title",
            ))
        }
        pub struct SelectStyleFilterChoicesStmt(cornucopia_async::private::Stmt);
        impl SelectStyleFilterChoicesStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
            ) -> EntityFilterChoiceRowQuery<'a, C, EntityFilterChoiceRow, 1> {
                EntityFilterChoiceRowQuery {
                    client,
                    params: [organization_id],
                    stmt: &mut self.0,
                    extractor: |row| EntityFilterChoiceRowBorrowed {
                        id: row.get(0),
                        title: row.get(1),
                        subtitle: row.get(2),
                        image: row.get(3),
                    },
                    mapper: |it| <EntityFilterChoiceRow>::from(it),
                }
            }
        }
        pub fn select_category_filter_choices() -> SelectCategoryFilterChoicesStmt {
            SelectCategoryFilterChoicesStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    category.id,
    category.\"name\" AS title,
    NULL::jsonb AS subtitle,
    NULL::jsonb AS image
FROM category WHERE category.organization_id = $1
ORDER BY title",
            ))
        }
        pub struct SelectCategoryFilterChoicesStmt(cornucopia_async::private::Stmt);
        impl SelectCategoryFilterChoicesStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
            ) -> EntityFilterChoiceRowQuery<'a, C, EntityFilterChoiceRow, 1> {
                EntityFilterChoiceRowQuery {
                    client,
                    params: [organization_id],
                    stmt: &mut self.0,
                    extractor: |row| EntityFilterChoiceRowBorrowed {
                        id: row.get(0),
                        title: row.get(1),
                        subtitle: row.get(2),
                        image: row.get(3),
                    },
                    mapper: |it| <EntityFilterChoiceRow>::from(it),
                }
            }
        }
        pub fn select_attribute_filter_choices() -> SelectAttributeFilterChoicesStmt {
            SelectAttributeFilterChoicesStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    \"attribute\".id,
    \"attribute\".title,
    attributetype.\"name\" AS subtitle,
    NULL::jsonb AS image
FROM \"attribute\"
INNER JOIN attributetype ON attributetype.id = \"attribute\".type_id
WHERE \"attribute\".organization_id = $1
ORDER BY \"attribute\".title",
            ))
        }
        pub struct SelectAttributeFilterChoicesStmt(cornucopia_async::private::Stmt);
        impl SelectAttributeFilterChoicesStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
            ) -> EntityFilterChoiceRowQuery<'a, C, EntityFilterChoiceRow, 1> {
                EntityFilterChoiceRowQuery {
                    client,
                    params: [organization_id],
                    stmt: &mut self.0,
                    extractor: |row| EntityFilterChoiceRowBorrowed {
                        id: row.get(0),
                        title: row.get(1),
                        subtitle: row.get(2),
                        image: row.get(3),
                    },
                    mapper: |it| <EntityFilterChoiceRow>::from(it),
                }
            }
        }
        pub fn select_status_filter_choices() -> SelectStatusFilterChoicesStmt {
            SelectStatusFilterChoicesStmt(cornucopia_async::private::Stmt::new(
                "SELECT DISTINCT size.status AS title FROM size
WHERE size.organization_id = $1
ORDER BY title",
            ))
        }
        pub struct SelectStatusFilterChoicesStmt(cornucopia_async::private::Stmt);
        impl SelectStatusFilterChoicesStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
            ) -> StringFilterChoiceRowQuery<'a, C, StringFilterChoiceRow, 1> {
                StringFilterChoiceRowQuery {
                    client,
                    params: [organization_id],
                    stmt: &mut self.0,
                    extractor: |row| StringFilterChoiceRowBorrowed { title: row.get(0) },
                    mapper: |it| <StringFilterChoiceRow>::from(it),
                }
            }
        }
    }
    pub mod attribute {
        #[derive(Debug)]
        pub struct GetAttributeParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetAttributeIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertAttributeParams<
            T1: cornucopia_async::JsonSql,
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
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
            T1: cornucopia_async::JsonSql,
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
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
        pub struct AssociateStyleAttributesParams<T1: cornucopia_async::ArraySql<Item = i32>> {
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct AttributeRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> AttributeRowBorrowed,
            mapper: fn(AttributeRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> AttributeRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(AttributeRowBorrowed) -> R,
            ) -> AttributeRowQuery<'a, C, R, N> {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_attributes() -> ListAttributesStmt {
            ListAttributesStmt(cornucopia_async::private::Stmt::new(
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
        pub struct ListAttributesStmt(cornucopia_async::private::Stmt);
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
            GetAttributeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetAttributeStmt(cornucopia_async::private::Stmt);
        impl GetAttributeStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            GetAttributeIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetAttributeIdStmt(cornucopia_async::private::Stmt);
        impl GetAttributeIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetAttributeIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
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
            InsertAttributeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertAttributeStmt(cornucopia_async::private::Stmt);
        impl InsertAttributeStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
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
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            UpdateAttributeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateAttributeStmt(cornucopia_async::private::Stmt);
        impl UpdateAttributeStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
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
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateAttributeParams<T1, T2, T3, T4>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteAttributeStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM attribute
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeleteAttributeStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteAttributeParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            AssociateStyleAttributesStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    associate_style_attributes($1, $2)",
            ))
        }
        pub struct AssociateStyleAttributesStmt(cornucopia_async::private::Stmt);
        impl AssociateStyleAttributesStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
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
    }
    pub mod attributetype {
        #[derive(Debug)]
        pub struct GetAttributeTypeParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetAttributeTypeIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertAttributeTypeParams<
            T1: cornucopia_async::JsonSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub name: T1,
            pub slug: T2,
            pub external_id: Option<T3>,
            pub organization_id: i32,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdateAttributeTypeParams<
            T1: cornucopia_async::JsonSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
        }
        pub struct AttributeTypeRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct AttributeTypeRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_attribute_types() -> ListAttributeTypesStmt {
            ListAttributeTypesStmt(cornucopia_async::private::Stmt::new(
                "SELECT attributetype.*
FROM
    attributetype
WHERE
    attributetype.organization_id = $1
ORDER BY
    attributetype.updated_at DESC",
            ))
        }
        pub struct ListAttributeTypesStmt(cornucopia_async::private::Stmt);
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
            GetAttributeTypeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetAttributeTypeStmt(cornucopia_async::private::Stmt);
        impl GetAttributeTypeStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            GetAttributeTypeIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetAttributeTypeIdStmt(cornucopia_async::private::Stmt);
        impl GetAttributeTypeIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                GetAttributeTypeIdParams<T1, T2>,
                I32Query<'a, C, i32, 4>,
                C,
            > for GetAttributeTypeIdStmt
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
            InsertAttributeTypeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertAttributeTypeStmt(cornucopia_async::private::Stmt);
        impl InsertAttributeTypeStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            UpdateAttributeTypeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateAttributeTypeStmt(cornucopia_async::private::Stmt);
        impl UpdateAttributeTypeStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
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
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateAttributeTypeParams<T1, T2, T3>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteAttributeTypeStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM attributetype
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeleteAttributeTypeStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteAttributeTypeParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
    }
    pub mod category {
        #[derive(Debug)]
        pub struct GetCategoryIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetCategoryParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertCategoryParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::JsonSql,
        > {
            pub slug: T1,
            pub external_id: Option<T2>,
            pub name: T3,
            pub organization_id: i32,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdateCategoryParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::JsonSql,
        > {
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
        pub struct AssociateStyleCategoriesParams<T1: cornucopia_async::ArraySql<Item = i32>> {
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
        }
        pub struct CategoryRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct CategoryRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> CategoryRowBorrowed,
            mapper: fn(CategoryRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> CategoryRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(CategoryRowBorrowed) -> R,
            ) -> CategoryRowQuery<'a, C, R, N> {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_categories() -> ListCategoriesStmt {
            ListCategoriesStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    category
WHERE
    category.organization_id = $1",
            ))
        }
        pub struct ListCategoriesStmt(cornucopia_async::private::Stmt);
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
            GetCategoryIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetCategoryIdStmt(cornucopia_async::private::Stmt);
        impl GetCategoryIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetCategoryIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
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
            GetCategoryStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetCategoryStmt(cornucopia_async::private::Stmt);
        impl GetCategoryStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            InsertCategoryStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertCategoryStmt(cornucopia_async::private::Stmt);
        impl InsertCategoryStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::JsonSql,
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::JsonSql,
            >
            cornucopia_async::Params<
                'a,
                InsertCategoryParams<T1, T2, T3>,
                I32Query<'a, C, i32, 5>,
                C,
            > for InsertCategoryStmt
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
            UpdateCategoryStmt(cornucopia_async::private::Stmt::new(
                "UPDATE category
SET
    slug = coalesce($1, slug),
    external_id = coalesce($2, external_id),
    name = coalesce($3, name)
WHERE
    category.id = $4",
            ))
        }
        pub struct UpdateCategoryStmt(cornucopia_async::private::Stmt);
        impl UpdateCategoryStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::JsonSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::JsonSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateCategoryParams<T1, T2, T3>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteCategoryStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM category
WHERE
    organization_id = $1
    AND id = $2",
            ))
        }
        pub struct DeleteCategoryStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteCategoryParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            AssociateStyleCategoriesStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    associate_style_categories($1, $2)",
            ))
        }
        pub struct AssociateStyleCategoriesStmt(cornucopia_async::private::Stmt);
        impl AssociateStyleCategoriesStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
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
    }
    pub mod collection {
        #[derive(Debug)]
        pub struct SelectCollectionsParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub requester_id: i32,
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct SelectCollectionSummariesParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub requester_id: i32,
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetCollectionIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertCollectionParams<
            T1: cornucopia_async::JsonSql,
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
            T5: cornucopia_async::StringSql,
        > {
            pub acronym: T1,
            pub name: T2,
            pub image_url: Option<T3>,
            pub slug: T4,
            pub external_id: Option<T5>,
            pub organization_id: i32,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdateCollectionParams<
            T1: cornucopia_async::JsonSql,
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
            T5: cornucopia_async::StringSql,
        > {
            pub acronym: Option<T1>,
            pub name: Option<T2>,
            pub image_url: Option<T3>,
            pub slug: Option<T4>,
            pub external_id: Option<T5>,
            pub id: i32,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct DeleteCollectionParams {
            pub organization_id: i32,
            pub id: i32,
        }
        #[derive(Debug)]
        pub struct AssociateCollectionSizesParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub collection_id: i32,
            pub size_ids: T1,
        }
        #[derive(Debug)]
        pub struct ReplaceCollectionPricelistsParams<
            T1: cornucopia_async::ArraySql<
                Item = super::super::types::public::CollectionPricelistRelation,
            >,
        > {
            pub collection_id: i32,
            pub collection_pricelist_relations: T1,
        }
        #[derive(Debug)]
        pub struct SetNewCollectionStylesParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub collection_id: i32,
            pub style_ids: T1,
        }
        #[derive(Debug)]
        pub struct SetNewCollectionColorsParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub collection_id: i32,
            pub color_ids: T1,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectionRow {
            pub id: i32,
            pub organization_id: i32,
            pub slug: String,
            pub external_id: Option<String>,
            pub name: serde_json::Value,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub image_url: Option<String>,
            pub acronym: serde_json::Value,
            pub pricing: serde_json::Value,
            pub sizes: serde_json::Value,
        }
        pub struct CollectionRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub image_url: Option<&'a str>,
            pub acronym: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub pricing: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub sizes: postgres_types::Json<&'a serde_json::value::RawValue>,
        }
        impl<'a> From<CollectionRowBorrowed<'a>> for CollectionRow {
            fn from(
                CollectionRowBorrowed {
                    id,
                    organization_id,
                    slug,
                    external_id,
                    name,
                    created_by,
                    created_at,
                    updated_at,
                    image_url,
                    acronym,
                    pricing,
                    sizes,
                }: CollectionRowBorrowed<'a>,
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
                    image_url: image_url.map(|v| v.into()),
                    acronym: serde_json::from_str(acronym.0.get()).unwrap(),
                    pricing: serde_json::from_str(pricing.0.get()).unwrap(),
                    sizes: serde_json::from_str(sizes.0.get()).unwrap(),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CollectionSummaryRow {
            pub id: i32,
            pub organization_id: i32,
            pub slug: String,
            pub external_id: Option<String>,
            pub name: serde_json::Value,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub image_url: Option<String>,
            pub acronym: serde_json::Value,
            pub num_sizes: i64,
            pub num_colors: i64,
            pub num_styles: i64,
            pub pricing: serde_json::Value,
        }
        pub struct CollectionSummaryRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub image_url: Option<&'a str>,
            pub acronym: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub num_sizes: i64,
            pub num_colors: i64,
            pub num_styles: i64,
            pub pricing: postgres_types::Json<&'a serde_json::value::RawValue>,
        }
        impl<'a> From<CollectionSummaryRowBorrowed<'a>> for CollectionSummaryRow {
            fn from(
                CollectionSummaryRowBorrowed {
                    id,
                    organization_id,
                    slug,
                    external_id,
                    name,
                    created_by,
                    created_at,
                    updated_at,
                    image_url,
                    acronym,
                    num_sizes,
                    num_colors,
                    num_styles,
                    pricing,
                }: CollectionSummaryRowBorrowed<'a>,
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
                    image_url: image_url.map(|v| v.into()),
                    acronym: serde_json::from_str(acronym.0.get()).unwrap(),
                    num_sizes,
                    num_colors,
                    num_styles,
                    pricing: serde_json::from_str(pricing.0.get()).unwrap(),
                }
            }
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct CollectionRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> CollectionRowBorrowed,
            mapper: fn(CollectionRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> CollectionRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(CollectionRowBorrowed) -> R,
            ) -> CollectionRowQuery<'a, C, R, N> {
                CollectionRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct CollectionSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> CollectionSummaryRowBorrowed,
            mapper: fn(CollectionSummaryRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> CollectionSummaryRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(CollectionSummaryRowBorrowed) -> R,
            ) -> CollectionSummaryRowQuery<'a, C, R, N> {
                CollectionSummaryRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_collections() -> SelectCollectionsStmt {
            SelectCollectionsStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    collection.*,
    coalesce(pricing.json_data, '[]'::json) AS pricing,
    coalesce(sizes.json_data, '[]'::json) AS sizes
FROM
    collection
INNER JOIN (
    SELECT group_collection.collection_id
    FROM group_collection
    INNER JOIN group_user
        ON group_user.group_id = group_collection.group_id
    WHERE
        group_user.user_id = $1
    GROUP BY group_collection.collection_id
    UNION
    SELECT collection.id AS collection_id
    FROM collection
    WHERE collection.created_by = $1
) AS requester_collections ON requester_collections.collection_id = collection.id
LEFT JOIN (
    SELECT
        size_collection.collection_id,
        json_agg(
            size.* ORDER BY size_collection.position
        ) FILTER (WHERE size.id IS NOT NULL) AS json_data
    FROM size
    INNER JOIN size_collection ON size_collection.size_id = size.id
    GROUP BY size_collection.collection_id
) AS sizes ON sizes.collection_id = collection.id
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
WHERE
    collection.organization_id = $2
    AND ($3::int IS NULL OR collection.id = $3)
    AND ($4::text IS NULL OR collection.external_id = $4)
    AND ($5::text IS NULL OR collection.slug = $5)
ORDER BY
    collection.updated_at DESC",
            ))
        }
        pub struct SelectCollectionsStmt(cornucopia_async::private::Stmt);
        impl SelectCollectionsStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                requester_id: &'a i32,
                organization_id: &'a i32,
                id: &'a Option<i32>,
                external_id: &'a Option<T1>,
                slug: &'a Option<T2>,
            ) -> CollectionRowQuery<'a, C, CollectionRow, 5> {
                CollectionRowQuery {
                    client,
                    params: [requester_id, organization_id, id, external_id, slug],
                    stmt: &mut self.0,
                    extractor: |row| CollectionRowBorrowed {
                        id: row.get(0),
                        organization_id: row.get(1),
                        slug: row.get(2),
                        external_id: row.get(3),
                        name: row.get(4),
                        created_by: row.get(5),
                        created_at: row.get(6),
                        updated_at: row.get(7),
                        image_url: row.get(8),
                        acronym: row.get(9),
                        pricing: row.get(10),
                        sizes: row.get(11),
                    },
                    mapper: |it| <CollectionRow>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                SelectCollectionsParams<T1, T2>,
                CollectionRowQuery<'a, C, CollectionRow, 5>,
                C,
            > for SelectCollectionsStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SelectCollectionsParams<T1, T2>,
            ) -> CollectionRowQuery<'a, C, CollectionRow, 5> {
                self.bind(
                    client,
                    &params.requester_id,
                    &params.organization_id,
                    &params.id,
                    &params.external_id,
                    &params.slug,
                )
            }
        }
        pub fn select_collection_summaries() -> SelectCollectionSummariesStmt {
            SelectCollectionSummariesStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    collection.*,
    coalesce(stats.num_sizes, 0) AS num_sizes,
    coalesce(stats.num_colors, 0) AS num_colors,
    coalesce(stats.num_styles, 0) AS num_styles,
    coalesce(pricing.json_data, '[]'::json) AS pricing
FROM
    collection
INNER JOIN (
    SELECT group_collection.collection_id
    FROM group_collection
    INNER JOIN group_user
        ON group_user.group_id = group_collection.group_id
    WHERE
        group_user.user_id = $1
    GROUP BY group_collection.collection_id
    UNION
    SELECT collection.id AS collection_id
    FROM collection
    WHERE collection.created_by = $1
) AS requester_collections ON requester_collections.collection_id = collection.id
LEFT JOIN (
    SELECT
        size_collection.collection_id,
        count(DISTINCT size.id) AS num_sizes,
        count(DISTINCT color.id) AS num_colors,
        count(DISTINCT color.style_id) AS num_styles
    FROM color
    INNER JOIN size ON size.color_id = color.id
    INNER JOIN size_collection ON size.id = size_collection.size_id
    GROUP BY size_collection.collection_id
) AS stats ON stats.collection_id = collection.id
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
        ) FILTER (WHERE collection_pricelist.collection_id IS NOT NULL) AS json_data,
        min(collection_pricelist.price_date) AS min_price_date
    FROM collection_pricelist
    INNER JOIN pricelist
        ON pricelist.id = collection_pricelist.pricelist_id
    WHERE collection_pricelist.collection_id = collection.id
    GROUP BY collection_pricelist.collection_id
) AS pricing ON pricing.collection_id = collection.id
WHERE
    collection.organization_id = $2
    AND ($3::int IS NULL OR collection.id = $3)
    AND ($4::text IS NULL OR collection.external_id = $4)
    AND ($5::text IS NULL OR collection.slug = $5)
ORDER BY
    pricing.min_price_date DESC, collection.name ASC",
            ))
        }
        pub struct SelectCollectionSummariesStmt(cornucopia_async::private::Stmt);
        impl SelectCollectionSummariesStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                requester_id: &'a i32,
                organization_id: &'a i32,
                id: &'a Option<i32>,
                external_id: &'a Option<T1>,
                slug: &'a Option<T2>,
            ) -> CollectionSummaryRowQuery<'a, C, CollectionSummaryRow, 5> {
                CollectionSummaryRowQuery {
                    client,
                    params: [requester_id, organization_id, id, external_id, slug],
                    stmt: &mut self.0,
                    extractor: |row| CollectionSummaryRowBorrowed {
                        id: row.get(0),
                        organization_id: row.get(1),
                        slug: row.get(2),
                        external_id: row.get(3),
                        name: row.get(4),
                        created_by: row.get(5),
                        created_at: row.get(6),
                        updated_at: row.get(7),
                        image_url: row.get(8),
                        acronym: row.get(9),
                        num_sizes: row.get(10),
                        num_colors: row.get(11),
                        num_styles: row.get(12),
                        pricing: row.get(13),
                    },
                    mapper: |it| <CollectionSummaryRow>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                SelectCollectionSummariesParams<T1, T2>,
                CollectionSummaryRowQuery<'a, C, CollectionSummaryRow, 5>,
                C,
            > for SelectCollectionSummariesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SelectCollectionSummariesParams<T1, T2>,
            ) -> CollectionSummaryRowQuery<'a, C, CollectionSummaryRow, 5> {
                self.bind(
                    client,
                    &params.requester_id,
                    &params.organization_id,
                    &params.id,
                    &params.external_id,
                    &params.slug,
                )
            }
        }
        pub fn get_collection_id() -> GetCollectionIdStmt {
            GetCollectionIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT collection.id
FROM
    collection
WHERE
    collection.organization_id = $1
    AND ($2::int IS NULL OR collection.id = $2)
    AND ($3::text IS NULL OR collection.external_id = $3)
    AND ($4::text IS NULL OR collection.slug = $4)",
            ))
        }
        pub struct GetCollectionIdStmt(cornucopia_async::private::Stmt);
        impl GetCollectionIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetCollectionIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
            for GetCollectionIdStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetCollectionIdParams<T1, T2>,
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
        pub fn insert_collection() -> InsertCollectionStmt {
            InsertCollectionStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO collection (
    acronym,
    name,
    image_url,
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
        pub struct InsertCollectionStmt(cornucopia_async::private::Stmt);
        impl InsertCollectionStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                acronym: &'a T1,
                name: &'a T2,
                image_url: &'a Option<T3>,
                slug: &'a T4,
                external_id: &'a Option<T5>,
                organization_id: &'a i32,
                created_by: &'a i32,
            ) -> I32Query<'a, C, i32, 7> {
                I32Query {
                    client,
                    params: [
                        acronym,
                        name,
                        image_url,
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
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertCollectionParams<T1, T2, T3, T4, T5>,
                I32Query<'a, C, i32, 7>,
                C,
            > for InsertCollectionStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertCollectionParams<T1, T2, T3, T4, T5>,
            ) -> I32Query<'a, C, i32, 7> {
                self.bind(
                    client,
                    &params.acronym,
                    &params.name,
                    &params.image_url,
                    &params.slug,
                    &params.external_id,
                    &params.organization_id,
                    &params.created_by,
                )
            }
        }
        pub fn update_collection() -> UpdateCollectionStmt {
            UpdateCollectionStmt(cornucopia_async::private::Stmt::new(
                "UPDATE
collection
SET
    acronym = coalesce($1, acronym),
    name = coalesce($2, name),
    image_url = coalesce($3, image_url),
    slug = coalesce($4, slug),
    external_id = coalesce($5, external_id)
WHERE
    id = $6",
            ))
        }
        pub struct UpdateCollectionStmt(cornucopia_async::private::Stmt);
        impl UpdateCollectionStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                acronym: &'a Option<T1>,
                name: &'a Option<T2>,
                image_url: &'a Option<T3>,
                slug: &'a Option<T4>,
                external_id: &'a Option<T5>,
                id: &'a i32,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(stmt, &[acronym, name, image_url, slug, external_id, id])
                    .await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::JsonSql,
                T2: cornucopia_async::JsonSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateCollectionParams<T1, T2, T3, T4, T5>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for UpdateCollectionStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpdateCollectionParams<T1, T2, T3, T4, T5>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(
                    client,
                    &params.acronym,
                    &params.name,
                    &params.image_url,
                    &params.slug,
                    &params.external_id,
                    &params.id,
                ))
            }
        }
        pub fn delete_collection() -> DeleteCollectionStmt {
            DeleteCollectionStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM collection
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeleteCollectionStmt(cornucopia_async::private::Stmt);
        impl DeleteCollectionStmt {
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
            cornucopia_async::Params<
                'a,
                DeleteCollectionParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for DeleteCollectionStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a DeleteCollectionParams,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.organization_id, &params.id))
            }
        }
        pub fn associate_collection_sizes() -> AssociateCollectionSizesStmt {
            AssociateCollectionSizesStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    associate_collection_sizes($1, $2)",
            ))
        }
        pub struct AssociateCollectionSizesStmt(cornucopia_async::private::Stmt);
        impl AssociateCollectionSizesStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
                &'a mut self,
                client: &'a C,
                collection_id: &'a i32,
                size_ids: &'a T1,
            ) -> I32Query<'a, C, i32, 2> {
                I32Query {
                    client,
                    params: [collection_id, size_ids],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                AssociateCollectionSizesParams<T1>,
                I32Query<'a, C, i32, 2>,
                C,
            > for AssociateCollectionSizesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a AssociateCollectionSizesParams<T1>,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(client, &params.collection_id, &params.size_ids)
            }
        }
        pub fn replace_collection_pricelists() -> ReplaceCollectionPricelistsStmt {
            ReplaceCollectionPricelistsStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    replace_collection_pricelists($1, $2)",
            ))
        }
        pub struct ReplaceCollectionPricelistsStmt(cornucopia_async::private::Stmt);
        impl ReplaceCollectionPricelistsStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::ArraySql<
                    Item = super::super::types::public::CollectionPricelistRelation,
                >,
            >(
                &'a mut self,
                client: &'a C,
                collection_id: &'a i32,
                collection_pricelist_relations: &'a T1,
            ) -> I32Query<'a, C, i32, 2> {
                I32Query {
                    client,
                    params: [collection_id, collection_pricelist_relations],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::ArraySql<
                    Item = super::super::types::public::CollectionPricelistRelation,
                >,
            >
            cornucopia_async::Params<
                'a,
                ReplaceCollectionPricelistsParams<T1>,
                I32Query<'a, C, i32, 2>,
                C,
            > for ReplaceCollectionPricelistsStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a ReplaceCollectionPricelistsParams<T1>,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(
                    client,
                    &params.collection_id,
                    &params.collection_pricelist_relations,
                )
            }
        }
        pub fn set_new_collection_styles() -> SetNewCollectionStylesStmt {
            SetNewCollectionStylesStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    set_new_collection_styles($1, $2)",
            ))
        }
        pub struct SetNewCollectionStylesStmt(cornucopia_async::private::Stmt);
        impl SetNewCollectionStylesStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
                &'a mut self,
                client: &'a C,
                collection_id: &'a i32,
                style_ids: &'a T1,
            ) -> I32Query<'a, C, i32, 2> {
                I32Query {
                    client,
                    params: [collection_id, style_ids],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                SetNewCollectionStylesParams<T1>,
                I32Query<'a, C, i32, 2>,
                C,
            > for SetNewCollectionStylesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SetNewCollectionStylesParams<T1>,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(client, &params.collection_id, &params.style_ids)
            }
        }
        pub fn set_new_collection_colors() -> SetNewCollectionColorsStmt {
            SetNewCollectionColorsStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    set_new_collection_colors($1, $2)",
            ))
        }
        pub struct SetNewCollectionColorsStmt(cornucopia_async::private::Stmt);
        impl SetNewCollectionColorsStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
                &'a mut self,
                client: &'a C,
                collection_id: &'a i32,
                color_ids: &'a T1,
            ) -> I32Query<'a, C, i32, 2> {
                I32Query {
                    client,
                    params: [collection_id, color_ids],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                SetNewCollectionColorsParams<T1>,
                I32Query<'a, C, i32, 2>,
                C,
            > for SetNewCollectionColorsStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SetNewCollectionColorsParams<T1>,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(client, &params.collection_id, &params.color_ids)
            }
        }
    }
    pub mod color {
        #[derive(Debug)]
        pub struct GetColorParams<T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetColorIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetColorRefsParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertColorParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::JsonSql,
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
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::JsonSql,
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct ColorRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct ColorRefsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_colors() -> ListColorsStmt {
            ListColorsStmt(cornucopia_async::private::Stmt::new(
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
            ORDER BY image.position) FILTER (WHERE image.id IS NOT NULL
        ), '[]'::jsonb) AS \"images\"
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
        pub struct ListColorsStmt(cornucopia_async::private::Stmt);
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
            GetColorStmt(cornucopia_async::private::Stmt::new(
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
            ORDER BY image.position) FILTER (WHERE image.id IS NOT NULL
        ), '[]'::jsonb) AS \"images\"
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
        pub struct GetColorStmt(cornucopia_async::private::Stmt);
        impl GetColorStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                GetColorParams<T1, T2>,
                ColorRowQuery<'a, C, ColorRow, 4>,
                C,
            > for GetColorStmt
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
            GetColorIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetColorIdStmt(cornucopia_async::private::Stmt);
        impl GetColorIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetColorIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
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
            GetColorRefsStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetColorRefsStmt(cornucopia_async::private::Stmt);
        impl GetColorRefsStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            InsertColorStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertColorStmt(cornucopia_async::private::Stmt);
        impl InsertColorStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
            >
            cornucopia_async::Params<
                'a,
                InsertColorParams<T1, T2, T3, T4>,
                I32Query<'a, C, i32, 7>,
                C,
            > for InsertColorStmt
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
            UpdateColorStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateColorStmt(cornucopia_async::private::Stmt);
        impl UpdateColorStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateColorParams<T1, T2, T3, T4>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteColorStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM color
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeleteColorStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteColorParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
    }
    pub mod group {
        #[derive(Debug)]
        pub struct SelectGroupsParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct SelectGroupSummariesParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetGroupIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertGroupParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
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
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
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
        pub struct ReplaceGroupUsersParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub group_id: i32,
            pub user_ids: T1,
        }
        #[derive(Debug)]
        pub struct ReplaceGroupCollectionsParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub group_id: i32,
            pub collection_ids: T1,
        }
        #[derive(Debug)]
        pub struct ReplaceGroupPricelistsParams<T1: cornucopia_async::ArraySql<Item = i32>> {
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct GroupRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct GroupSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_groups() -> SelectGroupsStmt {
            SelectGroupsStmt(cornucopia_async::private::Stmt::new(
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
        pub struct SelectGroupsStmt(cornucopia_async::private::Stmt);
        impl SelectGroupsStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            SelectGroupSummariesStmt(cornucopia_async::private::Stmt::new(
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
        pub struct SelectGroupSummariesStmt(cornucopia_async::private::Stmt);
        impl SelectGroupSummariesStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            GetGroupIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetGroupIdStmt(cornucopia_async::private::Stmt);
        impl GetGroupIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetGroupIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
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
            InsertGroupStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertGroupStmt(cornucopia_async::private::Stmt);
        impl InsertGroupStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertGroupParams<T1, T2, T3, T4>,
                I32Query<'a, C, i32, 6>,
                C,
            > for InsertGroupStmt
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
            UpdateGroupStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateGroupStmt(cornucopia_async::private::Stmt);
        impl UpdateGroupStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateGroupParams<T1, T2, T3, T4>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteGroupStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM \"group\"
WHERE
    organization_id = $1
    AND id = $2",
            ))
        }
        pub struct DeleteGroupStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteGroupParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            ReplaceGroupUsersStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    replace_group_users($1, $2)",
            ))
        }
        pub struct ReplaceGroupUsersStmt(cornucopia_async::private::Stmt);
        impl ReplaceGroupUsersStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<'a, ReplaceGroupUsersParams<T1>, I32Query<'a, C, i32, 2>, C>
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
            ReplaceGroupCollectionsStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    replace_group_collections($1, $2)",
            ))
        }
        pub struct ReplaceGroupCollectionsStmt(cornucopia_async::private::Stmt);
        impl ReplaceGroupCollectionsStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                ReplaceGroupCollectionsParams<T1>,
                I32Query<'a, C, i32, 2>,
                C,
            > for ReplaceGroupCollectionsStmt
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
            ReplaceGroupPricelistsStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    replace_group_pricelists($1, $2)",
            ))
        }
        pub struct ReplaceGroupPricelistsStmt(cornucopia_async::private::Stmt);
        impl ReplaceGroupPricelistsStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                ReplaceGroupPricelistsParams<T1>,
                I32Query<'a, C, i32, 2>,
                C,
            > for ReplaceGroupPricelistsStmt
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
            EnsureSuperuserAccessStmt(cornucopia_async::private::Stmt::new(
                "SELECT ensure_superuser_access($1)",
            ))
        }
        pub struct EnsureSuperuserAccessStmt(cornucopia_async::private::Stmt);
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
    }
    pub mod image {
        #[derive(Debug)]
        pub struct GetImageIdParams<T1: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
        }
        #[derive(Debug)]
        pub struct GetImageUrlByExternalChecksumParams<T1: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub external_checksum: T1,
        }
        #[derive(Debug)]
        pub struct GetImageParams<T1: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
        }
        #[derive(Debug)]
        pub struct InsertImageParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub color_id: i32,
            pub external_id: Option<T1>,
            pub url: T2,
            pub organization_id: i32,
            pub uploaded_by: i32,
            pub external_checksum: Option<T3>,
            pub position: i32,
        }
        #[derive(Debug)]
        pub struct UpdateImageParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
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
            pub uploaded_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
            pub uploaded_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct ImageRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_images() -> ListImagesStmt {
            ListImagesStmt(cornucopia_async::private::Stmt::new(
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
        pub struct ListImagesStmt(cornucopia_async::private::Stmt);
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
            GetImageIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetImageIdStmt(cornucopia_async::private::Stmt);
        impl GetImageIdStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<'a, GetImageIdParams<T1>, I32Query<'a, C, i32, 3>, C>
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
            GetImageUrlByExternalChecksumStmt(cornucopia_async::private::Stmt::new(
                "SELECT image.url
FROM
    image
WHERE
    image.organization_id = $1
    AND image.external_checksum = $2",
            ))
        }
        pub struct GetImageUrlByExternalChecksumStmt(cornucopia_async::private::Stmt);
        impl GetImageUrlByExternalChecksumStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
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
            GetImageStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetImageStmt(cornucopia_async::private::Stmt);
        impl GetImageStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<'a, GetImageParams<T1>, ImageRowQuery<'a, C, ImageRow, 3>, C>
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
            InsertImageStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertImageStmt(cornucopia_async::private::Stmt);
        impl InsertImageStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, InsertImageParams<T1, T2, T3>, I32Query<'a, C, i32, 7>, C>
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
            UpdateImageStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateImageStmt(cornucopia_async::private::Stmt);
        impl UpdateImageStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateImageParams<T1, T2, T3>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteImageStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM image
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeleteImageStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteImageParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
    }
    pub mod misc {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn migrate_revision() -> MigrateRevisionStmt {
            MigrateRevisionStmt(cornucopia_async::private::Stmt::new(
                "SELECT migrations.revision
FROM
    migrations",
            ))
        }
        pub struct MigrateRevisionStmt(cornucopia_async::private::Stmt);
        impl MigrateRevisionStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> I32Query<'a, C, i32, 0> {
                I32Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        pub fn set_migrate_revision() -> SetMigrateRevisionStmt {
            SetMigrateRevisionStmt(cornucopia_async::private::Stmt::new(
                "SELECT set_migrate_revision($1)",
            ))
        }
        pub struct SetMigrateRevisionStmt(cornucopia_async::private::Stmt);
        impl SetMigrateRevisionStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                revision: &'a i32,
            ) -> I32Query<'a, C, i32, 1> {
                I32Query {
                    client,
                    params: [revision],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
    }
    pub mod organization {
        #[derive(Debug)]
        pub struct InsertOrganizationParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub name: T1,
            pub logo_url: Option<T2>,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdateOrganizationParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub name: Option<T1>,
            pub logo_url: Option<T2>,
            pub id: i32,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OrganizationRow {
            pub id: i32,
            pub name: String,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub logo_url: Option<String>,
        }
        pub struct OrganizationRowBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
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
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct OrganizationRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn get_organization() -> GetOrganizationStmt {
            GetOrganizationStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    organization
WHERE
    organization.id = $1",
            ))
        }
        pub struct GetOrganizationStmt(cornucopia_async::private::Stmt);
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
            GetOrganizationIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT organization.id
FROM
    organization
WHERE
    organization.id = $1",
            ))
        }
        pub struct GetOrganizationIdStmt(cornucopia_async::private::Stmt);
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
            InsertOrganizationStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertOrganizationStmt(cornucopia_async::private::Stmt);
        impl InsertOrganizationStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            UpdateOrganizationStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateOrganizationStmt(cornucopia_async::private::Stmt);
        impl UpdateOrganizationStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            DeleteOrganizationStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM organization
WHERE id = $1
RETURNING
id",
            ))
        }
        pub struct DeleteOrganizationStmt(cornucopia_async::private::Stmt);
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
            ListUserOrganizationsStmt(cornucopia_async::private::Stmt::new(
                "SELECT organization.*
FROM
    organization
INNER JOIN
    user_organization ON organization.id = user_organization.organization_id
WHERE
    user_organization.user_id = $1",
            ))
        }
        pub struct ListUserOrganizationsStmt(cornucopia_async::private::Stmt);
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
    }
    pub mod price {
        #[derive(Debug)]
        pub struct SelectPricesParams<
            T1: cornucopia_async::ArraySql<Item = i32>,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::ArraySql<Item = T2>,
        > {
            pub organization_id: i32,
            pub ids: Option<T1>,
            pub external_ids: Option<T3>,
        }
        #[derive(Debug)]
        pub struct GetPriceIdParams<T1: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
        }
        #[derive(Debug)]
        pub struct InsertPriceParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub r#type: super::super::types::public::Pricetype,
            pub uom: Option<T1>,
            pub currency: T2,
            pub amount: rust_decimal::Decimal,
            pub start: time::Date,
            pub end: time::Date,
            pub style_id: i32,
            pub list_id: i32,
            pub external_id: Option<T3>,
            pub organization_id: i32,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdatePriceParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub r#type: Option<super::super::types::public::Pricetype>,
            pub uom: Option<T1>,
            pub currency: Option<T2>,
            pub amount: Option<rust_decimal::Decimal>,
            pub start: Option<time::Date>,
            pub end: Option<time::Date>,
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
            pub r#type: super::super::types::public::Pricetype,
            pub currency: String,
            pub uom: Option<String>,
            pub list_id: i32,
            pub external_id: Option<String>,
            pub style_id: i32,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub amount: rust_decimal::Decimal,
            pub start: time::Date,
            pub end: time::Date,
            pub style: serde_json::Value,
            pub list: serde_json::Value,
        }
        pub struct PriceRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub r#type: super::super::types::public::Pricetype,
            pub currency: &'a str,
            pub uom: Option<&'a str>,
            pub list_id: i32,
            pub external_id: Option<&'a str>,
            pub style_id: i32,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub amount: rust_decimal::Decimal,
            pub start: time::Date,
            pub end: time::Date,
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
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct PriceRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_prices() -> SelectPricesStmt {
            SelectPricesStmt(cornucopia_async::private::Stmt::new(
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
        pub struct SelectPricesStmt(cornucopia_async::private::Stmt);
        impl SelectPricesStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::ArraySql<Item = i32>,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::ArraySql<Item = T2>,
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
                T1: cornucopia_async::ArraySql<Item = i32>,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::ArraySql<Item = T2>,
            >
            cornucopia_async::Params<
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
            GetPriceIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT price.id
FROM
    price
WHERE
    price.organization_id = $1
    AND ($2::int IS NULL OR price.id = $2)
    AND ($3::text IS NULL OR price.external_id = $3)",
            ))
        }
        pub struct GetPriceIdStmt(cornucopia_async::private::Stmt);
        impl GetPriceIdStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<'a, GetPriceIdParams<T1>, I32Query<'a, C, i32, 3>, C>
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
            InsertPriceStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertPriceStmt(cornucopia_async::private::Stmt);
        impl InsertPriceStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                r#type: &'a super::super::types::public::Pricetype,
                uom: &'a Option<T1>,
                currency: &'a T2,
                amount: &'a rust_decimal::Decimal,
                start: &'a time::Date,
                end: &'a time::Date,
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, InsertPriceParams<T1, T2, T3>, I32Query<'a, C, i32, 11>, C>
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
            UpdatePriceStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdatePriceStmt(cornucopia_async::private::Stmt);
        impl UpdatePriceStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                r#type: &'a Option<super::super::types::public::Pricetype>,
                uom: &'a Option<T1>,
                currency: &'a Option<T2>,
                amount: &'a Option<rust_decimal::Decimal>,
                start: &'a Option<time::Date>,
                end: &'a Option<time::Date>,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdatePriceParams<T1, T2, T3>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeletePriceStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM price
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeletePriceStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeletePriceParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
    }
    pub mod pricelist {
        #[derive(Clone, Copy, Debug)]
        pub struct ListPricelistsParams {
            pub requester_id: i32,
            pub organization_id: i32,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct ListPricelistSummariesParams {
            pub requester_id: i32,
            pub organization_id: i32,
        }
        #[derive(Debug)]
        pub struct GetPricelistParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetPricelistIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertPricelistParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub name: T1,
            pub slug: T2,
            pub external_id: Option<T3>,
            pub organization_id: i32,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdatePricelistParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub name: Option<T1>,
            pub slug: Option<T2>,
            pub external_id: Option<T3>,
            pub id: i32,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct DeletePricelistParams {
            pub organization_id: i32,
            pub id: i32,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct AllowedPricelistIdsParams {
            pub organization_id: i32,
            pub user_id: i32,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PriceListRow {
            pub id: i32,
            pub organization_id: i32,
            pub name: String,
            pub slug: String,
            pub external_id: Option<String>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
        }
        pub struct PriceListRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub name: &'a str,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
        }
        impl<'a> From<PriceListRowBorrowed<'a>> for PriceListRow {
            fn from(
                PriceListRowBorrowed {
                    id,
                    organization_id,
                    name,
                    slug,
                    external_id,
                    created_by,
                    created_at,
                    updated_at,
                }: PriceListRowBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    organization_id,
                    name: name.into(),
                    slug: slug.into(),
                    external_id: external_id.map(|v| v.into()),
                    created_by,
                    created_at,
                    updated_at,
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PriceListSummaryRow {
            pub id: i32,
            pub name: String,
            pub slug: String,
            pub external_id: Option<String>,
        }
        pub struct PriceListSummaryRowBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
        }
        impl<'a> From<PriceListSummaryRowBorrowed<'a>> for PriceListSummaryRow {
            fn from(
                PriceListSummaryRowBorrowed {
                    id,
                    name,
                    slug,
                    external_id,
                }: PriceListSummaryRowBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.into(),
                    slug: slug.into(),
                    external_id: external_id.map(|v| v.into()),
                }
            }
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct PriceListRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> PriceListRowBorrowed,
            mapper: fn(PriceListRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> PriceListRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(PriceListRowBorrowed) -> R,
            ) -> PriceListRowQuery<'a, C, R, N> {
                PriceListRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct PriceListSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> PriceListSummaryRowBorrowed,
            mapper: fn(PriceListSummaryRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> PriceListSummaryRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(PriceListSummaryRowBorrowed) -> R,
            ) -> PriceListSummaryRowQuery<'a, C, R, N> {
                PriceListSummaryRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_pricelists() -> ListPricelistsStmt {
            ListPricelistsStmt(cornucopia_async::private::Stmt::new(
                "SELECT pricelist.*
FROM
    pricelist
INNER JOIN (
    SELECT group_pricelist.pricelist_id
    FROM group_pricelist
    INNER JOIN group_user
        ON group_user.group_id = group_pricelist.group_id
    WHERE
        group_user.user_id = $1
    GROUP BY group_pricelist.pricelist_id
) AS requester_pricelists ON requester_pricelists.pricelist_id = pricelist.id
WHERE
    pricelist.organization_id = $2
ORDER BY
    pricelist.name",
            ))
        }
        pub struct ListPricelistsStmt(cornucopia_async::private::Stmt);
        impl ListPricelistsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                requester_id: &'a i32,
                organization_id: &'a i32,
            ) -> PriceListRowQuery<'a, C, PriceListRow, 2> {
                PriceListRowQuery {
                    client,
                    params: [requester_id, organization_id],
                    stmt: &mut self.0,
                    extractor: |row| PriceListRowBorrowed {
                        id: row.get(0),
                        organization_id: row.get(1),
                        name: row.get(2),
                        slug: row.get(3),
                        external_id: row.get(4),
                        created_by: row.get(5),
                        created_at: row.get(6),
                        updated_at: row.get(7),
                    },
                    mapper: |it| <PriceListRow>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                ListPricelistsParams,
                PriceListRowQuery<'a, C, PriceListRow, 2>,
                C,
            > for ListPricelistsStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a ListPricelistsParams,
            ) -> PriceListRowQuery<'a, C, PriceListRow, 2> {
                self.bind(client, &params.requester_id, &params.organization_id)
            }
        }
        pub fn list_pricelist_summaries() -> ListPricelistSummariesStmt {
            ListPricelistSummariesStmt(cornucopia_async::private::Stmt::new(
                "
SELECT
    pricelist.id,
    pricelist.name,
    pricelist.slug,
    pricelist.external_id
FROM
    pricelist
INNER JOIN (
    SELECT group_pricelist.pricelist_id
    FROM group_pricelist
    INNER JOIN group_user
        ON group_user.group_id = group_pricelist.group_id
    WHERE
        group_user.user_id = $1
    GROUP BY group_pricelist.pricelist_id
) AS requester_pricelists ON requester_pricelists.pricelist_id = pricelist.id
WHERE
    pricelist.organization_id = $2
ORDER BY
    pricelist.name",
            ))
        }
        pub struct ListPricelistSummariesStmt(cornucopia_async::private::Stmt);
        impl ListPricelistSummariesStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                requester_id: &'a i32,
                organization_id: &'a i32,
            ) -> PriceListSummaryRowQuery<'a, C, PriceListSummaryRow, 2> {
                PriceListSummaryRowQuery {
                    client,
                    params: [requester_id, organization_id],
                    stmt: &mut self.0,
                    extractor: |row| PriceListSummaryRowBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        slug: row.get(2),
                        external_id: row.get(3),
                    },
                    mapper: |it| <PriceListSummaryRow>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                ListPricelistSummariesParams,
                PriceListSummaryRowQuery<'a, C, PriceListSummaryRow, 2>,
                C,
            > for ListPricelistSummariesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a ListPricelistSummariesParams,
            ) -> PriceListSummaryRowQuery<'a, C, PriceListSummaryRow, 2> {
                self.bind(client, &params.requester_id, &params.organization_id)
            }
        }
        pub fn get_pricelist() -> GetPricelistStmt {
            GetPricelistStmt(cornucopia_async::private::Stmt::new(
                "SELECT pricelist.*
FROM
    pricelist
WHERE
    pricelist.organization_id = $1
    AND (
        ($2::int IS NULL OR pricelist.id = $2)
        AND ($3::text IS NULL OR pricelist.external_id = $3)
        AND ($4::text IS NULL OR pricelist.slug = $4)
    )",
            ))
        }
        pub struct GetPricelistStmt(cornucopia_async::private::Stmt);
        impl GetPricelistStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
                id: &'a Option<i32>,
                external_id: &'a Option<T1>,
                slug: &'a Option<T2>,
            ) -> PriceListRowQuery<'a, C, PriceListRow, 4> {
                PriceListRowQuery {
                    client,
                    params: [organization_id, id, external_id, slug],
                    stmt: &mut self.0,
                    extractor: |row| PriceListRowBorrowed {
                        id: row.get(0),
                        organization_id: row.get(1),
                        name: row.get(2),
                        slug: row.get(3),
                        external_id: row.get(4),
                        created_by: row.get(5),
                        created_at: row.get(6),
                        updated_at: row.get(7),
                    },
                    mapper: |it| <PriceListRow>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                GetPricelistParams<T1, T2>,
                PriceListRowQuery<'a, C, PriceListRow, 4>,
                C,
            > for GetPricelistStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetPricelistParams<T1, T2>,
            ) -> PriceListRowQuery<'a, C, PriceListRow, 4> {
                self.bind(
                    client,
                    &params.organization_id,
                    &params.id,
                    &params.external_id,
                    &params.slug,
                )
            }
        }
        pub fn get_pricelist_id() -> GetPricelistIdStmt {
            GetPricelistIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT pricelist.id
FROM
    pricelist
WHERE
    pricelist.organization_id = $1
    AND (
        ($2::int IS NULL OR pricelist.id = $2)
        AND ($3::text IS NULL OR pricelist.external_id = $3)
        AND ($4::text IS NULL OR pricelist.slug = $4)
    )",
            ))
        }
        pub struct GetPricelistIdStmt(cornucopia_async::private::Stmt);
        impl GetPricelistIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetPricelistIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
            for GetPricelistIdStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetPricelistIdParams<T1, T2>,
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
        pub fn insert_pricelist() -> InsertPricelistStmt {
            InsertPricelistStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO pricelist (
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
        pub struct InsertPricelistStmt(cornucopia_async::private::Stmt);
        impl InsertPricelistStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertPricelistParams<T1, T2, T3>,
                I32Query<'a, C, i32, 5>,
                C,
            > for InsertPricelistStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertPricelistParams<T1, T2, T3>,
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
        pub fn update_pricelist() -> UpdatePricelistStmt {
            UpdatePricelistStmt(cornucopia_async::private::Stmt::new(
                "UPDATE pricelist
SET
    name = coalesce($1, name),
    slug = coalesce($2, slug),
    external_id = coalesce($3, external_id)
WHERE
    id = $4",
            ))
        }
        pub struct UpdatePricelistStmt(cornucopia_async::private::Stmt);
        impl UpdatePricelistStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdatePricelistParams<T1, T2, T3>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for UpdatePricelistStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpdatePricelistParams<T1, T2, T3>,
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
        pub fn delete_pricelist() -> DeletePricelistStmt {
            DeletePricelistStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM pricelist
WHERE
    organization_id = $1
    AND id = $2",
            ))
        }
        pub struct DeletePricelistStmt(cornucopia_async::private::Stmt);
        impl DeletePricelistStmt {
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
            cornucopia_async::Params<
                'a,
                DeletePricelistParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for DeletePricelistStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a DeletePricelistParams,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.organization_id, &params.id))
            }
        }
        pub fn allowed_pricelist_ids() -> AllowedPricelistIdsStmt {
            AllowedPricelistIdsStmt(cornucopia_async::private::Stmt::new(
                "SELECT DISTINCT group_pricelist.pricelist_id FROM group_pricelist
INNER JOIN group_user ON group_user.group_id = group_pricelist.group_id
INNER JOIN user_organization ON user_organization.user_id = group_user.user_id
WHERE
    user_organization.organization_id = $1
    AND group_user.user_id = $2",
            ))
        }
        pub struct AllowedPricelistIdsStmt(cornucopia_async::private::Stmt);
        impl AllowedPricelistIdsStmt {
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
            cornucopia_async::Params<'a, AllowedPricelistIdsParams, I32Query<'a, C, i32, 2>, C>
            for AllowedPricelistIdsStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a AllowedPricelistIdsParams,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(client, &params.organization_id, &params.user_id)
            }
        }
    }
    pub mod size {
        #[derive(Debug)]
        pub struct GetSizeIdParams<T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetSizeParams<T1: cornucopia_async::StringSql, T2: cornucopia_async::StringSql> {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertSizeParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::StringSql,
            T6: cornucopia_async::StringSql,
        > {
            pub color_id: i32,
            pub slug: T1,
            pub external_id: Option<T2>,
            pub number: T3,
            pub name: T4,
            pub service_item: Option<bool>,
            pub delivery_period: Option<time::Date>,
            pub ean_code: Option<T5>,
            pub status: Option<T6>,
            pub organization_id: i32,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdateSizeParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::StringSql,
            T6: cornucopia_async::StringSql,
        > {
            pub color_id: i32,
            pub slug: Option<T1>,
            pub external_id: Option<T2>,
            pub number: Option<T3>,
            pub position: Option<i16>,
            pub name: Option<T4>,
            pub service_item: Option<bool>,
            pub delivery_period: Option<time::Date>,
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub service_item: Option<bool>,
            pub delivery_period: Option<time::Date>,
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
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub service_item: Option<bool>,
            pub delivery_period: Option<time::Date>,
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
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct SizeRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn list_sizes() -> ListSizesStmt {
            ListSizesStmt(cornucopia_async::private::Stmt::new(
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
        pub struct ListSizesStmt(cornucopia_async::private::Stmt);
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
            GetSizeIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetSizeIdStmt(cornucopia_async::private::Stmt);
        impl GetSizeIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetSizeIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
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
            GetSizeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetSizeStmt(cornucopia_async::private::Stmt);
        impl GetSizeStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetSizeParams<T1, T2>, SizeRowQuery<'a, C, SizeRow, 4>, C>
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
            InsertSizeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertSizeStmt(cornucopia_async::private::Stmt);
        impl InsertSizeStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::StringSql,
                T6: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                color_id: &'a i32,
                slug: &'a T1,
                external_id: &'a Option<T2>,
                number: &'a T3,
                name: &'a T4,
                service_item: &'a Option<bool>,
                delivery_period: &'a Option<time::Date>,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::StringSql,
                T6: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
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
            UpdateSizeStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateSizeStmt(cornucopia_async::private::Stmt);
        impl UpdateSizeStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::StringSql,
                T6: cornucopia_async::StringSql,
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
                delivery_period: &'a Option<time::Date>,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::StringSql,
                T6: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateSizeParams<T1, T2, T3, T4, T5, T6>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteSizeStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM size
WHERE organization_id = $1
      AND id = $2",
            ))
        }
        pub struct DeleteSizeStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<
                'a,
                DeleteSizeParams,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
    }
    pub mod style {
        #[derive(Debug)]
        pub struct SelectStylesParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub organization_id: i32,
            pub ids: Option<T1>,
        }
        #[derive(Debug)]
        pub struct SelectCollectionStylesNestedParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::ArraySql<Item = T1>,
            T3: cornucopia_async::ArraySql<Item = i32>,
            T4: cornucopia_async::ArraySql<Item = i32>,
        > {
            pub collection_id: i32,
            pub organization_id: i32,
            pub statuses: Option<T2>,
            pub pricelist_ids_to_display: Option<T3>,
            pub ids: Option<T4>,
        }
        #[derive(Debug)]
        pub struct SelectNestedStyleSummariesParams<
            T1: cornucopia_async::ArraySql<Item = i32>,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::ArraySql<Item = T2>,
            T4: cornucopia_async::ArraySql<Item = i32>,
            T5: cornucopia_async::ArraySql<Item = i32>,
        > {
            pub attributes: Option<T1>,
            pub organization_id: i32,
            pub statuses: Option<T3>,
            pub ids: Option<T4>,
            pub categories: Option<T5>,
        }
        #[derive(Debug)]
        pub struct GetStyleIdParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct GetStyleRefsParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub id: Option<i32>,
            pub external_id: Option<T1>,
            pub slug: Option<T2>,
        }
        #[derive(Debug)]
        pub struct InsertStyleParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::JsonSql,
            T6: cornucopia_async::StringSql,
            T7: cornucopia_async::StringSql,
        > {
            pub organization_id: i32,
            pub slug: T1,
            pub external_id: Option<T2>,
            pub number: T3,
            pub name: T4,
            pub description: T5,
            pub core: Option<bool>,
            pub country_of_origin: Option<T6>,
            pub tariff_no: Option<T7>,
            pub net_weight: rust_decimal::Decimal,
            pub gross_weight: rust_decimal::Decimal,
            pub unit_volume: rust_decimal::Decimal,
            pub created_by: i32,
        }
        #[derive(Debug)]
        pub struct UpdateStyleParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::JsonSql,
            T6: cornucopia_async::StringSql,
            T7: cornucopia_async::StringSql,
        > {
            pub slug: Option<T1>,
            pub external_id: Option<T2>,
            pub number: Option<T3>,
            pub name: Option<T4>,
            pub description: Option<T5>,
            pub core: Option<bool>,
            pub country_of_origin: Option<T6>,
            pub tariff_no: Option<T7>,
            pub net_weight: Option<rust_decimal::Decimal>,
            pub gross_weight: Option<rust_decimal::Decimal>,
            pub unit_volume: Option<rust_decimal::Decimal>,
            pub id: i32,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct DeleteStyleParams {
            pub organization_id: i32,
            pub id: i32,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct StyleRow {
            pub id: i32,
            pub organization_id: i32,
            pub slug: String,
            pub external_id: Option<String>,
            pub number: String,
            pub name: serde_json::Value,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub description: serde_json::Value,
            pub core: Option<bool>,
            pub country_of_origin: Option<String>,
            pub tariff_no: Option<String>,
            pub unit_volume: rust_decimal::Decimal,
            pub gross_weight: rust_decimal::Decimal,
            pub net_weight: rust_decimal::Decimal,
            pub categories: serde_json::Value,
            pub attributes: serde_json::Value,
        }
        pub struct StyleRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub number: &'a str,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub description: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub core: Option<bool>,
            pub country_of_origin: Option<&'a str>,
            pub tariff_no: Option<&'a str>,
            pub unit_volume: rust_decimal::Decimal,
            pub gross_weight: rust_decimal::Decimal,
            pub net_weight: rust_decimal::Decimal,
            pub categories: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub attributes: postgres_types::Json<&'a serde_json::value::RawValue>,
        }
        impl<'a> From<StyleRowBorrowed<'a>> for StyleRow {
            fn from(
                StyleRowBorrowed {
                    id,
                    organization_id,
                    slug,
                    external_id,
                    number,
                    name,
                    created_by,
                    created_at,
                    updated_at,
                    description,
                    core,
                    country_of_origin,
                    tariff_no,
                    unit_volume,
                    gross_weight,
                    net_weight,
                    categories,
                    attributes,
                }: StyleRowBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    organization_id,
                    slug: slug.into(),
                    external_id: external_id.map(|v| v.into()),
                    number: number.into(),
                    name: serde_json::from_str(name.0.get()).unwrap(),
                    created_by,
                    created_at,
                    updated_at,
                    description: serde_json::from_str(description.0.get()).unwrap(),
                    core,
                    country_of_origin: country_of_origin.map(|v| v.into()),
                    tariff_no: tariff_no.map(|v| v.into()),
                    unit_volume,
                    gross_weight,
                    net_weight,
                    categories: serde_json::from_str(categories.0.get()).unwrap(),
                    attributes: serde_json::from_str(attributes.0.get()).unwrap(),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NestedStyleRow {
            pub id: i32,
            pub organization_id: i32,
            pub slug: String,
            pub external_id: Option<String>,
            pub number: String,
            pub name: serde_json::Value,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub description: serde_json::Value,
            pub core: Option<bool>,
            pub country_of_origin: Option<String>,
            pub tariff_no: Option<String>,
            pub unit_volume: rust_decimal::Decimal,
            pub gross_weight: rust_decimal::Decimal,
            pub net_weight: rust_decimal::Decimal,
            pub is_new: Option<bool>,
            pub colors: serde_json::Value,
            pub prices: serde_json::Value,
            pub attributes: serde_json::Value,
            pub categories: serde_json::Value,
        }
        pub struct NestedStyleRowBorrowed<'a> {
            pub id: i32,
            pub organization_id: i32,
            pub slug: &'a str,
            pub external_id: Option<&'a str>,
            pub number: &'a str,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub created_by: Option<i32>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub description: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub core: Option<bool>,
            pub country_of_origin: Option<&'a str>,
            pub tariff_no: Option<&'a str>,
            pub unit_volume: rust_decimal::Decimal,
            pub gross_weight: rust_decimal::Decimal,
            pub net_weight: rust_decimal::Decimal,
            pub is_new: Option<bool>,
            pub colors: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub prices: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub attributes: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub categories: postgres_types::Json<&'a serde_json::value::RawValue>,
        }
        impl<'a> From<NestedStyleRowBorrowed<'a>> for NestedStyleRow {
            fn from(
                NestedStyleRowBorrowed {
                    id,
                    organization_id,
                    slug,
                    external_id,
                    number,
                    name,
                    created_by,
                    created_at,
                    updated_at,
                    description,
                    core,
                    country_of_origin,
                    tariff_no,
                    unit_volume,
                    gross_weight,
                    net_weight,
                    is_new,
                    colors,
                    prices,
                    attributes,
                    categories,
                }: NestedStyleRowBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    organization_id,
                    slug: slug.into(),
                    external_id: external_id.map(|v| v.into()),
                    number: number.into(),
                    name: serde_json::from_str(name.0.get()).unwrap(),
                    created_by,
                    created_at,
                    updated_at,
                    description: serde_json::from_str(description.0.get()).unwrap(),
                    core,
                    country_of_origin: country_of_origin.map(|v| v.into()),
                    tariff_no: tariff_no.map(|v| v.into()),
                    unit_volume,
                    gross_weight,
                    net_weight,
                    is_new,
                    colors: serde_json::from_str(colors.0.get()).unwrap(),
                    prices: serde_json::from_str(prices.0.get()).unwrap(),
                    attributes: serde_json::from_str(attributes.0.get()).unwrap(),
                    categories: serde_json::from_str(categories.0.get()).unwrap(),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NestedStyleSummaryRow {
            pub id: i32,
            pub name: serde_json::Value,
            pub number: String,
            pub colors: serde_json::Value,
        }
        pub struct NestedStyleSummaryRowBorrowed<'a> {
            pub id: i32,
            pub name: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub number: &'a str,
            pub colors: postgres_types::Json<&'a serde_json::value::RawValue>,
        }
        impl<'a> From<NestedStyleSummaryRowBorrowed<'a>> for NestedStyleSummaryRow {
            fn from(
                NestedStyleSummaryRowBorrowed {
                    id,
                    name,
                    number,
                    colors,
                }: NestedStyleSummaryRowBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: serde_json::from_str(name.0.get()).unwrap(),
                    number: number.into(),
                    colors: serde_json::from_str(colors.0.get()).unwrap(),
                }
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct StyleRefs {
            pub id: i32,
            pub external_id: Option<String>,
            pub slug: String,
        }
        pub struct StyleRefsBorrowed<'a> {
            pub id: i32,
            pub external_id: Option<&'a str>,
            pub slug: &'a str,
        }
        impl<'a> From<StyleRefsBorrowed<'a>> for StyleRefs {
            fn from(
                StyleRefsBorrowed {
                    id,
                    external_id,
                    slug,
                }: StyleRefsBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    external_id: external_id.map(|v| v.into()),
                    slug: slug.into(),
                }
            }
        }
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct StyleRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> StyleRowBorrowed,
            mapper: fn(StyleRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StyleRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(StyleRowBorrowed) -> R) -> StyleRowQuery<'a, C, R, N> {
                StyleRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct NestedStyleRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> NestedStyleRowBorrowed,
            mapper: fn(NestedStyleRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> NestedStyleRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(NestedStyleRowBorrowed) -> R,
            ) -> NestedStyleRowQuery<'a, C, R, N> {
                NestedStyleRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct NestedStyleSummaryRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> NestedStyleSummaryRowBorrowed,
            mapper: fn(NestedStyleSummaryRowBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> NestedStyleSummaryRowQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(NestedStyleSummaryRowBorrowed) -> R,
            ) -> NestedStyleSummaryRowQuery<'a, C, R, N> {
                NestedStyleSummaryRowQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct StyleRefsQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> StyleRefsBorrowed,
            mapper: fn(StyleRefsBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StyleRefsQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(StyleRefsBorrowed) -> R) -> StyleRefsQuery<'a, C, R, N> {
                StyleRefsQuery {
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_styles() -> SelectStylesStmt {
            SelectStylesStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    style.*,
    coalesce(joined_categories.json_data, '[]') AS \"categories\",
    coalesce(joined_attributes.json_data, '[]') AS \"attributes\"
FROM
    style
LEFT JOIN (
    SELECT
        style_attribute.style_id,
        json_agg(
            json_build_object(
                'id',
                \"attribute\".id,
                'title',
                \"attribute\".title,
                'description',
                \"attribute\".description,
                'slug',
                \"attribute\".slug,
                'external_id',
                \"attribute\".external_id,
                'type',
                json_build_object(
                    'id',
                    attributetype.id,
                    'name',
                    attributetype.name,
                    'slug',
                    attributetype.slug,
                    'external_id',
                    attributetype.external_id
                )
            )
            ORDER BY
                attributetype.name,
                \"attribute\".title
        ) FILTER (
            WHERE
            \"attribute\".id IS NOT NULL
        ) AS json_data
    FROM
        attribute
    INNER JOIN attributetype ON attributetype.id = \"attribute\".type_id
    INNER JOIN style_attribute ON \"attribute\".id = style_attribute.attribute_id
    WHERE
        \"attribute\".organization_id = $1
    GROUP BY
        style_attribute.style_id
    ) AS joined_attributes ON joined_attributes.style_id = style.id
LEFT JOIN (
    SELECT
        style_category.style_id,
        json_agg(category.*) FILTER (
            WHERE
            category.id IS NOT NULL
        ) AS \"json_data\"
    FROM
        category
    INNER JOIN style_category ON category.id = style_category.category_id
    WHERE
        category.organization_id = $1
    GROUP BY
        style_category.style_id
    ) AS joined_categories ON joined_categories.style_id = style.id
WHERE
    style.organization_id = $1
    AND $2::int[] IS NULL OR style.id = any($2)
ORDER BY
    style.number",
            ))
        }
        pub struct SelectStylesStmt(cornucopia_async::private::Stmt);
        impl SelectStylesStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
                ids: &'a Option<T1>,
            ) -> StyleRowQuery<'a, C, StyleRow, 2> {
                StyleRowQuery {
                    client,
                    params: [organization_id, ids],
                    stmt: &mut self.0,
                    extractor: |row| StyleRowBorrowed {
                        id: row.get(0),
                        organization_id: row.get(1),
                        slug: row.get(2),
                        external_id: row.get(3),
                        number: row.get(4),
                        name: row.get(5),
                        created_by: row.get(6),
                        created_at: row.get(7),
                        updated_at: row.get(8),
                        description: row.get(9),
                        core: row.get(10),
                        country_of_origin: row.get(11),
                        tariff_no: row.get(12),
                        unit_volume: row.get(13),
                        gross_weight: row.get(14),
                        net_weight: row.get(15),
                        categories: row.get(16),
                        attributes: row.get(17),
                    },
                    mapper: |it| <StyleRow>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                SelectStylesParams<T1>,
                StyleRowQuery<'a, C, StyleRow, 2>,
                C,
            > for SelectStylesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SelectStylesParams<T1>,
            ) -> StyleRowQuery<'a, C, StyleRow, 2> {
                self.bind(client, &params.organization_id, &params.ids)
            }
        }
        pub fn select_collection_styles_nested() -> SelectCollectionStylesNestedStmt {
            SelectCollectionStylesNestedStmt(cornucopia_async::private::Stmt::new(
                "WITH new_styles AS (
    SELECT
        new_collection_style.style_id,
        new_collection_style.is_new
    FROM
        new_collection_style
    WHERE
        new_collection_style.collection_id = $1
),

new_colors AS (
    SELECT
        new_collection_color.color_id,
        new_collection_color.is_new
    FROM
        new_collection_color
    WHERE
        new_collection_color.collection_id = $1
)

SELECT
    style.*,
    new_styles.is_new,
    coalesce(joined_colors.json_data, '[]') AS \"colors\",
    coalesce(joined_prices.json_data, '[]') AS \"prices\",
    coalesce(joined_attributes.json_data, '[]') AS \"attributes\",
    coalesce(joined_categories.json_data, '[]') AS \"categories\"
FROM
    style
LEFT JOIN new_styles ON new_styles.style_id = style.id
INNER JOIN (
        SELECT
            color.style_id,
            json_agg(
                json_build_object(
                    'id',
                    color.id,
                    'number',
                    color.number,
                    'name',
                    color.name,
                    'slug',
                    color.slug,
                    'external_id',
                    color.external_id,
                    'sizes', coalesce(joined_sizes.json_data, '[]'),
                    'images', coalesce(joined_images.json_data, '[]'),
                    'is_new',
                    new_colors.is_new
                )
                ORDER BY
                    color.number
            ) FILTER (
                WHERE
                color.id IS NOT NULL
            ) AS json_data
        FROM
            color
        LEFT JOIN new_colors ON new_colors.color_id = color.id
        INNER JOIN (
            SELECT
                size.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        size.id,
                        'number',
                        size.number,
                        'position',
                        size.position,
                        'name',
                        size.name,
                        'service_item',
                        size.service_item,
                        'delivery_period',
                        size.delivery_period,
                        'ean_code',
                        size.ean_code,
                        'status',
                        size.status,
                        'slug',
                        size.slug,
                        'external_id',
                        size.external_id
                    )
                    ORDER BY
                        size.number
                ) FILTER (
                    WHERE
                    size.id IS NOT NULL
                ) AS json_data
            FROM
                size
            INNER JOIN color ON size.color_id = color.id
            INNER JOIN style ON color.style_id = style.id
            INNER JOIN size_collection ON size_collection.size_id = size.id
            WHERE
                size.organization_id = $2
                AND size_collection.collection_id = $1
                AND (
                    $3::text[] IS NULL OR size.status = any($3)
                )
            GROUP BY
                size.color_id
            ) AS joined_sizes ON joined_sizes.color_id = color.id
        LEFT JOIN (
            SELECT
                image.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        image.id,
                        'url',
                        image.url,
                        'external_id',
                        image.external_id
                    )
                    ORDER BY
                        image.position
                ) AS json_data
            FROM
                image
            WHERE
                image.organization_id = $2
            GROUP BY
                image.color_id
            ) AS joined_images ON joined_images.color_id = color.id
        WHERE
            color.organization_id = $2
        GROUP BY
            color.style_id
    ) AS joined_colors ON joined_colors.style_id = style.id
LEFT JOIN (
        SELECT
            price.style_id,
            json_agg(
                json_build_object(
                    'id',
                    price.id,
                    'type',
                    price.type,
                    'uom',
                    price.uom,
                    'currency',
                    price.currency,
                    'amount',
                    price.amount,
                    'start',
                    price.start,
                    'end',
                    price.end,
                    'list',
                    json_build_object(
                        'id',
                        pricelist.id,
                        'slug',
                        pricelist.slug,
                        'external_id',
                        pricelist.external_id,
                        'name',
                        pricelist.name
                    )
                )
                ORDER BY
                    price.type,
                    pricelist.name,
                    price.\"start\"
            ) FILTER (
                WHERE
                price.id IS NOT NULL
            ) AS json_data
        FROM
            price
        INNER JOIN pricelist ON pricelist.id = price.list_id
        INNER JOIN collection_pricelist ON
            collection_pricelist.pricelist_id = pricelist.id
        WHERE
            price.organization_id = $2
            AND (
                $4::int[] IS NULL
                OR pricelist.id = any($4)
            )
            AND collection_pricelist.collection_id = $1
            AND (
                collection_pricelist.price_date
                BETWEEN price.\"start\" AND price.\"end\"
            )
        GROUP BY
            price.style_id
    ) AS joined_prices ON joined_prices.style_id = style.id
LEFT JOIN (
        SELECT
            style_attribute.style_id,
            json_agg(
                json_build_object(
                    'id',
                    \"attribute\".id,
                    'title',
                    \"attribute\".title,
                    'description',
                    \"attribute\".description,
                    'slug',
                    \"attribute\".slug,
                    'external_id',
                    \"attribute\".external_id,
                    'type',
                    json_build_object(
                        'id',
                        attributetype.id,
                        'name',
                        attributetype.name,
                        'slug',
                        attributetype.slug,
                        'external_id',
                        attributetype.external_id
                    )
                )
                ORDER BY
                    attributetype.name,
                    \"attribute\".title
            ) FILTER (
                WHERE
                \"attribute\".id IS NOT NULL
            ) AS json_data
        FROM
            attribute
        INNER JOIN attributetype ON attributetype.id = \"attribute\".type_id
        INNER JOIN style_attribute ON \"attribute\".id = style_attribute.attribute_id
        WHERE
            \"attribute\".organization_id = $2
        GROUP BY
            style_attribute.style_id
    ) AS joined_attributes ON joined_attributes.style_id = style.id
LEFT JOIN (
        SELECT
            style_category.style_id,
            json_agg(
                json_build_object(
                    'id',
                    category.id,
                    'slug',
                    category.slug,
                    'name',
                    category.name,
                    'external_id',
                    category.external_id
                )
            ) FILTER (
                WHERE
                category.id IS NOT NULL
            ) AS \"json_data\"
        FROM
            category
        INNER JOIN style_category ON category.id = style_category.category_id
        WHERE
            category.organization_id = $2
        GROUP BY
            style_category.style_id
    ) AS joined_categories ON joined_categories.style_id = style.id
WHERE
    ($5::int[] IS NULL OR style.id = any($5))
ORDER BY
    style.number",
            ))
        }
        pub struct SelectCollectionStylesNestedStmt(cornucopia_async::private::Stmt);
        impl SelectCollectionStylesNestedStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
                T3: cornucopia_async::ArraySql<Item = i32>,
                T4: cornucopia_async::ArraySql<Item = i32>,
            >(
                &'a mut self,
                client: &'a C,
                collection_id: &'a i32,
                organization_id: &'a i32,
                statuses: &'a Option<T2>,
                pricelist_ids_to_display: &'a Option<T3>,
                ids: &'a Option<T4>,
            ) -> NestedStyleRowQuery<'a, C, NestedStyleRow, 5> {
                NestedStyleRowQuery {
                    client,
                    params: [
                        collection_id,
                        organization_id,
                        statuses,
                        pricelist_ids_to_display,
                        ids,
                    ],
                    stmt: &mut self.0,
                    extractor: |row| NestedStyleRowBorrowed {
                        id: row.get(0),
                        organization_id: row.get(1),
                        slug: row.get(2),
                        external_id: row.get(3),
                        number: row.get(4),
                        name: row.get(5),
                        created_by: row.get(6),
                        created_at: row.get(7),
                        updated_at: row.get(8),
                        description: row.get(9),
                        core: row.get(10),
                        country_of_origin: row.get(11),
                        tariff_no: row.get(12),
                        unit_volume: row.get(13),
                        gross_weight: row.get(14),
                        net_weight: row.get(15),
                        is_new: row.get(16),
                        colors: row.get(17),
                        prices: row.get(18),
                        attributes: row.get(19),
                        categories: row.get(20),
                    },
                    mapper: |it| <NestedStyleRow>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
                T3: cornucopia_async::ArraySql<Item = i32>,
                T4: cornucopia_async::ArraySql<Item = i32>,
            >
            cornucopia_async::Params<
                'a,
                SelectCollectionStylesNestedParams<T1, T2, T3, T4>,
                NestedStyleRowQuery<'a, C, NestedStyleRow, 5>,
                C,
            > for SelectCollectionStylesNestedStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SelectCollectionStylesNestedParams<T1, T2, T3, T4>,
            ) -> NestedStyleRowQuery<'a, C, NestedStyleRow, 5> {
                self.bind(
                    client,
                    &params.collection_id,
                    &params.organization_id,
                    &params.statuses,
                    &params.pricelist_ids_to_display,
                    &params.ids,
                )
            }
        }
        pub fn select_nested_style_summaries() -> SelectNestedStyleSummariesStmt {
            SelectNestedStyleSummariesStmt(cornucopia_async::private::Stmt::new(
                "WITH attribute_matches AS (
    SELECT styles_matching_attributes($1)
)

SELECT
    style.id,
    style.name,
    style.number,
    coalesce(joined_colors.json_data, '[]') AS \"colors\"
FROM
    style
INNER JOIN (
        SELECT
            color.style_id,
            json_agg(
                json_build_object(
                    'id',
                    color.id,
                    'number',
                    color.number,
                    'name',
                    color.name,
                    'sizes', coalesce(joined_sizes.json_data, '[]'),
                    'primary_image', primary_image.json_data
                )
                ORDER BY
                    color.number
            ) FILTER (
                WHERE
                color.id IS NOT NULL
            ) AS json_data
        FROM
            color
        INNER JOIN (
            SELECT
                size.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        size.id,
                        'number',
                        size.number,
                        'name',
                        size.name
                    )
                    ORDER BY
                        size.number
                ) FILTER (
                    WHERE
                    size.id IS NOT NULL
                ) AS json_data
            FROM
                size
            INNER JOIN color ON size.color_id = color.id
            WHERE
                size.organization_id = $2
                AND ($3::text[] IS NULL OR size.status = any($3))
            GROUP BY
                size.color_id
            ) AS joined_sizes ON joined_sizes.color_id = color.id
        LEFT JOIN (
            -- We utilize distinct here because there might be multiple rows with
            -- `position = 1` for a given `color_id`.
            SELECT DISTINCT ON (image.color_id)
                image.color_id,
                json_build_object(
                    'id',
                    image.id,
                    'url',
                    image.url,
                    'external_id',
                    image.external_id
                ) AS json_data
            FROM
                image
            WHERE
                image.organization_id = $2
                AND image.position = 1
            ) AS primary_image ON primary_image.color_id = color.id
        WHERE
            color.organization_id = $2
        GROUP BY
            color.style_id
    ) AS joined_colors ON joined_colors.style_id = style.id
LEFT JOIN (
    SELECT
        style_category.style_id,
        array_agg(style_category.category_id) AS category_ids
    FROM style_category
    GROUP BY style_category.style_id
) AS style_categories ON style_categories.style_id = style.id
WHERE
    style.organization_id = $2
    AND ($4::int[] IS NULL OR style.id = any($4))
    AND ($5::int[] IS NULL OR style_categories.category_ids && $5)
    AND ($1::int[] IS NULL OR style.id IN (SELECT * FROM attribute_matches))
ORDER BY
    style.number",
            ))
        }
        pub struct SelectNestedStyleSummariesStmt(cornucopia_async::private::Stmt);
        impl SelectNestedStyleSummariesStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::ArraySql<Item = i32>,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::ArraySql<Item = T2>,
                T4: cornucopia_async::ArraySql<Item = i32>,
                T5: cornucopia_async::ArraySql<Item = i32>,
            >(
                &'a mut self,
                client: &'a C,
                attributes: &'a Option<T1>,
                organization_id: &'a i32,
                statuses: &'a Option<T3>,
                ids: &'a Option<T4>,
                categories: &'a Option<T5>,
            ) -> NestedStyleSummaryRowQuery<'a, C, NestedStyleSummaryRow, 5> {
                NestedStyleSummaryRowQuery {
                    client,
                    params: [attributes, organization_id, statuses, ids, categories],
                    stmt: &mut self.0,
                    extractor: |row| NestedStyleSummaryRowBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        number: row.get(2),
                        colors: row.get(3),
                    },
                    mapper: |it| <NestedStyleSummaryRow>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::ArraySql<Item = i32>,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::ArraySql<Item = T2>,
                T4: cornucopia_async::ArraySql<Item = i32>,
                T5: cornucopia_async::ArraySql<Item = i32>,
            >
            cornucopia_async::Params<
                'a,
                SelectNestedStyleSummariesParams<T1, T2, T3, T4, T5>,
                NestedStyleSummaryRowQuery<'a, C, NestedStyleSummaryRow, 5>,
                C,
            > for SelectNestedStyleSummariesStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a SelectNestedStyleSummariesParams<T1, T2, T3, T4, T5>,
            ) -> NestedStyleSummaryRowQuery<'a, C, NestedStyleSummaryRow, 5> {
                self.bind(
                    client,
                    &params.attributes,
                    &params.organization_id,
                    &params.statuses,
                    &params.ids,
                    &params.categories,
                )
            }
        }
        pub fn get_style_id() -> GetStyleIdStmt {
            GetStyleIdStmt(cornucopia_async::private::Stmt::new(
                "SELECT style.id
FROM
    style
WHERE
    style.organization_id = $1
    AND (
        (style.id = coalesce($2, -1))
        OR (
            style.external_id = coalesce($3, '___NON_EXISTING___')
            OR (style.slug = coalesce($4, '___NON_EXISTING___'))
        )
    )",
            ))
        }
        pub struct GetStyleIdStmt(cornucopia_async::private::Stmt);
        impl GetStyleIdStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
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
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<'a, GetStyleIdParams<T1, T2>, I32Query<'a, C, i32, 4>, C>
            for GetStyleIdStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetStyleIdParams<T1, T2>,
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
        pub fn get_style_refs() -> GetStyleRefsStmt {
            GetStyleRefsStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    style.id,
    style.external_id,
    style.slug
FROM
    style
WHERE
    style.organization_id = $1
    AND (
        (style.id = coalesce($2, -1))
        OR (
            style.external_id = coalesce($3, '___NON_EXISTING___')
            OR (style.slug = coalesce($4, '___NON_EXISTING___'))
        )
    )",
            ))
        }
        pub struct GetStyleRefsStmt(cornucopia_async::private::Stmt);
        impl GetStyleRefsStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
                id: &'a Option<i32>,
                external_id: &'a Option<T1>,
                slug: &'a Option<T2>,
            ) -> StyleRefsQuery<'a, C, StyleRefs, 4> {
                StyleRefsQuery {
                    client,
                    params: [organization_id, id, external_id, slug],
                    stmt: &mut self.0,
                    extractor: |row| StyleRefsBorrowed {
                        id: row.get(0),
                        external_id: row.get(1),
                        slug: row.get(2),
                    },
                    mapper: |it| <StyleRefs>::from(it),
                }
            }
        }
        impl<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                GetStyleRefsParams<T1, T2>,
                StyleRefsQuery<'a, C, StyleRefs, 4>,
                C,
            > for GetStyleRefsStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetStyleRefsParams<T1, T2>,
            ) -> StyleRefsQuery<'a, C, StyleRefs, 4> {
                self.bind(
                    client,
                    &params.organization_id,
                    &params.id,
                    &params.external_id,
                    &params.slug,
                )
            }
        }
        pub fn insert_style() -> InsertStyleStmt {
            InsertStyleStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO
style (
    organization_id,
    slug,
    external_id,
    number,
    name,
    description,
    core,
    country_of_origin,
    tariff_no,
    net_weight,
    gross_weight,
    unit_volume,
    created_by
)
VALUES
(
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
    $11,
    $12,
    $13
)
RETURNING
id",
            ))
        }
        pub struct InsertStyleStmt(cornucopia_async::private::Stmt);
        impl InsertStyleStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::JsonSql,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
                slug: &'a T1,
                external_id: &'a Option<T2>,
                number: &'a T3,
                name: &'a T4,
                description: &'a T5,
                core: &'a Option<bool>,
                country_of_origin: &'a Option<T6>,
                tariff_no: &'a Option<T7>,
                net_weight: &'a rust_decimal::Decimal,
                gross_weight: &'a rust_decimal::Decimal,
                unit_volume: &'a rust_decimal::Decimal,
                created_by: &'a i32,
            ) -> I32Query<'a, C, i32, 13> {
                I32Query {
                    client,
                    params: [
                        organization_id,
                        slug,
                        external_id,
                        number,
                        name,
                        description,
                        core,
                        country_of_origin,
                        tariff_no,
                        net_weight,
                        gross_weight,
                        unit_volume,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::JsonSql,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertStyleParams<T1, T2, T3, T4, T5, T6, T7>,
                I32Query<'a, C, i32, 13>,
                C,
            > for InsertStyleStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertStyleParams<T1, T2, T3, T4, T5, T6, T7>,
            ) -> I32Query<'a, C, i32, 13> {
                self.bind(
                    client,
                    &params.organization_id,
                    &params.slug,
                    &params.external_id,
                    &params.number,
                    &params.name,
                    &params.description,
                    &params.core,
                    &params.country_of_origin,
                    &params.tariff_no,
                    &params.net_weight,
                    &params.gross_weight,
                    &params.unit_volume,
                    &params.created_by,
                )
            }
        }
        pub fn update_style() -> UpdateStyleStmt {
            UpdateStyleStmt(cornucopia_async::private::Stmt::new(
                "UPDATE
style
SET
    slug = coalesce($1, slug),
    external_id = coalesce($2, external_id),
    number = coalesce($3, number),
    name = coalesce($4, name),
    description = coalesce($5, description),
    core = coalesce($6, core),
    country_of_origin = coalesce($7, country_of_origin),
    tariff_no = coalesce($8, tariff_no),
    net_weight = coalesce($9, net_weight),
    gross_weight = coalesce($10, gross_weight),
    unit_volume = coalesce($11, unit_volume)
WHERE
    id = $12
RETURNING
id",
            ))
        }
        pub struct UpdateStyleStmt(cornucopia_async::private::Stmt);
        impl UpdateStyleStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::JsonSql,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                slug: &'a Option<T1>,
                external_id: &'a Option<T2>,
                number: &'a Option<T3>,
                name: &'a Option<T4>,
                description: &'a Option<T5>,
                core: &'a Option<bool>,
                country_of_origin: &'a Option<T6>,
                tariff_no: &'a Option<T7>,
                net_weight: &'a Option<rust_decimal::Decimal>,
                gross_weight: &'a Option<rust_decimal::Decimal>,
                unit_volume: &'a Option<rust_decimal::Decimal>,
                id: &'a i32,
            ) -> I32Query<'a, C, i32, 12> {
                I32Query {
                    client,
                    params: [
                        slug,
                        external_id,
                        number,
                        name,
                        description,
                        core,
                        country_of_origin,
                        tariff_no,
                        net_weight,
                        gross_weight,
                        unit_volume,
                        id,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::JsonSql,
                T5: cornucopia_async::JsonSql,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateStyleParams<T1, T2, T3, T4, T5, T6, T7>,
                I32Query<'a, C, i32, 12>,
                C,
            > for UpdateStyleStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpdateStyleParams<T1, T2, T3, T4, T5, T6, T7>,
            ) -> I32Query<'a, C, i32, 12> {
                self.bind(
                    client,
                    &params.slug,
                    &params.external_id,
                    &params.number,
                    &params.name,
                    &params.description,
                    &params.core,
                    &params.country_of_origin,
                    &params.tariff_no,
                    &params.net_weight,
                    &params.gross_weight,
                    &params.unit_volume,
                    &params.id,
                )
            }
        }
        pub fn delete_style() -> DeleteStyleStmt {
            DeleteStyleStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM
style
WHERE
    organization_id = $1
    AND id = $2
RETURNING
id",
            ))
        }
        pub struct DeleteStyleStmt(cornucopia_async::private::Stmt);
        impl DeleteStyleStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                organization_id: &'a i32,
                id: &'a i32,
            ) -> I32Query<'a, C, i32, 2> {
                I32Query {
                    client,
                    params: [organization_id, id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<'a, DeleteStyleParams, I32Query<'a, C, i32, 2>, C>
            for DeleteStyleStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a DeleteStyleParams,
            ) -> I32Query<'a, C, i32, 2> {
                self.bind(client, &params.organization_id, &params.id)
            }
        }
    }
    pub mod user {
        #[derive(Debug)]
        pub struct SelectUsersParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::ArraySql<Item = i32>,
            T3: cornucopia_async::ArraySql<Item = i32>,
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
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
        > {
            pub name: T1,
            pub email: T2,
            pub password_hash: Option<T3>,
            pub profile_image: Option<T4>,
        }
        #[derive(Debug)]
        pub struct UpdateUserParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
        > {
            pub name: Option<T1>,
            pub email: Option<T2>,
            pub password_hash: Option<T3>,
            pub profile_image: Option<T4>,
            pub id: i32,
        }
        #[derive(Debug)]
        pub struct UpsertUserOrganizationParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub user_id: i32,
            pub organization_id: i32,
            pub role_ids: Option<T1>,
        }
        #[derive(Debug)]
        pub struct ReplaceUserGroupsParams<T1: cornucopia_async::ArraySql<Item = i32>> {
            pub user_id: i32,
            pub group_ids: T1,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct UserRow {
            pub id: i32,
            pub name: String,
            pub email: String,
            pub password_hash: Option<String>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub profile_image: Option<String>,
            pub last_sign_in: Option<time::OffsetDateTime>,
            pub organizations: serde_json::Value,
            pub groups: serde_json::Value,
        }
        pub struct UserRowBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub email: &'a str,
            pub password_hash: Option<&'a str>,
            pub created_at: time::OffsetDateTime,
            pub updated_at: time::OffsetDateTime,
            pub profile_image: Option<&'a str>,
            pub last_sign_in: Option<time::OffsetDateTime>,
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
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        pub struct UserRowQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct I32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct Veci32Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> cornucopia_async::ArrayIterator<'_, i32>,
            mapper: fn(cornucopia_async::ArrayIterator<'_, i32>) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> Veci32Query<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(cornucopia_async::ArrayIterator<'_, i32>) -> R,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
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
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_users() -> SelectUsersStmt {
            SelectUsersStmt(cornucopia_async::private::Stmt::new(
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
        pub struct SelectUsersStmt(cornucopia_async::private::Stmt);
        impl SelectUsersStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = i32>,
                T3: cornucopia_async::ArraySql<Item = i32>,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = i32>,
                T3: cornucopia_async::ArraySql<Item = i32>,
            >
            cornucopia_async::Params<
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
            GetOrgUserIdStmt(cornucopia_async::private::Stmt::new(
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
        pub struct GetOrgUserIdStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<'a, GetOrgUserIdParams, I32Query<'a, C, i32, 2>, C>
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
            GetRoleIdsStmt(cornucopia_async::private::Stmt::new(
                "SELECT user_organization.role_ids
FROM
    user_organization
WHERE
    user_organization.user_id = $1
    AND user_organization.organization_id = $2",
            ))
        }
        pub struct GetRoleIdsStmt(cornucopia_async::private::Stmt);
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
            cornucopia_async::Params<'a, GetRoleIdsParams, Veci32Query<'a, C, Vec<i32>, 2>, C>
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
            InsertUserStmt(cornucopia_async::private::Stmt::new(
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
        pub struct InsertUserStmt(cornucopia_async::private::Stmt);
        impl InsertUserStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertUserParams<T1, T2, T3, T4>,
                I32Query<'a, C, i32, 4>,
                C,
            > for InsertUserStmt
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
            UpdateUserStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpdateUserStmt(cornucopia_async::private::Stmt);
        impl UpdateUserStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
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
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpdateUserParams<T1, T2, T3, T4>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            DeleteUserStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM \"user\"
WHERE id = $1",
            ))
        }
        pub struct DeleteUserStmt(cornucopia_async::private::Stmt);
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
            UpsertUserOrganizationStmt(cornucopia_async::private::Stmt::new(
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
        pub struct UpsertUserOrganizationStmt(cornucopia_async::private::Stmt);
        impl UpsertUserOrganizationStmt {
            pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient + Send + Sync, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<
                'a,
                UpsertUserOrganizationParams<T1>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
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
            GetUserPasswordHashStmt(cornucopia_async::private::Stmt::new(
                "SELECT \"user\".password_hash
FROM
    \"user\"
WHERE
    \"user\".id = $1",
            ))
        }
        pub struct GetUserPasswordHashStmt(cornucopia_async::private::Stmt);
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
            UpdateLastSignInStmt(cornucopia_async::private::Stmt::new(
                "UPDATE \"user\" SET last_sign_in = now() WHERE id = $1",
            ))
        }
        pub struct UpdateLastSignInStmt(cornucopia_async::private::Stmt);
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
            ReplaceUserGroupsStmt(cornucopia_async::private::Stmt::new(
                "SELECT *
FROM
    replace_user_groups($1, $2)",
            ))
        }
        pub struct ReplaceUserGroupsStmt(cornucopia_async::private::Stmt);
        impl ReplaceUserGroupsStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>(
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
        impl<'a, C: GenericClient, T1: cornucopia_async::ArraySql<Item = i32>>
            cornucopia_async::Params<'a, ReplaceUserGroupsParams<T1>, I32Query<'a, C, i32, 2>, C>
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
    }
}
