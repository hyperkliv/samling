use axum::extract::{Path, Query};
use axum::routing;
use http::header::CONTENT_DISPOSITION;
use http::{HeaderMap, HeaderValue};
use indexmap::IndexSet;
use serde::Deserialize;

use super::aggregator::GroupBy;
use super::export::ExportField;
use super::formats::ExportFormat;
use super::Export;
use crate::auth::Permission;
use crate::errors::Result;
use crate::extractors::{FiltersQuery, JsonQuery, PoolClient};
use crate::routes::AppRouter;
use crate::signing::Claims;
use crate::{
    Collection, CollectionFilters, CollectionsRepo, Filters, Id, Language, NestedStyleSortOrder,
    Organization, Ref,
};

pub(crate) fn org_routes() -> AppRouter {
    AppRouter::new().nest(
        "/exports/:collection_ref",
        AppRouter::new().route("/:export_format", routing::get(export_items)),
    )
}

#[derive(Deserialize)]
pub(crate) struct ExportQueryJsonParams {
    fields: Option<Vec<ExportField>>,
    group_by: Option<IndexSet<GroupBy>>,
}

#[derive(Deserialize)]
pub(crate) struct ExportQueryPlainParams {
    #[serde(default)]
    language: Language,
    cell_separator: Option<String>,
    #[serde(default)]
    all_fields: bool,
}

#[derive(Deserialize)]
pub(crate) struct ExportPathParams {
    organization_id: Id<Organization>,
    export_format: ExportFormat,
}

/// Get an export of style items, in the given format
// TODO: Lower amount of arguments
#[allow(clippy::too_many_arguments)]
pub(crate) async fn export_items(
    PoolClient(client): PoolClient,
    claims: Claims,
    collection_ref: Ref<Collection>,
    FiltersQuery(filters): FiltersQuery<CollectionFilters>,
    sorter: NestedStyleSortOrder,
    JsonQuery(ExportQueryJsonParams { fields, group_by }): JsonQuery<ExportQueryJsonParams>,
    Query(ExportQueryPlainParams {
        language,
        cell_separator,
        all_fields,
    }): Query<ExportQueryPlainParams>,
    Path(ExportPathParams {
        organization_id,
        export_format,
    }): Path<ExportPathParams>,
) -> Result<(HeaderMap, bytes::Bytes)> {
    let metadata = claims.ensure(
        organization_id,
        &[Permission::ExportStyles, Permission::ListCollectionItems],
    )?;
    let filters = filters.resolve(&client, organization_id).await?;
    let collection = CollectionsRepo
        .get_with_items(&client, metadata, &collection_ref, filters, sorter)
        .await?;
    let fields = if all_fields {
        ExportField::all()
    } else {
        fields.unwrap_or_else(ExportField::default_choices)
    };

    let export = Export {
        collection,
        fields: &fields,
        language,
        group_by,
        export_format,
        cell_separator: &cell_separator.unwrap_or_else(|| "\n".to_string()),
    };

    let content_disposition_value = format!(r#"attachment; filename="{}""#, export.filename());
    let mut header_map = HeaderMap::new();
    header_map
        .entry(CONTENT_DISPOSITION)
        .or_insert(HeaderValue::from_str(&content_disposition_value)?);
    Ok((header_map, export.data()?))
}
