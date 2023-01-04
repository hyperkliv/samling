use indexmap::IndexSet;
use itertools::Itertools;

use super::aggregator::GroupBy;
use crate::{
    AnyId, CategorySummary, ImageSummary, NestedColor, NestedPrice, NestedSize, NestedStyle,
};

#[derive(Debug, Clone)]
pub(super) struct ExpandedRow<'a> {
    pub(super) style: &'a NestedStyle,
    pub(super) color: Option<&'a NestedColor>,
    pub(super) size: Option<&'a NestedSize>,
    pub(super) category: Option<&'a CategorySummary>,
    pub(super) image: Option<&'a ImageSummary>,
    pub(super) price: Option<&'a NestedPrice>,
}

impl<'a> ExpandedRow<'a> {
    pub(crate) fn group_by_ids(&self, group_by: &IndexSet<GroupBy>) -> Vec<AnyId> {
        group_by
            .iter()
            .filter_map(|&g| match g {
                GroupBy::Style => Some(self.style.id.into()),
                GroupBy::Color => self.color.map(|c| c.id.into()),
                GroupBy::Size => self.size.map(|s| s.id.into()),
                GroupBy::Category => self.category.map(|c| c.id.into()),
                GroupBy::Image => self.image.map(|i| i.id.into()),
                GroupBy::PriceList => self.price.map(|r| r.list.id.into()),
            })
            .collect_vec()
    }

    fn new(style: &'a NestedStyle) -> Self {
        Self {
            style,
            color: None,
            size: None,
            category: None,
            image: None,
            price: None,
        }
    }

    fn with_color(self, color: &'a NestedColor) -> Self {
        Self {
            color: Some(color),
            ..self
        }
    }

    fn with_size(self, size: &'a NestedSize) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    fn clear_size(self) -> Self {
        Self { size: None, ..self }
    }

    fn with_category(self, category: &'a CategorySummary) -> Self {
        Self {
            category: Some(category),
            ..self
        }
    }

    fn with_image(self, image: &'a ImageSummary) -> Self {
        Self {
            image: Some(image),
            ..self
        }
    }

    fn with_price(self, price: &'a NestedPrice) -> Self {
        Self {
            price: Some(price),
            ..self
        }
    }
}

pub(super) fn expand(style: &NestedStyle) -> Vec<ExpandedRow> {
    let base = ExpandedRow::new(style);
    let expanded = expand_colors(base, &style.colors);

    let expanded = expand_categories(expanded, &style.categories);
    expand_prices(expanded, &style.prices)
}

fn expand_colors<'a>(mut base: ExpandedRow<'a>, colors: &'a [NestedColor]) -> Vec<ExpandedRow<'a>> {
    let mut sources = Vec::new();
    if colors.is_empty() {
        sources.push(base);
    } else {
        let mut sub_bases = Vec::new();
        for color in colors {
            base = base.with_color(color).clear_size();
            if color.sizes.is_empty() {
                sub_bases.push(base.clone());
            } else {
                for size in &color.sizes {
                    base = base.with_size(size);
                    sub_bases.push(base.clone());
                }
            }

            sub_bases = expand_images(sub_bases, &color.images);
            sources.append(&mut sub_bases);
        }
    }
    sources
}

fn expand_images<'a>(
    mut sources: Vec<ExpandedRow<'a>>,
    images: &'a [ImageSummary],
) -> Vec<ExpandedRow<'a>> {
    if images.is_empty() {
        sources
    } else {
        let temp_rows = sources.drain(..).collect_vec();
        for image in images {
            for temp_row in temp_rows.clone() {
                sources.push(temp_row.with_image(image));
            }
        }
        sources
    }
}

fn expand_prices<'a>(
    mut sources: Vec<ExpandedRow<'a>>,
    prices: &'a [NestedPrice],
) -> Vec<ExpandedRow<'a>> {
    if prices.is_empty() {
        sources
    } else {
        let temp_rows = sources.drain(..).collect_vec();
        for price in prices {
            for temp_row in temp_rows.clone() {
                sources.push(temp_row.with_price(price));
            }
        }
        sources
    }
}

fn expand_categories<'a>(
    mut sources: Vec<ExpandedRow<'a>>,
    categories: &'a [CategorySummary],
) -> Vec<ExpandedRow<'a>> {
    if categories.is_empty() {
        sources
    } else {
        let temp_rows = sources.drain(..).collect_vec();
        for category in categories {
            for temp_row in temp_rows.clone() {
                sources.push(temp_row.with_category(category));
            }
        }
        sources
    }
}
