use indexmap::IndexSet;
use itertools::Itertools;
use schemars::JsonSchema;
use serde::Deserialize;
use strum::IntoStaticStr;

use super::expander::ExpandedRow;
use crate::{CategorySummary, ImageSummary, NestedColor, NestedPrice, NestedSize, NestedStyle};

pub(super) fn aggregate(
    mut rows: Vec<ExpandedRow>,
    mut group_by: IndexSet<GroupBy>,
) -> Vec<AggregatedRow> {
    if !group_by.contains(&GroupBy::Style) {
        group_by.insert(GroupBy::Style);
    }
    rows.sort_by_key(|row| row.group_by_ids(&group_by));
    let groups = rows
        .into_iter()
        .group_by(|row| row.group_by_ids(&group_by))
        .into_iter()
        .map(|(_ids, group)| group.collect_vec())
        .collect_vec();

    groups.into_iter().map(AggregatedRow::make).collect_vec()
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoStaticStr, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GroupBy {
    Style,
    Color,
    Size,
    Category,
    Image,
    PriceList,
}

impl std::fmt::Display for GroupBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        f.write_str(name)
    }
}

#[derive(Debug, Clone)]
pub(super) struct AggregatedRow<'a> {
    pub(super) style: &'a NestedStyle,
    pub(super) colors: Vec<&'a NestedColor>,
    pub(super) sizes: Vec<&'a NestedSize>,
    pub(super) categories: Vec<&'a CategorySummary>,
    pub(super) images: Vec<&'a ImageSummary>,
    pub(super) prices: Vec<&'a NestedPrice>,
}

impl<'a> AggregatedRow<'a> {
    fn make(rows: Vec<ExpandedRow<'a>>) -> Self {
        debug_assert!(!rows.is_empty());
        let row0 = rows.first().unwrap();
        let colors = rows
            .iter()
            .flat_map(|row| row.color)
            .unique_by(|x| x.id)
            .collect_vec();
        let sizes = rows
            .iter()
            .flat_map(|row| row.size)
            .unique_by(|x| x.id)
            .collect_vec();
        let categories = rows
            .iter()
            .flat_map(|row| row.category)
            .unique_by(|x| x.id)
            .collect();
        let images = rows
            .iter()
            .flat_map(|row| row.image)
            .unique_by(|x| x.id)
            .collect();
        let prices = rows
            .iter()
            .flat_map(|row| row.price)
            .unique_by(|x| x.id)
            .sorted_by_key(|p| p.list.id) // NOTE: To ensure reproducible order of prices
            .collect_vec();
        Self {
            style: row0.style,
            colors,
            sizes,
            categories,
            images,
            prices,
        }
    }
}
