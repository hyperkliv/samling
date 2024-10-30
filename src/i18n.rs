use cornucopia_async::JsonSql;
use postgres_types::{private::BytesMut, to_sql_checked, IsNull, ToSql, Type};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumIter,
    JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    En,
    Sv,
    De,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, JsonSchema, Default,
)]
pub struct I18nString {
    #[serde(default)]
    pub en: String,
    #[serde(default)]
    pub sv: String,
    #[serde(default)]
    pub de: String,
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
    pub fn new(language: Language, value: String) -> Self {
        match language {
            Language::En => Self {
                en: value,
                ..Self::default()
            },
            Language::Sv => Self {
                sv: value,
                ..Self::default()
            },
            Language::De => Self {
                de: value,
                ..Self::default()
            },
        }
    }

    pub fn get<L: Into<Language>>(&self, language: L) -> &str {
        match language.into() {
            Language::En => &self.en,
            Language::Sv => &self.sv,
            Language::De => &self.de,
        }
    }

    pub fn get_or_default<L: Into<Language>>(&self, language: L) -> &str {
        let val = self.get(language);
        if val.is_empty() {
            &self.en
        } else {
            val
        }
    }

    pub fn set<L: Into<Language>>(&mut self, language: L, value: String) {
        match language.into() {
            Language::En => self.en = value,
            Language::Sv => self.sv = value,
            Language::De => self.de = value,
        }
    }

    pub fn merge(values: Vec<Self>) -> Self {
        let mut out = I18nString::default();
        for value in values {
            for language in Language::iter() {
                let found = value.get(language);
                if out.get(language) == "" && !found.is_empty() {
                    out.set(language, found.to_owned());
                }
            }
        }
        out
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

#[cfg(test)]
mod tests {
    use crate::Language;

    use super::I18nString;

    #[test]
    fn i18nstring_merge_works() {
        let values = vec![
            I18nString::new(Language::En, "Hello".to_string()),
            I18nString::new(Language::Sv, "Hej".to_string()),
            I18nString::new(Language::De, "Hallo".to_string()),
        ];
        assert_eq!(
            I18nString::merge(values),
            I18nString {
                en: "Hello".to_string(),
                sv: "Hej".to_string(),
                de: "Hallo".to_string(),
            }
        );
    }

    #[test]
    fn i18nstring_merge_takes_first_nonempty_string() {
        let values = vec![
            I18nString::new(Language::En, "Hello".to_string()),
            I18nString::new(Language::Sv, "".to_string()),
            I18nString::new(Language::Sv, "Tjena".to_string()),
        ];
        assert_eq!(
            I18nString::merge(values),
            I18nString {
                en: "Hello".to_string(),
                sv: "Tjena".to_string(),
                ..Default::default()
            }
        );
    }
}
