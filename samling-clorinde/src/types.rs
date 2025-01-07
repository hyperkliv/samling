// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
#[postgres(name = "collection_pricelist_relation")]
pub struct CollectionPricelistRelation {
    #[postgres(name = "pricelist_id")]
    pub pricelist_id: i32,
    #[postgres(name = "price_date")]
    pub price_date: chrono::NaiveDate,
    #[postgres(name = "created_by")]
    pub created_by: i32,
}
impl<'a> postgres_types::ToSql for CollectionPricelistRelation {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
                "pricelist_id" => postgres_types::ToSql::to_sql(pricelist_id, field.type_(), out),
                "price_date" => postgres_types::ToSql::to_sql(price_date, field.type_(), out),
                "created_by" => postgres_types::ToSql::to_sql(created_by, field.type_(), out),
                _ => unreachable!(),
            };
            let count = match r? {
                postgres_types::IsNull::Yes => -1,
                postgres_types::IsNull::No => {
                    let len = out.len() - base - 4;
                    if len > i32::MAX as usize {
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
                        <chrono::NaiveDate as postgres_types::ToSql>::accepts(f.type_())
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
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
