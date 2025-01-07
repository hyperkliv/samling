mod aggregator;
mod expander;
mod export;
mod formats;
pub(crate) mod routes;

pub(crate) use aggregator::*;
use chrono::Utc;
pub(crate) use export::*;
pub(crate) use formats::*;
use indexmap::IndexSet;

use crate::{CollectionWithItems, Language, Result};

use self::expander::expand;

struct Export<'a> {
    collection: CollectionWithItems,
    fields: &'a [ExportField],
    language: Language,
    group_by: Option<IndexSet<GroupBy>>,
    export_format: ExportFormat,
    cell_separator: &'a str,
}

impl Export<'_> {
    fn filename(&self) -> String {
        format!(
            "{} {}{}",
            self.collection
                .collection
                .name
                .get_or_default(self.language),
            Utc::now(),
            self.export_format.file_extension()
        )
    }
    fn data(self) -> Result<bytes::Bytes> {
        let group_by = self.group_by.clone().unwrap_or_default();
        let aggregate_rows = self
            .collection
            .items
            .iter()
            .flat_map(|item| aggregate(expand(&item.style), group_by.clone()))
            .collect();
        let (columns, rows) = export_rows(aggregate_rows, self.fields, self.language);

        self.export_format
            .export(&columns, rows, self.cell_separator)
    }
}
