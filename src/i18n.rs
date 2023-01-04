use cornucopia_async::JsonSql;
use postgres_types::{private::BytesMut, to_sql_checked, IsNull, ToSql, Type};
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    En,
    Sv,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct I18nString {
    #[serde(default)]
    pub en: String,
    #[serde(default)]
    pub sv: String,
}

impl From<String> for I18nString {
    fn from(en: String) -> Self {
        Self::new_en(en)
    }
}

impl I18nString {
    pub fn new_en(en: String) -> Self {
        Self {
            en,
            ..Self::default()
        }
    }

    pub fn get<L: Into<Language>>(&self, language: L) -> String {
        match language.into() {
            Language::En => self.en.to_owned(),
            Language::Sv => {
                if self.sv.is_empty() {
                    self.en.to_owned()
                } else {
                    self.sv.to_owned()
                }
            }
        }
    }
}

impl ToSql for I18nString {
    fn to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        <serde_json::Value as ToSql>::to_sql(&serde_json::to_value(self)?, ty, w)
    }

    fn accepts(ty: &Type) -> bool {
        <serde_json::Value as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl JsonSql for I18nString {}

impl JsonSchema for I18nString {
    fn schema_name() -> String {
        "I18nString".into()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            ..Default::default()
        };
        let obj = schema.object();
        obj.properties
            .insert("en".to_owned(), <String>::json_schema(gen));
        obj.properties
            .insert("sv".to_owned(), <String>::json_schema(gen));
        schema.into()
    }
}
