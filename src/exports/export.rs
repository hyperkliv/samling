use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use strum::{EnumIter, IntoEnumIterator};

use super::aggregator::AggregatedRow;
use crate::{traits::YearWeek, ExternalId, Language, RefTarget, RefType};

#[derive(
    Debug, Copy, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExportField {
    StyleNumber,
    StyleName,
    StyleExternalId,
    StyleDescription,
    NewStyle,
    Core,
    CountryOfOrigin,
    ColorNumber,
    ColorName,
    ColorExternalId,
    NewColor,
    SizeType,
    SizeNumber,
    ServiceItem,
    RetailPriceAmount,
    RetailPriceCurrency,
    RetailPriceList,
    UnitPriceAmount,
    UnitPriceCurrency,
    UnitPriceList,
    CategoryName,
    DeliveryPeriod,
    TariffNo,
    PrimaryImage,
    Images,
    EanCode,
    GrossWeight,
    UnitVolume,
    Attribute,
}

impl ExportField {
    pub(super) fn all() -> Vec<Self> {
        Self::iter().collect()
    }

    pub(super) fn default_choices() -> Vec<Self> {
        vec![
            Self::StyleNumber,
            Self::StyleName,
            Self::CategoryName,
            Self::NewStyle,
            Self::Core,
            Self::DeliveryPeriod,
            Self::CountryOfOrigin,
            Self::ColorNumber,
            Self::ColorName,
            Self::NewColor,
            Self::SizeType,
            Self::SizeNumber,
            Self::ServiceItem,
            Self::RetailPriceAmount,
            Self::RetailPriceCurrency,
            Self::RetailPriceList,
            Self::UnitPriceAmount,
            Self::UnitPriceCurrency,
            Self::UnitPriceList,
            Self::TariffNo,
            Self::GrossWeight,
            Self::UnitVolume,
            Self::StyleDescription,
            Self::PrimaryImage,
            Self::Images,
            Self::EanCode,
            Self::Attribute,
        ]
    }
}

impl ExportField {
    fn column(&self) -> Column {
        match self {
            Self::StyleNumber => "Style number".into(),
            Self::StyleName => "Style name".into(),
            Self::StyleExternalId => "Style external ID".into(),
            Self::SizeType => "Size type".into(),
            Self::DeliveryPeriod => "Delivery period".into(),
            Self::TariffNo => "Tariff no".into(),
            Self::PrimaryImage => "Primary image".into(),
            Self::StyleDescription => "Description".into(),
            Self::NewStyle => "New style".into(),
            Self::CountryOfOrigin => "Country of origin".into(),
            Self::Core => "Core".into(),
            Self::ColorNumber => "Color number".into(),
            Self::ColorName => "Color name".into(),
            Self::ColorExternalId => "Color external ID".into(),
            Self::NewColor => "New color".into(),
            Self::SizeNumber => "Size".into(),
            Self::ServiceItem => "Service item".into(),
            Self::EanCode => "EAN code".into(),
            Self::GrossWeight => "Gross weight".into(),
            Self::UnitVolume => "Unit volume".into(),
            Self::CategoryName => "Category".into(),
            Self::RetailPriceAmount => "Retail price".into(),
            Self::RetailPriceCurrency => "Retail price currency".into(),
            Self::RetailPriceList => "Retail price list".into(),
            Self::UnitPriceAmount => "Unit price".into(),
            Self::UnitPriceCurrency => "Unit price currency".into(),
            Self::UnitPriceList => "Unit price list".into(),
            Self::Images => "Image".into(),
            Self::Attribute => "TODO: Ugly solution".into(),
        }
    }

    fn process(&self, row: &AggregatedRow, language: Language) -> Vec<(Column, Value)> {
        match self {
            Self::StyleNumber => vec![(self.column(), row.style.number.clone().into())],
            Self::StyleName => vec![(self.column(), row.style.name.get(language).into())],
            Self::StyleExternalId => vec![(
                self.column(),
                row.style
                    .external_id
                    .clone()
                    .map(|e| e.into())
                    .unwrap_or_default(),
            )],
            Self::NewStyle => vec![(self.column(), row.style.is_new.into())],
            Self::CountryOfOrigin => vec![(
                self.column(),
                row.style
                    .country_of_origin
                    .clone()
                    .unwrap_or_default()
                    .into(),
            )],
            Self::StyleDescription => {
                vec![(self.column(), row.style.description.get(language).into())]
            }
            Self::Core => vec![(self.column(), row.style.core.into())],
            Self::TariffNo => vec![(self.column(), row.style.tariff_no.clone().into())],
            Self::GrossWeight => vec![(self.column(), row.style.gross_weight.into())],
            Self::UnitVolume => vec![(self.column(), row.style.unit_volume.into())],
            Self::ColorNumber => vec![(
                self.column(),
                row.colors
                    .iter()
                    .map(|c| c.number.clone())
                    .collect_vec()
                    .into(),
            )],
            Self::ColorName => vec![(
                self.column(),
                row.colors
                    .iter()
                    .map(|c| c.name.get(language))
                    .collect_vec()
                    .into(),
            )],
            Self::ColorExternalId => vec![(
                self.column(),
                row.colors
                    .iter()
                    .map(|c| c.external_id.clone().map(|e| e.to_string()))
                    .collect_vec()
                    .into(),
            )],
            Self::NewColor => vec![(
                self.column(),
                row.colors.iter().map(|c| c.is_new).collect_vec().into(),
            )],
            Self::SizeType => vec![(
                self.column(),
                (if row.sizes.iter().any(|s| s.number.contains('½')) {
                    "Half"
                } else {
                    "Whole"
                })
                .into(),
            )],
            Self::SizeNumber => {
                use itertools::MinMaxResult::*;
                let has_half_size = row.sizes.iter().any(|s| s.number.contains('½'));
                vec![(
                    self.column(),
                    match row
                        .sizes
                        .iter()
                        // TODO: Add field position
                        .minmax_by_key(|s| s.position)
                    {
                        NoElements => "".into(),
                        OneElement(s) => s.number.clone().into(),
                        MinMax(min, max) => {
                            let mut s = format!("{} - {}", min.number, max.number);
                            if has_half_size && !s.contains('½') {
                                s.push_str(" (½)");
                            }
                            s.into()
                        }
                    },
                )]
            }
            Self::CategoryName => vec![(
                self.column(),
                row.categories
                    .iter()
                    .map(|c| c.name.get(language))
                    .collect_vec()
                    .into(),
            )],
            Self::ServiceItem => vec![(
                self.column(),
                row.sizes
                    .iter()
                    .map(|s| s.service_item)
                    .collect_vec()
                    .into(),
            )],
            Self::EanCode => vec![(
                self.column(),
                row.sizes
                    .iter()
                    .map(|size| size.ean_code.clone().unwrap_or_default())
                    .collect_vec()
                    .into(),
            )],
            Self::DeliveryPeriod => vec![(
                self.column(),
                row.sizes
                    .iter()
                    .map(|size| {
                        size.delivery_period
                            .map(|d| d.year_week())
                            .unwrap_or_default()
                    })
                    .sorted()
                    .unique()
                    .collect_vec()
                    .into(),
            )],
            Self::RetailPriceAmount => vec![(
                self.column(),
                row.prices
                    .iter()
                    .filter(|price| price.is_retail_price())
                    .map(|price| price.amount.to_string())
                    .collect_vec()
                    .into(),
            )],
            Self::RetailPriceCurrency => vec![(
                self.column(),
                row.prices
                    .iter()
                    .filter(|price| price.is_retail_price())
                    .map(|price| price.currency.clone())
                    .collect_vec()
                    .into(),
            )],
            Self::RetailPriceList => vec![(
                self.column(),
                row.prices
                    .iter()
                    .filter(|price| price.is_retail_price())
                    .map(|price| price.list.name.clone())
                    .collect_vec()
                    .into(),
            )],
            Self::UnitPriceAmount => vec![(
                self.column(),
                row.prices
                    .iter()
                    .filter(|price| price.is_unit_price())
                    .map(|price| price.amount.to_string())
                    .collect_vec()
                    .into(),
            )],
            Self::UnitPriceCurrency => vec![(
                self.column(),
                row.prices
                    .iter()
                    .filter(|price| price.is_unit_price())
                    .map(|price| price.currency.clone())
                    .collect_vec()
                    .into(),
            )],
            Self::UnitPriceList => vec![(
                self.column(),
                row.prices
                    .iter()
                    .filter(|price| price.is_unit_price())
                    .map(|price| price.list.name.clone())
                    .collect_vec()
                    .into(),
            )],
            Self::PrimaryImage => vec![(
                self.column(),
                row.images
                    .first()
                    .map(|image| image.url.to_string())
                    .unwrap_or_default()
                    .into(),
            )],
            Self::Images => vec![(
                self.column(),
                row.images
                    .iter()
                    .map(|c| c.url.clone().to_string())
                    .collect_vec()
                    .into(),
            )],
            Self::Attribute => {
                let map = row
                    .style
                    .attributes
                    .iter()
                    .into_group_map_by(|&attr| attr.r#type.name.get(language));
                map.into_iter()
                    .map(|(type_name, type_attrs)| {
                        (
                            type_name.into(),
                            type_attrs
                                .into_iter()
                                .map(|attr| attr.title.get(language))
                                .collect_vec()
                                .into(),
                        )
                    })
                    .collect_vec()
            }
        }
    }
}
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct Column(smol_str::SmolStr);

impl Column {
    pub(super) fn name(&self) -> smol_str::SmolStr {
        self.0.clone()
    }
}

impl From<String> for Column {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&str> for Column {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum Value {
    String(String),
    VecString(Vec<String>),
    Url(url::Url),
    Bool(bool),
    VecBool(Vec<bool>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(s) => serializer.serialize_str(s),
            Value::VecString(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for s in v {
                    seq.serialize_element(s)?;
                }
                seq.end()
            }
            Value::Url(u) => serializer.serialize_str(u.as_str()),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::VecBool(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for s in v {
                    seq.serialize_element(s)?;
                }
                seq.end()
            }
        }
    }
}

impl Value {
    pub(super) fn finalize(&self, separator: &str) -> String {
        match self {
            Value::VecString(val) => val.join(separator),
            Value::String(val) => val.to_owned(),
            Value::Url(url) => url.to_string(),
            Value::Bool(val) => {
                if *val {
                    "Yes".into()
                } else {
                    "No".into()
                }
            }
            Value::VecBool(values) => {
                let mut counts = values.iter().counts();
                match (
                    counts.remove(&true).unwrap_or_default(),
                    counts.remove(&false).unwrap_or_default(),
                ) {
                    (1.., 0) => "Yes".into(),
                    (0, 0..) => "No".into(),
                    (yes, no) => [format!("Yes ({yes})"), format!("No ({no})")].join(separator),
                }
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.finalize(" | "))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::String(String::new())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<Decimal> for Value {
    fn from(d: Decimal) -> Self {
        Self::String(d.to_string())
    }
}

impl From<url::Url> for Value {
    fn from(s: url::Url) -> Self {
        Self::Url(s)
    }
}

impl From<Option<String>> for Value {
    fn from(s: Option<String>) -> Self {
        Self::String(s.unwrap_or_default())
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<Option<bool>> for Value {
    fn from(v: Option<bool>) -> Self {
        Self::Bool(v.unwrap_or_default())
    }
}

impl From<Vec<String>> for Value {
    fn from(s: Vec<String>) -> Self {
        Self::VecString(s)
    }
}

impl From<Vec<Option<String>>> for Value {
    fn from(s: Vec<Option<String>>) -> Self {
        Self::VecString(s.into_iter().flatten().collect())
    }
}

impl From<Vec<bool>> for Value {
    fn from(s: Vec<bool>) -> Self {
        Self::VecBool(s)
    }
}

impl From<Vec<Option<bool>>> for Value {
    fn from(s: Vec<Option<bool>>) -> Self {
        Self::VecBool(s.into_iter().flatten().collect())
    }
}

impl<T: RefTarget> From<ExternalId<T>> for Value {
    fn from(e: ExternalId<T>) -> Self {
        Self::String(e.inner().to_owned())
    }
}

impl<T: RefTarget> From<Option<ExternalId<T>>> for Value {
    fn from(o: Option<ExternalId<T>>) -> Self {
        Self::String(o.map(|e| e.inner().to_owned()).unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
struct PreExportRow(IndexMap<Column, Value>);

impl PreExportRow {
    fn new(row: AggregatedRow, fields: &[ExportField], language: Language) -> Self {
        let mut map = IndexMap::with_capacity(fields.len());
        for field in fields {
            for (column, value) in field.process(&row, language) {
                map.insert(column, value);
            }
        }
        Self(map)
    }

    fn columns(&self) -> IndexSet<Column> {
        self.0.keys().cloned().collect()
    }

    fn remove(&mut self, key: &Column) -> Option<Value> {
        self.0.swap_remove(key)
    }
}

#[derive(Debug, Clone)]
pub(super) struct ExportRow(Vec<Value>);

impl ExportRow {
    fn new(columns: &IndexSet<Column>, mut prerow: PreExportRow) -> Self {
        ExportRow(
            columns
                .iter()
                .map(|col| prerow.remove(col).unwrap_or_default())
                .collect_vec(),
        )
    }

    pub(super) fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    pub(super) fn finalized_strings(self, separator: &str) -> Vec<String> {
        self.0.into_iter().map(|v| v.finalize(separator)).collect()
    }

    pub(crate) fn inner(self) -> Vec<Value> {
        self.0
    }
}

pub(super) fn export_rows(
    aggregate_rows: Vec<AggregatedRow>,
    fields: &[ExportField],
    language: Language,
) -> (IndexSet<Column>, VecDeque<ExportRow>) {
    let mut columns = IndexSet::new();
    let mut pre_rows = Vec::new();
    for aggrow in aggregate_rows {
        let prerow = PreExportRow::new(aggrow, fields, language);
        columns.extend(prerow.columns());
        pre_rows.push(prerow);
    }
    let rows = pre_rows
        .into_iter()
        .map(|row| ExportRow::new(&columns, row))
        .collect();
    (columns, rows)
}
