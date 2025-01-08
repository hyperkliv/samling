use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    forward_to_deserialize_any,
};

use crate::deadpool_postgres::{self, Pool, Timeouts};
use crate::Result;

pub fn create_deadpool_manager(
    db_host: String,
    db_port: u16,
    db_name: String,
    db_user: String,
    db_password: Option<String>,
    max_db_connections: u32,
) -> Result<Pool> {
    let mut cfg = deadpool_postgres::Config::new();
    cfg.user = Some(db_user);
    cfg.password = db_password;
    cfg.host = Some(db_host);
    cfg.port = Some(db_port);
    cfg.dbname = Some(db_name);
    let builder = cfg
        .builder(crate::tokio_postgres::NoTls)
        .unwrap()
        .max_size(max_db_connections as usize) // TODO: Do these really correspond?
        .timeouts(Timeouts::wait_millis(30_000))
        .runtime(deadpool_postgres::Runtime::Tokio1);
    let pool = builder.build()?;
    Ok(pool)
}

pub fn slugify(parts: &[&str]) -> String {
    ::slug::slugify(parts.join(" "))
}

/// Returns the type's name, without module
pub(crate) fn short_type_name<T>() -> &'static str {
    std::any::type_name::<T>().rsplit_once(':').unwrap().1
}

/// Get the fields of a serde deserializable struct
pub(crate) fn struct_fields<'de, T>() -> &'static [&'static str]
where
    T: Deserialize<'de>,
{
    struct StructFieldsDeserializer<'a> {
        fields: &'a mut Option<&'static [&'static str]>,
    }

    impl<'de> Deserializer<'de> for StructFieldsDeserializer<'_> {
        type Error = serde::de::value::Error;

        fn deserialize_any<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            Err(de::Error::custom("I'm just here for the fields"))
        }

        fn deserialize_struct<V>(
            self,
            _name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> std::result::Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            *self.fields = Some(fields);
            self.deserialize_any(visitor)
        }

        forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map enum identifier ignored_any
        }
    }

    let mut fields = None;
    let _ = T::deserialize(StructFieldsDeserializer {
        fields: &mut fields,
    });
    fields.unwrap()
}
