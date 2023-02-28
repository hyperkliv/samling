use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    cornucopia::queries::admin::{
        AttributeTableData, AttributetypeTableData, CategoryTableData,
        CollectionPricelistTableData, CollectionTableData, ColorTableData,
        GroupCollectionTableData, GroupPricelistTableData, GroupTableData, GroupUserTableData,
        ImageTableData, NewCollectionColorTableData, NewCollectionStyleTableData,
        OrganizationTableData, PriceTableData, PricelistTableData, SizeCollectionTableData,
        SizeTableData, StyleAttributeTableData, StyleCategoryTableData, StyleTableData,
        UserOrganizationTableData, UserTableData,
    },
    I18nString, ImageSummary,
};

#[derive(Debug, Serialize, Clone)]
pub enum OrganizationDataRow {
    User(UserTableData),
    Organization(OrganizationTableData),
    Group(GroupTableData),
    Attributetype(AttributetypeTableData),
    Attribute(AttributeTableData),
    Category(CategoryTableData),
    Collection(CollectionTableData),
    Style(StyleTableData),
    Color(ColorTableData),
    Size(SizeTableData),
    Image(ImageTableData),
    Pricelist(PricelistTableData),
    Price(PriceTableData),
    UserOrganization(UserOrganizationTableData),
    CollectionPricelist(CollectionPricelistTableData),
    GroupCollection(GroupCollectionTableData),
    GroupPricelist(GroupPricelistTableData),
    GroupUser(GroupUserTableData),
    NewCollectionStyle(NewCollectionStyleTableData),
    NewCollectionColor(NewCollectionColorTableData),
    SizeCollection(SizeCollectionTableData),
    StyleAttribute(StyleAttributeTableData),
    StyleCategory(StyleCategoryTableData),
}

#[derive(Debug, Serialize, Clone)]
pub struct OrganizationData {
    pub user: Vec<UserTableData>,
    pub organization: Vec<OrganizationTableData>,
    pub group: Vec<GroupTableData>,
    pub attributetype: Vec<AttributetypeTableData>,
    pub attribute: Vec<AttributeTableData>,
    pub category: Vec<CategoryTableData>,
    pub collection: Vec<CollectionTableData>,
    pub style: Vec<StyleTableData>,
    pub color: Vec<ColorTableData>,
    pub size: Vec<SizeTableData>,
    pub image: Vec<ImageTableData>,
    pub pricelist: Vec<PricelistTableData>,
    pub price: Vec<PriceTableData>,
    pub user_organization: Vec<UserOrganizationTableData>,
    pub collection_pricelist: Vec<CollectionPricelistTableData>,
    pub group_collection: Vec<GroupCollectionTableData>,
    pub group_pricelist: Vec<GroupPricelistTableData>,
    pub group_user: Vec<GroupUserTableData>,
    pub new_collection_style: Vec<NewCollectionStyleTableData>,
    pub new_collection_color: Vec<NewCollectionColorTableData>,
    pub size_collection: Vec<SizeCollectionTableData>,
    pub style_attribute: Vec<StyleAttributeTableData>,
    pub style_category: Vec<StyleCategoryTableData>,
}

#[derive(Debug, Serialize, Clone, JsonSchema)]
pub struct ItemFilterChoices {
    pub status: Vec<StringFilterChoice>,
    pub category: Vec<EntityFilterChoice>,
    pub style: Vec<EntityFilterChoice>,
}

#[derive(Debug, Serialize, Clone, JsonSchema, derive_more::From)]
pub struct StringFilterChoice(String);

#[derive(Debug, Serialize, Clone, JsonSchema)]
pub struct EntityFilterChoice {
    pub id: i32,
    pub title: I18nString,
    pub subtitle: Option<I18nString>,
    pub image: Option<ImageSummary>,
}
