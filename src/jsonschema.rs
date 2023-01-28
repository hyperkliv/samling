use crate::{
    auth::{filters::UserFilters, sorting::UserSortOrder},
    exports::GroupBy,
    ApiErrorCode, ApiErrorResponse, CollectionFilters, CreateGroup, CreateUser, ExportField,
    ExportFormat, Group, GroupSummary, I18nString, NestedStyleSortOrder, Result, StyleFilters,
    UpdateGroup, UpdateOwnUser, UpdateUser,
};
use std::path::Path;

use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
    schema_for, JsonSchema,
};
use tokio::io::AsyncWriteExt;

pub(crate) fn rfc3339_date(gen: &mut SchemaGenerator) -> Schema {
    let mut schema: SchemaObject = <String>::json_schema(gen).into();
    schema.format = Some("date".to_owned());
    schema.into()
}

pub(crate) fn rfc3339_date_time(gen: &mut SchemaGenerator) -> Schema {
    let mut schema: SchemaObject = <String>::json_schema(gen).into();
    schema.format = Some("date-time".to_owned());
    schema.into()
}

pub async fn generate_schema_file(output_path: &Path) -> Result<()> {
    let schema = schema_for!(PublicJsonSchema);
    let schema_data = serde_json::to_string_pretty(&schema)?;
    tracing::info!("Writing schema to {output_path:?}...");
    let mut file = tokio::fs::File::create(&output_path).await?;
    file.write_all(schema_data.as_bytes()).await?;
    Ok(())
}

// To find JsonSchema derived structs you can use ripgrep (rg):
//     rg -A 3 JsonSchema | rg -r '$1' '.+struct (\w+) .+' | sort
#[derive(JsonSchema)]
#[allow(dead_code)]
pub(crate) struct PublicJsonSchema {
    i18n_string: I18nString,
    auth: AuthJsonSchema,
    errors: ErrorsSchema,
    export: ExportSchema,
    filters: FiltersSchema,
    sort_by: SortByJsonSchema,
    environment: crate::Environment,
    attribute: crate::Attribute,
    attribute_type: crate::AttributeType,
    attribute_type_summary: crate::AttributeTypeSummary,
    category: crate::Category,
    category_summary: crate::CategorySummary,
    collection: crate::Collection,
    collection_style_item: crate::CollectionItem,
    collection_with_items: crate::CollectionWithItems,
    collection_summary: crate::CollectionSummary,
    update_collection: crate::UpdateCollection,
    create_collection: crate::CreateCollection,
    color: crate::Color,
    color_summary: crate::ColorSummary,
    image: crate::Image,
    image_summary: crate::ImageSummary,
    nested_attribute: crate::AttributeSummary,
    nested_color: crate::NestedColor,
    nested_price: crate::NestedPrice,
    nested_size: crate::NestedSize,
    nested_style: crate::NestedStyle,
    nested_style_summary: crate::NestedStyleSummary,
    organization: crate::Organization,
    price: crate::Price,
    price_list: crate::PriceList,
    price_list_summary: crate::PriceListSummary,
    size: crate::Size,
    style: crate::Style,
    style_summary: crate::StyleSummary,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
pub(crate) struct ErrorsSchema {
    response: ApiErrorResponse,
    code: ApiErrorCode,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
pub(crate) struct SortByJsonSchema {
    user: UserSortOrder,
    nested_style: NestedStyleSortOrder,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
pub(crate) struct ExportSchema {
    format: ExportFormat,
    field: ExportField,
    group_by: GroupBy,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
pub(crate) struct FiltersSchema {
    user: UserFilters,
    style: StyleFilters,
    collection: CollectionFilters,
    item_filter_choices: crate::ItemFilterChoices,
}

#[derive(JsonSchema)]
#[allow(dead_code)]
pub(crate) struct AuthJsonSchema {
    user: crate::User,
    user_summary: crate::UserSummary,
    authenticated_user: crate::AuthenticatedUser,
    google_credentials: crate::GoogleCredentials,
    microsoft_credentials: crate::MicrosoftCredentials,
    group: Group,
    group_summary: GroupSummary,
    create_group: CreateGroup,
    update_group: UpdateGroup,
    create_user: CreateUser,
    update_user: UpdateUser,
    update_own_user: UpdateOwnUser,
}
