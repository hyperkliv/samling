// To parse this data:
//
//   import { Convert, Api } from "./file";
//
//   const api = Convert.toApi(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface Api {
    attribute:              Attribute;
    attribute_type:         AttributeType;
    attribute_type_summary: AttributeTypeSummary;
    auth:                   AuthJsonSchema;
    category:               Category;
    category_summary:       CategorySummary;
    collection:             Collection;
    collection_style_item:  CollectionItem;
    collection_summary:     CollectionSummary;
    collection_with_items:  CollectionWithItems;
    color:                  Color;
    color_summary:          ColorSummary;
    create_collection:      CreateCollection;
    environment:            Environment;
    errors:                 ErrorsSchema;
    export:                 ExportSchema;
    filters:                FiltersSchema;
    i18n_string:            I18NString;
    image:                  Image;
    image_summary:          ImageSummary;
    nested_attribute:       AttributeSummary;
    nested_color:           NestedColor;
    nested_price:           NestedPrice;
    nested_size:            NestedSize;
    nested_style:           NestedStyle;
    nested_style_summary:   NestedStyleSummary;
    organization:           Organization;
    price:                  Price;
    price_list:             PriceList;
    price_list_summary:     PriceListSummary;
    size:                   Size;
    sort_by:                SortByJsonSchema;
    style:                  Style;
    style_summary:          StyleSummary;
    update_collection:      UpdateCollection;
}

/**
 * Attribute
 */
export interface Attribute {
    created_at:   string;
    created_by?:  number | null;
    description:  I18NString;
    external_id?: null | string;
    id:           number;
    slug:         string;
    title:        I18NString;
    type:         AttributeTypeSummary;
    updated_at:   string;
}

export interface I18NString {
    en?: string;
    sv?: string;
}

/**
 * Attribute type summary
 */
export interface AttributeTypeSummary {
    external_id?: null | string;
    id:           number;
    name:         I18NString;
    slug:         string;
}

/**
 * Attribute type
 */
export interface AttributeType {
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    name:         I18NString;
    slug:         string;
    updated_at:   string;
}

export interface AuthJsonSchema {
    authenticated_user:    AuthenticatedUser;
    create_group:          CreateGroup;
    create_user:           CreateUser;
    google_credentials:    GoogleCredentials;
    group:                 Group;
    group_summary:         GroupSummary;
    microsoft_credentials: MicrosoftCredentials;
    update_group:          UpdateGroup;
    update_own_user:       UpdateOwnUser;
    update_user:           UpdateUser;
    user:                  User;
    user_summary:          UserSummary;
}

export interface AuthenticatedUser {
    environment: Environment;
    token:       string;
    user:        User;
}

export enum Environment {
    Development = "development",
    Production = "production",
    Staging = "staging",
}

export interface User {
    created_at:     string;
    email:          string;
    groups:         GroupSummary[];
    id:             number;
    last_sign_in:   string;
    name:           string;
    organizations:  UserOrganization[];
    profile_image?: null | string;
    updated_at:     string;
}

export interface GroupSummary {
    description:     string;
    external_id?:    null | string;
    id:              number;
    name:            string;
    num_collections: number;
    num_price_lists: number;
    num_users:       number;
    slug:            string;
}

export interface UserOrganization {
    organization: OrganizationSummary;
    roles:        Role[];
}

/**
 * Organization
 */
export interface OrganizationSummary {
    id:        number;
    logo_url?: null | string;
    name:      string;
}

/**
 * A role that can be assigned to a user
 *
 * The u8 representation is meant for database storage/retrieval
 */
export enum Role {
    Active = "Active",
    Administrator = "Administrator",
    Editor = "Editor",
    Viewer = "Viewer",
}

export interface CreateGroup {
    collections?: RefForCollection[];
    description?: string;
    external_id?: null | string;
    name:         string;
    price_lists?: RefForPriceList[];
    slug?:        null | string;
    users?:       RefForUser[];
}

export interface RefForCollection {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface RefForPriceList {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface RefForUser {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface CreateUser {
    email:          string;
    groups?:        RefForGroup[] | null;
    name:           string;
    password?:      null | string;
    profile_image?: null | string;
    roles?:         Role[] | null;
}

export interface RefForGroup {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface GoogleCredentials {
    idToken: string;
}

export interface Group {
    collections:  CollectionSummary[];
    created_at:   string;
    created_by?:  number | null;
    description:  string;
    external_id?: null | string;
    id:           number;
    name:         string;
    price_lists:  PriceListSummary[];
    slug:         string;
    updated_at:   string;
    users:        UserSummary[];
}

/**
 * Collection
 */
export interface CollectionSummary {
    acronym:      I18NString;
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    image_url?:   null | string;
    name:         I18NString;
    num_colors:   number;
    num_sizes:    number;
    num_styles:   number;
    pricing:      CollectionPricing[];
    slug:         string;
    updated_at:   string;
}

export interface CollectionPricing {
    date: string;
    list: PriceListSummary;
}

/**
 * Price list summary
 */
export interface PriceListSummary {
    external_id?: null | string;
    id:           number;
    name:         string;
    slug:         string;
}

export interface UserSummary {
    email:          string;
    id:             number;
    last_sign_in:   string;
    name:           string;
    profile_image?: null | string;
}

export interface MicrosoftCredentials {
    accessToken:   string;
    idToken:       string;
    idTokenClaims: MicrosoftClaims;
}

export interface MicrosoftClaims {
    email: string;
    name:  string;
}

export interface UpdateGroup {
    collections?: RefForCollection[] | null;
    description?: null | string;
    external_id?: null | string;
    name?:        null | string;
    price_lists?: RefForPriceList[] | null;
    slug?:        null | string;
    users?:       RefForUser[] | null;
}

export interface UpdateOwnUser {
    email?:         null | string;
    name?:          null | string;
    password?:      null | string;
    profile_image?: null | string;
}

export interface UpdateUser {
    email?:         null | string;
    groups?:        RefForGroup[] | null;
    name?:          null | string;
    password?:      null | string;
    profile_image?: null | string;
    roles?:         Role[] | null;
}

/**
 * Category
 */
export interface Category {
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    name:         I18NString;
    slug:         string;
    updated_at:   string;
}

/**
 * Category summary
 */
export interface CategorySummary {
    external_id?: null | string;
    id:           number;
    name:         I18NString;
    slug:         string;
}

/**
 * Collection
 */
export interface Collection {
    acronym:      I18NString;
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    image_url?:   null | string;
    name:         I18NString;
    pricing:      CollectionPricing[];
    sizes:        NestedSize[];
    slug:         string;
    updated_at:   string;
}

/**
 * Nested size (well, used by NestedColor, so `color` field isn't needed)
 */
export interface NestedSize {
    delivery_period?: string;
    ean_code?:        null | string;
    external_id?:     null | string;
    id:               number;
    name:             I18NString;
    number:           string;
    position:         number;
    service_item?:    boolean | null;
    slug:             string;
    status?:          null | string;
}

/**
 * Collection
 */
export interface CollectionItem {
    style:        NestedStyle;
    user_comment: string;
}

/**
 * Style nested with colors and sizes, with some metadata fields excluded
 */
export interface NestedStyle {
    attributes:         AttributeSummary[];
    categories:         CategorySummary[];
    colors:             NestedColor[];
    core?:              boolean | null;
    country_of_origin?: null | string;
    description:        I18NString;
    external_id?:       null | string;
    gross_weight:       number;
    id:                 number;
    is_new?:            boolean | null;
    name:               I18NString;
    net_weight:         number;
    number:             string;
    prices:             NestedPrice[];
    slug:               string;
    tariff_no?:         null | string;
    unit_volume:        number;
}

/**
 * Nested attribute
 */
export interface AttributeSummary {
    description:  I18NString;
    external_id?: null | string;
    id:           number;
    slug:         string;
    title:        I18NString;
    type:         AttributeTypeSummary;
}

/**
 * Color with sizes included
 */
export interface NestedColor {
    external_id?: null | string;
    id:           number;
    images:       ImageSummary[];
    is_new?:      boolean | null;
    name:         I18NString;
    number:       string;
    sizes:        NestedSize[];
    slug:         string;
}

/**
 * Image summary
 */
export interface ImageSummary {
    external_id?: null | string;
    id:           number;
    url:          string;
}

/**
 * Nested price set, for inclusion in a NestedStyle
 */
export interface NestedPrice {
    amount:   number;
    currency: string;
    end:      string;
    id:       number;
    list:     PriceListSummary;
    start:    string;
    type:     PriceType;
    uom?:     null | string;
}

export enum PriceType {
    Retail = "Retail",
    Unit = "Unit",
}

/**
 * Collection
 */
export interface CollectionWithItems {
    acronym:      I18NString;
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    image_url?:   null | string;
    items:        CollectionItem[];
    name:         I18NString;
    num_colors:   number;
    num_sizes:    number;
    num_styles:   number;
    pricing:      CollectionPricing[];
    slug:         string;
    updated_at:   string;
}

/**
 * Color
 */
export interface Color {
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    images:       ImageSummary[];
    name:         I18NString;
    number:       string;
    slug:         string;
    style:        StyleSummary;
    updated_at:   string;
}

/**
 * Style summary
 */
export interface StyleSummary {
    external_id?: null | string;
    id:           number;
    name:         I18NString;
    number:       string;
    slug:         string;
}

/**
 * Color summary
 */
export interface ColorSummary {
    external_id?: null | string;
    id:           number;
    name:         I18NString;
    number:       string;
    slug:         string;
    style:        StyleSummary;
}

/**
 * Collection, for creation
 */
export interface CreateCollection {
    acronym:      I18NString;
    external_id?: null | string;
    image?:       ImageSource | null;
    name:         I18NString;
    new_colors:   RefForColor[];
    new_styles:   RefForStyle[];
    pricing:      CollectionPricing[];
    sizes?:       RefForSize[];
    slug?:        null | string;
}

export interface ImageSource {
    url?:    string;
    bytes?:  number[];
    base64?: string;
}

export interface RefForColor {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface RefForStyle {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface RefForSize {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface ErrorsSchema {
    code:     ApiErrorCode;
    response: ApiErrorResponse;
}

/**
 * Auto-generated discriminant enum variants
 */
export enum ApiErrorCode {
    ApplicationNotReady = "ApplicationNotReady",
    Base64DecodeError = "Base64DecodeError",
    CsvError = "CsvError",
    DbBuildError = "DbBuildError",
    DbCreatePoolError = "DbCreatePoolError",
    DbError = "DbError",
    DbPoolError = "DbPoolError",
    EmptySlugDisallowed = "EmptySlugDisallowed",
    ExpiredToken = "ExpiredToken",
    ExplicitIdCreationDisallowed = "ExplicitIdCreationDisallowed",
    ExternalIdAlreadyExists = "ExternalIdAlreadyExists",
    ExternalIdNotFound = "ExternalIdNotFound",
    ExternalIdReferenceUnsupported = "ExternalIdReferenceUnsupported",
    ExternalRequestError = "ExternalRequestError",
    FailedPasswordHashing = "FailedPasswordHashing",
    FailedPasswordValidation = "FailedPasswordValidation",
    IdAlreadyExists = "IdAlreadyExists",
    IdNotFound = "IdNotFound",
    ImageAlreadyExists = "ImageAlreadyExists",
    ImageBackendMisconfigured = "ImageBackendMisconfigured",
    ImageUploadsUnavailable = "ImageUploadsUnavailable",
    InvalidDbRoleId = "InvalidDbRoleId",
    InvalidEntityRef = "InvalidEntityRef",
    InvalidHttpHeaderValue = "InvalidHttpHeaderValue",
    InvalidToken = "InvalidToken",
    InvalidUserCredentials = "InvalidUserCredentials",
    IoError = "IoError",
    JsonError = "JsonError",
    MissingEntityRefPathParameter = "MissingEntityRefPathParameter",
    MissingPermissions = "MissingPermissions",
    PathJsonError = "PathJsonError",
    PathRejection = "PathRejection",
    QueryParsingError = "QueryParsingError",
    QueryRejection = "QueryRejection",
    SlugAlreadyExists = "SlugAlreadyExists",
    SlugNotFound = "SlugNotFound",
    SlugReferenceUnsupported = "SlugReferenceUnsupported",
    UnverifiedEmail = "UnverifiedEmail",
    UrlParseError = "UrlParseError",
    UserEmailAlreadyExists = "UserEmailAlreadyExists",
    UserEmailNotFound = "UserEmailNotFound",
    XlsxError = "XlsxError",
}

export interface ApiErrorResponse {
    error_code:    ApiErrorCode;
    error_message: string;
}

export interface ExportSchema {
    field:    ExportField;
    format:   ExportFormat;
    group_by: GroupBy;
}

export enum ExportField {
    Attribute = "attribute",
    CategoryName = "category_name",
    ColorExternalid = "color_external_id",
    ColorName = "color_name",
    ColorNumber = "color_number",
    Core = "core",
    CountryOfOrigin = "country_of_origin",
    DeliveryPeriod = "delivery_period",
    EanCode = "ean_code",
    GrossWeight = "gross_weight",
    Images = "images",
    NewColor = "new_color",
    NewStyle = "new_style",
    PrimaryImage = "primary_image",
    RetailPriceAmount = "retail_price_amount",
    RetailPriceCurrency = "retail_price_currency",
    RetailPriceList = "retail_price_list",
    ServiceItem = "service_item",
    SizeNumber = "size_number",
    SizeType = "size_type",
    StyleDescription = "style_description",
    StyleExternalid = "style_external_id",
    StyleName = "style_name",
    StyleNumber = "style_number",
    TariffNo = "tariff_no",
    UnitPriceAmount = "unit_price_amount",
    UnitPriceCurrency = "unit_price_currency",
    UnitPriceList = "unit_price_list",
    UnitVolume = "unit_volume",
}

export enum ExportFormat {
    Csv = "csv",
    Json = "json",
    Xlsx = "xlsx",
}

export enum GroupBy {
    Category = "category",
    Color = "color",
    Image = "image",
    PriceList = "price_list",
    Size = "size",
    Style = "style",
}

export interface FiltersSchema {
    collection: CollectionFilters;
    style:      StyleFilters;
    user:       UserFilters;
}

export interface CollectionFilters {
    styles?: StyleFilters;
}

export interface StyleFilters {
    categories?:        RefForCategory[] | null;
    core?:              boolean | null;
    country_of_origin?: string[] | null;
    new_colors?:        boolean | null;
    new_styles?:        boolean | null;
    numbers?:           string[] | null;
    pricelists?:        RefForPriceList[] | null;
    refs?:              RefForStyle[] | null;
    service_item?:      boolean | null;
    status?:            string[] | null;
}

export interface RefForCategory {
    id?:          number;
    external_id?: string;
    slug?:        string;
}

export interface UserFilters {
    groups?: RefForGroup[] | null;
    roles?:  Role[] | null;
}

/**
 * Image
 */
export interface Image {
    color:              ColorSummary;
    external_checksum?: null | string;
    external_id?:       null | string;
    id:                 number;
    position:           number;
    updated_at:         string;
    uploaded_at:        string;
    uploaded_by?:       number | null;
    url:                string;
}

/**
 * Style nested with colors and sizes, with some metadata fields excluded
 */
export interface NestedStyleSummary {
    colors: NestedColorSummary[];
    id:     number;
    name:   I18NString;
    number: string;
}

/**
 * Color with sizes included
 */
export interface NestedColorSummary {
    id:             number;
    name:           I18NString;
    number:         string;
    primary_image?: null | ImageSummary;
    sizes:          NestedSizeSummary[];
}

/**
 * Nested size (well, used by NestedColor, so `color` field isn't needed)
 */
export interface NestedSizeSummary {
    id:     number;
    name:   I18NString;
    number: string;
}

/**
 * Organization
 */
export interface Organization {
    created_at:  string;
    created_by?: number | null;
    id:          number;
    logo_url?:   null | string;
    name:        string;
    updated_at:  string;
}

/**
 * Price set (belonging to the same list and style, but with different start/end dates)
 */
export interface Price {
    amount:       number;
    created_at:   string;
    created_by?:  number | null;
    currency:     string;
    end:          string;
    external_id?: null | string;
    id:           number;
    list:         PriceListSummary;
    start:        string;
    style:        StyleSummary;
    type:         PriceType;
    uom?:         null | string;
    updated_at:   string;
}

/**
 * PriceList
 */
export interface PriceList {
    created_at:   string;
    created_by?:  number | null;
    external_id?: null | string;
    id:           number;
    name:         string;
    slug:         string;
    updated_at:   string;
}

/**
 * Size
 */
export interface Size {
    color:           ColorSummary;
    created_at:      string;
    created_by?:     number | null;
    delivery_period: string;
    ean_code?:       null | string;
    external_id?:    null | string;
    id:              number;
    name:            I18NString;
    number:          string;
    position:        number;
    service_item?:   boolean | null;
    slug:            string;
    status?:         null | string;
    updated_at:      string;
}

export interface SortByJsonSchema {
    nested_style: NestedStyleSortOrder;
    user:         UserSortOrder;
}

export enum NestedStyleSortOrder {
    DeliveryPeriodAsc = "delivery_period:asc",
    DeliveryPeriodDesc = "delivery_period:desc",
    NameAsc = "name:asc",
    NumberAsc = "number:asc",
}

export enum UserSortOrder {
    EmailAsc = "email:asc",
    LastSignInAsc = "last_sign_in:asc",
    LastSignInDesc = "last_sign_in:desc",
    NameAsc = "name:asc",
}

/**
 * Style
 */
export interface Style {
    attributes:         AttributeSummary[];
    categories:         Category[];
    core?:              boolean | null;
    country_of_origin?: null | string;
    created_at:         string;
    created_by?:        number | null;
    description:        I18NString;
    external_id?:       null | string;
    gross_weight:       number;
    id:                 number;
    name:               I18NString;
    net_weight:         number;
    number:             string;
    slug:               string;
    tariff_no?:         null | string;
    unit_volume:        number;
    updated_at:         string;
}

/**
 * Collection, for update
 */
export interface UpdateCollection {
    acronym?:     null | I18NString;
    external_id?: null | string;
    image?:       ImageSource | null;
    name?:        null | I18NString;
    new_colors?:  RefForColor[] | null;
    new_styles?:  RefForStyle[] | null;
    pricing?:     CollectionPricing[] | null;
    sizes?:       RefForSize[] | null;
    slug?:        null | string;
}

// Converts JSON types to/from your types
// and asserts the results at runtime
export class Convert {
    public static toApi(json: any): Api {
        return cast(json, r("Api"));
    }

    public static apiToJson(value: Api): any {
        return uncast(value, r("Api"));
    }

    public static toAttribute(json: any): Attribute {
        return cast(json, r("Attribute"));
    }

    public static attributeToJson(value: Attribute): any {
        return uncast(value, r("Attribute"));
    }

    public static toI18NString(json: any): I18NString {
        return cast(json, r("I18NString"));
    }

    public static i18NStringToJson(value: I18NString): any {
        return uncast(value, r("I18NString"));
    }

    public static toAttributeTypeSummary(json: any): AttributeTypeSummary {
        return cast(json, r("AttributeTypeSummary"));
    }

    public static attributeTypeSummaryToJson(value: AttributeTypeSummary): any {
        return uncast(value, r("AttributeTypeSummary"));
    }

    public static toAttributeType(json: any): AttributeType {
        return cast(json, r("AttributeType"));
    }

    public static attributeTypeToJson(value: AttributeType): any {
        return uncast(value, r("AttributeType"));
    }

    public static toAuthJsonSchema(json: any): AuthJsonSchema {
        return cast(json, r("AuthJsonSchema"));
    }

    public static authJsonSchemaToJson(value: AuthJsonSchema): any {
        return uncast(value, r("AuthJsonSchema"));
    }

    public static toAuthenticatedUser(json: any): AuthenticatedUser {
        return cast(json, r("AuthenticatedUser"));
    }

    public static authenticatedUserToJson(value: AuthenticatedUser): any {
        return uncast(value, r("AuthenticatedUser"));
    }

    public static toUser(json: any): User {
        return cast(json, r("User"));
    }

    public static userToJson(value: User): any {
        return uncast(value, r("User"));
    }

    public static toGroupSummary(json: any): GroupSummary {
        return cast(json, r("GroupSummary"));
    }

    public static groupSummaryToJson(value: GroupSummary): any {
        return uncast(value, r("GroupSummary"));
    }

    public static toUserOrganization(json: any): UserOrganization {
        return cast(json, r("UserOrganization"));
    }

    public static userOrganizationToJson(value: UserOrganization): any {
        return uncast(value, r("UserOrganization"));
    }

    public static toOrganizationSummary(json: any): OrganizationSummary {
        return cast(json, r("OrganizationSummary"));
    }

    public static organizationSummaryToJson(value: OrganizationSummary): any {
        return uncast(value, r("OrganizationSummary"));
    }

    public static toCreateGroup(json: any): CreateGroup {
        return cast(json, r("CreateGroup"));
    }

    public static createGroupToJson(value: CreateGroup): any {
        return uncast(value, r("CreateGroup"));
    }

    public static toRefForCollection(json: any): RefForCollection {
        return cast(json, r("RefForCollection"));
    }

    public static refForCollectionToJson(value: RefForCollection): any {
        return uncast(value, r("RefForCollection"));
    }

    public static toRefForPriceList(json: any): RefForPriceList {
        return cast(json, r("RefForPriceList"));
    }

    public static refForPriceListToJson(value: RefForPriceList): any {
        return uncast(value, r("RefForPriceList"));
    }

    public static toRefForUser(json: any): RefForUser {
        return cast(json, r("RefForUser"));
    }

    public static refForUserToJson(value: RefForUser): any {
        return uncast(value, r("RefForUser"));
    }

    public static toCreateUser(json: any): CreateUser {
        return cast(json, r("CreateUser"));
    }

    public static createUserToJson(value: CreateUser): any {
        return uncast(value, r("CreateUser"));
    }

    public static toRefForGroup(json: any): RefForGroup {
        return cast(json, r("RefForGroup"));
    }

    public static refForGroupToJson(value: RefForGroup): any {
        return uncast(value, r("RefForGroup"));
    }

    public static toGoogleCredentials(json: any): GoogleCredentials {
        return cast(json, r("GoogleCredentials"));
    }

    public static googleCredentialsToJson(value: GoogleCredentials): any {
        return uncast(value, r("GoogleCredentials"));
    }

    public static toGroup(json: any): Group {
        return cast(json, r("Group"));
    }

    public static groupToJson(value: Group): any {
        return uncast(value, r("Group"));
    }

    public static toCollectionSummary(json: any): CollectionSummary {
        return cast(json, r("CollectionSummary"));
    }

    public static collectionSummaryToJson(value: CollectionSummary): any {
        return uncast(value, r("CollectionSummary"));
    }

    public static toCollectionPricing(json: any): CollectionPricing {
        return cast(json, r("CollectionPricing"));
    }

    public static collectionPricingToJson(value: CollectionPricing): any {
        return uncast(value, r("CollectionPricing"));
    }

    public static toPriceListSummary(json: any): PriceListSummary {
        return cast(json, r("PriceListSummary"));
    }

    public static priceListSummaryToJson(value: PriceListSummary): any {
        return uncast(value, r("PriceListSummary"));
    }

    public static toUserSummary(json: any): UserSummary {
        return cast(json, r("UserSummary"));
    }

    public static userSummaryToJson(value: UserSummary): any {
        return uncast(value, r("UserSummary"));
    }

    public static toMicrosoftCredentials(json: any): MicrosoftCredentials {
        return cast(json, r("MicrosoftCredentials"));
    }

    public static microsoftCredentialsToJson(value: MicrosoftCredentials): any {
        return uncast(value, r("MicrosoftCredentials"));
    }

    public static toMicrosoftClaims(json: any): MicrosoftClaims {
        return cast(json, r("MicrosoftClaims"));
    }

    public static microsoftClaimsToJson(value: MicrosoftClaims): any {
        return uncast(value, r("MicrosoftClaims"));
    }

    public static toUpdateGroup(json: any): UpdateGroup {
        return cast(json, r("UpdateGroup"));
    }

    public static updateGroupToJson(value: UpdateGroup): any {
        return uncast(value, r("UpdateGroup"));
    }

    public static toUpdateOwnUser(json: any): UpdateOwnUser {
        return cast(json, r("UpdateOwnUser"));
    }

    public static updateOwnUserToJson(value: UpdateOwnUser): any {
        return uncast(value, r("UpdateOwnUser"));
    }

    public static toUpdateUser(json: any): UpdateUser {
        return cast(json, r("UpdateUser"));
    }

    public static updateUserToJson(value: UpdateUser): any {
        return uncast(value, r("UpdateUser"));
    }

    public static toCategory(json: any): Category {
        return cast(json, r("Category"));
    }

    public static categoryToJson(value: Category): any {
        return uncast(value, r("Category"));
    }

    public static toCategorySummary(json: any): CategorySummary {
        return cast(json, r("CategorySummary"));
    }

    public static categorySummaryToJson(value: CategorySummary): any {
        return uncast(value, r("CategorySummary"));
    }

    public static toCollection(json: any): Collection {
        return cast(json, r("Collection"));
    }

    public static collectionToJson(value: Collection): any {
        return uncast(value, r("Collection"));
    }

    public static toNestedSize(json: any): NestedSize {
        return cast(json, r("NestedSize"));
    }

    public static nestedSizeToJson(value: NestedSize): any {
        return uncast(value, r("NestedSize"));
    }

    public static toCollectionItem(json: any): CollectionItem {
        return cast(json, r("CollectionItem"));
    }

    public static collectionItemToJson(value: CollectionItem): any {
        return uncast(value, r("CollectionItem"));
    }

    public static toNestedStyle(json: any): NestedStyle {
        return cast(json, r("NestedStyle"));
    }

    public static nestedStyleToJson(value: NestedStyle): any {
        return uncast(value, r("NestedStyle"));
    }

    public static toAttributeSummary(json: any): AttributeSummary {
        return cast(json, r("AttributeSummary"));
    }

    public static attributeSummaryToJson(value: AttributeSummary): any {
        return uncast(value, r("AttributeSummary"));
    }

    public static toNestedColor(json: any): NestedColor {
        return cast(json, r("NestedColor"));
    }

    public static nestedColorToJson(value: NestedColor): any {
        return uncast(value, r("NestedColor"));
    }

    public static toImageSummary(json: any): ImageSummary {
        return cast(json, r("ImageSummary"));
    }

    public static imageSummaryToJson(value: ImageSummary): any {
        return uncast(value, r("ImageSummary"));
    }

    public static toNestedPrice(json: any): NestedPrice {
        return cast(json, r("NestedPrice"));
    }

    public static nestedPriceToJson(value: NestedPrice): any {
        return uncast(value, r("NestedPrice"));
    }

    public static toCollectionWithItems(json: any): CollectionWithItems {
        return cast(json, r("CollectionWithItems"));
    }

    public static collectionWithItemsToJson(value: CollectionWithItems): any {
        return uncast(value, r("CollectionWithItems"));
    }

    public static toColor(json: any): Color {
        return cast(json, r("Color"));
    }

    public static colorToJson(value: Color): any {
        return uncast(value, r("Color"));
    }

    public static toStyleSummary(json: any): StyleSummary {
        return cast(json, r("StyleSummary"));
    }

    public static styleSummaryToJson(value: StyleSummary): any {
        return uncast(value, r("StyleSummary"));
    }

    public static toColorSummary(json: any): ColorSummary {
        return cast(json, r("ColorSummary"));
    }

    public static colorSummaryToJson(value: ColorSummary): any {
        return uncast(value, r("ColorSummary"));
    }

    public static toCreateCollection(json: any): CreateCollection {
        return cast(json, r("CreateCollection"));
    }

    public static createCollectionToJson(value: CreateCollection): any {
        return uncast(value, r("CreateCollection"));
    }

    public static toImageSource(json: any): ImageSource {
        return cast(json, r("ImageSource"));
    }

    public static imageSourceToJson(value: ImageSource): any {
        return uncast(value, r("ImageSource"));
    }

    public static toRefForColor(json: any): RefForColor {
        return cast(json, r("RefForColor"));
    }

    public static refForColorToJson(value: RefForColor): any {
        return uncast(value, r("RefForColor"));
    }

    public static toRefForStyle(json: any): RefForStyle {
        return cast(json, r("RefForStyle"));
    }

    public static refForStyleToJson(value: RefForStyle): any {
        return uncast(value, r("RefForStyle"));
    }

    public static toRefForSize(json: any): RefForSize {
        return cast(json, r("RefForSize"));
    }

    public static refForSizeToJson(value: RefForSize): any {
        return uncast(value, r("RefForSize"));
    }

    public static toErrorsSchema(json: any): ErrorsSchema {
        return cast(json, r("ErrorsSchema"));
    }

    public static errorsSchemaToJson(value: ErrorsSchema): any {
        return uncast(value, r("ErrorsSchema"));
    }

    public static toApiErrorResponse(json: any): ApiErrorResponse {
        return cast(json, r("ApiErrorResponse"));
    }

    public static apiErrorResponseToJson(value: ApiErrorResponse): any {
        return uncast(value, r("ApiErrorResponse"));
    }

    public static toExportSchema(json: any): ExportSchema {
        return cast(json, r("ExportSchema"));
    }

    public static exportSchemaToJson(value: ExportSchema): any {
        return uncast(value, r("ExportSchema"));
    }

    public static toFiltersSchema(json: any): FiltersSchema {
        return cast(json, r("FiltersSchema"));
    }

    public static filtersSchemaToJson(value: FiltersSchema): any {
        return uncast(value, r("FiltersSchema"));
    }

    public static toCollectionFilters(json: any): CollectionFilters {
        return cast(json, r("CollectionFilters"));
    }

    public static collectionFiltersToJson(value: CollectionFilters): any {
        return uncast(value, r("CollectionFilters"));
    }

    public static toStyleFilters(json: any): StyleFilters {
        return cast(json, r("StyleFilters"));
    }

    public static styleFiltersToJson(value: StyleFilters): any {
        return uncast(value, r("StyleFilters"));
    }

    public static toRefForCategory(json: any): RefForCategory {
        return cast(json, r("RefForCategory"));
    }

    public static refForCategoryToJson(value: RefForCategory): any {
        return uncast(value, r("RefForCategory"));
    }

    public static toUserFilters(json: any): UserFilters {
        return cast(json, r("UserFilters"));
    }

    public static userFiltersToJson(value: UserFilters): any {
        return uncast(value, r("UserFilters"));
    }

    public static toImage(json: any): Image {
        return cast(json, r("Image"));
    }

    public static imageToJson(value: Image): any {
        return uncast(value, r("Image"));
    }

    public static toNestedStyleSummary(json: any): NestedStyleSummary {
        return cast(json, r("NestedStyleSummary"));
    }

    public static nestedStyleSummaryToJson(value: NestedStyleSummary): any {
        return uncast(value, r("NestedStyleSummary"));
    }

    public static toNestedColorSummary(json: any): NestedColorSummary {
        return cast(json, r("NestedColorSummary"));
    }

    public static nestedColorSummaryToJson(value: NestedColorSummary): any {
        return uncast(value, r("NestedColorSummary"));
    }

    public static toNestedSizeSummary(json: any): NestedSizeSummary {
        return cast(json, r("NestedSizeSummary"));
    }

    public static nestedSizeSummaryToJson(value: NestedSizeSummary): any {
        return uncast(value, r("NestedSizeSummary"));
    }

    public static toOrganization(json: any): Organization {
        return cast(json, r("Organization"));
    }

    public static organizationToJson(value: Organization): any {
        return uncast(value, r("Organization"));
    }

    public static toPrice(json: any): Price {
        return cast(json, r("Price"));
    }

    public static priceToJson(value: Price): any {
        return uncast(value, r("Price"));
    }

    public static toPriceList(json: any): PriceList {
        return cast(json, r("PriceList"));
    }

    public static priceListToJson(value: PriceList): any {
        return uncast(value, r("PriceList"));
    }

    public static toSize(json: any): Size {
        return cast(json, r("Size"));
    }

    public static sizeToJson(value: Size): any {
        return uncast(value, r("Size"));
    }

    public static toSortByJsonSchema(json: any): SortByJsonSchema {
        return cast(json, r("SortByJsonSchema"));
    }

    public static sortByJsonSchemaToJson(value: SortByJsonSchema): any {
        return uncast(value, r("SortByJsonSchema"));
    }

    public static toStyle(json: any): Style {
        return cast(json, r("Style"));
    }

    public static styleToJson(value: Style): any {
        return uncast(value, r("Style"));
    }

    public static toUpdateCollection(json: any): UpdateCollection {
        return cast(json, r("UpdateCollection"));
    }

    public static updateCollectionToJson(value: UpdateCollection): any {
        return uncast(value, r("UpdateCollection"));
    }
}

function invalidValue(typ: any, val: any, key: any = ''): never {
    if (key) {
        throw Error(`Invalid value for key "${key}". Expected type ${JSON.stringify(typ)} but got ${JSON.stringify(val)}`);
    }
    throw Error(`Invalid value ${JSON.stringify(val)} for type ${JSON.stringify(typ)}`, );
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) {}
        }
        return invalidValue(typs, val);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases, val);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue("array", val);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue("Date", val);
        }
        return d;
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue("object", val);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, prop.key);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                result[key] = transform(val[key], additional, getProps, key);
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val);
    }
    if (typ === false) return invalidValue(typ, val);
    while (typeof typ === "object" && typ.ref !== undefined) {
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems")    ? transformArray(typ.arrayItems, val)
            : typ.hasOwnProperty("props")         ? transformObject(getProps(typ), typ.additional, val)
            : invalidValue(typ, val);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
    return transformPrimitive(typ, val);
}

function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "Api": o([
        { json: "attribute", js: "attribute", typ: r("Attribute") },
        { json: "attribute_type", js: "attribute_type", typ: r("AttributeType") },
        { json: "attribute_type_summary", js: "attribute_type_summary", typ: r("AttributeTypeSummary") },
        { json: "auth", js: "auth", typ: r("AuthJsonSchema") },
        { json: "category", js: "category", typ: r("Category") },
        { json: "category_summary", js: "category_summary", typ: r("CategorySummary") },
        { json: "collection", js: "collection", typ: r("Collection") },
        { json: "collection_style_item", js: "collection_style_item", typ: r("CollectionItem") },
        { json: "collection_summary", js: "collection_summary", typ: r("CollectionSummary") },
        { json: "collection_with_items", js: "collection_with_items", typ: r("CollectionWithItems") },
        { json: "color", js: "color", typ: r("Color") },
        { json: "color_summary", js: "color_summary", typ: r("ColorSummary") },
        { json: "create_collection", js: "create_collection", typ: r("CreateCollection") },
        { json: "environment", js: "environment", typ: r("Environment") },
        { json: "errors", js: "errors", typ: r("ErrorsSchema") },
        { json: "export", js: "export", typ: r("ExportSchema") },
        { json: "filters", js: "filters", typ: r("FiltersSchema") },
        { json: "i18n_string", js: "i18n_string", typ: r("I18NString") },
        { json: "image", js: "image", typ: r("Image") },
        { json: "image_summary", js: "image_summary", typ: r("ImageSummary") },
        { json: "nested_attribute", js: "nested_attribute", typ: r("AttributeSummary") },
        { json: "nested_color", js: "nested_color", typ: r("NestedColor") },
        { json: "nested_price", js: "nested_price", typ: r("NestedPrice") },
        { json: "nested_size", js: "nested_size", typ: r("NestedSize") },
        { json: "nested_style", js: "nested_style", typ: r("NestedStyle") },
        { json: "nested_style_summary", js: "nested_style_summary", typ: r("NestedStyleSummary") },
        { json: "organization", js: "organization", typ: r("Organization") },
        { json: "price", js: "price", typ: r("Price") },
        { json: "price_list", js: "price_list", typ: r("PriceList") },
        { json: "price_list_summary", js: "price_list_summary", typ: r("PriceListSummary") },
        { json: "size", js: "size", typ: r("Size") },
        { json: "sort_by", js: "sort_by", typ: r("SortByJsonSchema") },
        { json: "style", js: "style", typ: r("Style") },
        { json: "style_summary", js: "style_summary", typ: r("StyleSummary") },
        { json: "update_collection", js: "update_collection", typ: r("UpdateCollection") },
    ], "any"),
    "Attribute": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "description", js: "description", typ: r("I18NString") },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "slug", js: "slug", typ: "" },
        { json: "title", js: "title", typ: r("I18NString") },
        { json: "type", js: "type", typ: r("AttributeTypeSummary") },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "I18NString": o([
        { json: "en", js: "en", typ: u(undefined, "") },
        { json: "sv", js: "sv", typ: u(undefined, "") },
    ], "any"),
    "AttributeTypeSummary": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "slug", js: "slug", typ: "" },
    ], "any"),
    "AttributeType": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "AuthJsonSchema": o([
        { json: "authenticated_user", js: "authenticated_user", typ: r("AuthenticatedUser") },
        { json: "create_group", js: "create_group", typ: r("CreateGroup") },
        { json: "create_user", js: "create_user", typ: r("CreateUser") },
        { json: "google_credentials", js: "google_credentials", typ: r("GoogleCredentials") },
        { json: "group", js: "group", typ: r("Group") },
        { json: "group_summary", js: "group_summary", typ: r("GroupSummary") },
        { json: "microsoft_credentials", js: "microsoft_credentials", typ: r("MicrosoftCredentials") },
        { json: "update_group", js: "update_group", typ: r("UpdateGroup") },
        { json: "update_own_user", js: "update_own_user", typ: r("UpdateOwnUser") },
        { json: "update_user", js: "update_user", typ: r("UpdateUser") },
        { json: "user", js: "user", typ: r("User") },
        { json: "user_summary", js: "user_summary", typ: r("UserSummary") },
    ], "any"),
    "AuthenticatedUser": o([
        { json: "environment", js: "environment", typ: r("Environment") },
        { json: "token", js: "token", typ: "" },
        { json: "user", js: "user", typ: r("User") },
    ], "any"),
    "User": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "email", js: "email", typ: "" },
        { json: "groups", js: "groups", typ: a(r("GroupSummary")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "last_sign_in", js: "last_sign_in", typ: "" },
        { json: "name", js: "name", typ: "" },
        { json: "organizations", js: "organizations", typ: a(r("UserOrganization")) },
        { json: "profile_image", js: "profile_image", typ: u(undefined, u(null, "")) },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "GroupSummary": o([
        { json: "description", js: "description", typ: "" },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: "" },
        { json: "num_collections", js: "num_collections", typ: 0 },
        { json: "num_price_lists", js: "num_price_lists", typ: 0 },
        { json: "num_users", js: "num_users", typ: 0 },
        { json: "slug", js: "slug", typ: "" },
    ], "any"),
    "UserOrganization": o([
        { json: "organization", js: "organization", typ: r("OrganizationSummary") },
        { json: "roles", js: "roles", typ: a(r("Role")) },
    ], "any"),
    "OrganizationSummary": o([
        { json: "id", js: "id", typ: 3.14 },
        { json: "logo_url", js: "logo_url", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: "" },
    ], "any"),
    "CreateGroup": o([
        { json: "collections", js: "collections", typ: u(undefined, a(r("RefForCollection"))) },
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: "" },
        { json: "price_lists", js: "price_lists", typ: u(undefined, a(r("RefForPriceList"))) },
        { json: "slug", js: "slug", typ: u(undefined, u(null, "")) },
        { json: "users", js: "users", typ: u(undefined, a(r("RefForUser"))) },
    ], "any"),
    "RefForCollection": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "RefForPriceList": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "RefForUser": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "CreateUser": o([
        { json: "email", js: "email", typ: "" },
        { json: "groups", js: "groups", typ: u(undefined, u(a(r("RefForGroup")), null)) },
        { json: "name", js: "name", typ: "" },
        { json: "password", js: "password", typ: u(undefined, u(null, "")) },
        { json: "profile_image", js: "profile_image", typ: u(undefined, u(null, "")) },
        { json: "roles", js: "roles", typ: u(undefined, u(a(r("Role")), null)) },
    ], "any"),
    "RefForGroup": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "GoogleCredentials": o([
        { json: "idToken", js: "idToken", typ: "" },
    ], "any"),
    "Group": o([
        { json: "collections", js: "collections", typ: a(r("CollectionSummary")) },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "description", js: "description", typ: "" },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: "" },
        { json: "price_lists", js: "price_lists", typ: a(r("PriceListSummary")) },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
        { json: "users", js: "users", typ: a(r("UserSummary")) },
    ], "any"),
    "CollectionSummary": o([
        { json: "acronym", js: "acronym", typ: r("I18NString") },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "image_url", js: "image_url", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "num_colors", js: "num_colors", typ: 0 },
        { json: "num_sizes", js: "num_sizes", typ: 0 },
        { json: "num_styles", js: "num_styles", typ: 0 },
        { json: "pricing", js: "pricing", typ: a(r("CollectionPricing")) },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "CollectionPricing": o([
        { json: "date", js: "date", typ: "" },
        { json: "list", js: "list", typ: r("PriceListSummary") },
    ], "any"),
    "PriceListSummary": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: "" },
        { json: "slug", js: "slug", typ: "" },
    ], "any"),
    "UserSummary": o([
        { json: "email", js: "email", typ: "" },
        { json: "id", js: "id", typ: 3.14 },
        { json: "last_sign_in", js: "last_sign_in", typ: "" },
        { json: "name", js: "name", typ: "" },
        { json: "profile_image", js: "profile_image", typ: u(undefined, u(null, "")) },
    ], "any"),
    "MicrosoftCredentials": o([
        { json: "accessToken", js: "accessToken", typ: "" },
        { json: "idToken", js: "idToken", typ: "" },
        { json: "idTokenClaims", js: "idTokenClaims", typ: r("MicrosoftClaims") },
    ], "any"),
    "MicrosoftClaims": o([
        { json: "email", js: "email", typ: "" },
        { json: "name", js: "name", typ: "" },
    ], "any"),
    "UpdateGroup": o([
        { json: "collections", js: "collections", typ: u(undefined, u(a(r("RefForCollection")), null)) },
        { json: "description", js: "description", typ: u(undefined, u(null, "")) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: u(undefined, u(null, "")) },
        { json: "price_lists", js: "price_lists", typ: u(undefined, u(a(r("RefForPriceList")), null)) },
        { json: "slug", js: "slug", typ: u(undefined, u(null, "")) },
        { json: "users", js: "users", typ: u(undefined, u(a(r("RefForUser")), null)) },
    ], "any"),
    "UpdateOwnUser": o([
        { json: "email", js: "email", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: u(undefined, u(null, "")) },
        { json: "password", js: "password", typ: u(undefined, u(null, "")) },
        { json: "profile_image", js: "profile_image", typ: u(undefined, u(null, "")) },
    ], "any"),
    "UpdateUser": o([
        { json: "email", js: "email", typ: u(undefined, u(null, "")) },
        { json: "groups", js: "groups", typ: u(undefined, u(a(r("RefForGroup")), null)) },
        { json: "name", js: "name", typ: u(undefined, u(null, "")) },
        { json: "password", js: "password", typ: u(undefined, u(null, "")) },
        { json: "profile_image", js: "profile_image", typ: u(undefined, u(null, "")) },
        { json: "roles", js: "roles", typ: u(undefined, u(a(r("Role")), null)) },
    ], "any"),
    "Category": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "CategorySummary": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "slug", js: "slug", typ: "" },
    ], "any"),
    "Collection": o([
        { json: "acronym", js: "acronym", typ: r("I18NString") },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "image_url", js: "image_url", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "pricing", js: "pricing", typ: a(r("CollectionPricing")) },
        { json: "sizes", js: "sizes", typ: a(r("NestedSize")) },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "NestedSize": o([
        { json: "delivery_period", js: "delivery_period", typ: u(undefined, "") },
        { json: "ean_code", js: "ean_code", typ: u(undefined, u(null, "")) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "position", js: "position", typ: 0 },
        { json: "service_item", js: "service_item", typ: u(undefined, u(true, null)) },
        { json: "slug", js: "slug", typ: "" },
        { json: "status", js: "status", typ: u(undefined, u(null, "")) },
    ], "any"),
    "CollectionItem": o([
        { json: "style", js: "style", typ: r("NestedStyle") },
        { json: "user_comment", js: "user_comment", typ: "" },
    ], "any"),
    "NestedStyle": o([
        { json: "attributes", js: "attributes", typ: a(r("AttributeSummary")) },
        { json: "categories", js: "categories", typ: a(r("CategorySummary")) },
        { json: "colors", js: "colors", typ: a(r("NestedColor")) },
        { json: "core", js: "core", typ: u(undefined, u(true, null)) },
        { json: "country_of_origin", js: "country_of_origin", typ: u(undefined, u(null, "")) },
        { json: "description", js: "description", typ: r("I18NString") },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "gross_weight", js: "gross_weight", typ: 3.14 },
        { json: "id", js: "id", typ: 3.14 },
        { json: "is_new", js: "is_new", typ: u(undefined, u(true, null)) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "net_weight", js: "net_weight", typ: 3.14 },
        { json: "number", js: "number", typ: "" },
        { json: "prices", js: "prices", typ: a(r("NestedPrice")) },
        { json: "slug", js: "slug", typ: "" },
        { json: "tariff_no", js: "tariff_no", typ: u(undefined, u(null, "")) },
        { json: "unit_volume", js: "unit_volume", typ: 3.14 },
    ], "any"),
    "AttributeSummary": o([
        { json: "description", js: "description", typ: r("I18NString") },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "slug", js: "slug", typ: "" },
        { json: "title", js: "title", typ: r("I18NString") },
        { json: "type", js: "type", typ: r("AttributeTypeSummary") },
    ], "any"),
    "NestedColor": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "images", js: "images", typ: a(r("ImageSummary")) },
        { json: "is_new", js: "is_new", typ: u(undefined, u(true, null)) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "sizes", js: "sizes", typ: a(r("NestedSize")) },
        { json: "slug", js: "slug", typ: "" },
    ], "any"),
    "ImageSummary": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "url", js: "url", typ: "" },
    ], "any"),
    "NestedPrice": o([
        { json: "amount", js: "amount", typ: 3.14 },
        { json: "currency", js: "currency", typ: "" },
        { json: "end", js: "end", typ: "" },
        { json: "id", js: "id", typ: 3.14 },
        { json: "list", js: "list", typ: r("PriceListSummary") },
        { json: "start", js: "start", typ: "" },
        { json: "type", js: "type", typ: r("PriceType") },
        { json: "uom", js: "uom", typ: u(undefined, u(null, "")) },
    ], "any"),
    "CollectionWithItems": o([
        { json: "acronym", js: "acronym", typ: r("I18NString") },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "image_url", js: "image_url", typ: u(undefined, u(null, "")) },
        { json: "items", js: "items", typ: a(r("CollectionItem")) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "num_colors", js: "num_colors", typ: 0 },
        { json: "num_sizes", js: "num_sizes", typ: 0 },
        { json: "num_styles", js: "num_styles", typ: 0 },
        { json: "pricing", js: "pricing", typ: a(r("CollectionPricing")) },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "Color": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "images", js: "images", typ: a(r("ImageSummary")) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "slug", js: "slug", typ: "" },
        { json: "style", js: "style", typ: r("StyleSummary") },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "StyleSummary": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "slug", js: "slug", typ: "" },
    ], "any"),
    "ColorSummary": o([
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "slug", js: "slug", typ: "" },
        { json: "style", js: "style", typ: r("StyleSummary") },
    ], "any"),
    "CreateCollection": o([
        { json: "acronym", js: "acronym", typ: r("I18NString") },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "image", js: "image", typ: u(undefined, u(r("ImageSource"), null)) },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "new_colors", js: "new_colors", typ: a(r("RefForColor")) },
        { json: "new_styles", js: "new_styles", typ: a(r("RefForStyle")) },
        { json: "pricing", js: "pricing", typ: a(r("CollectionPricing")) },
        { json: "sizes", js: "sizes", typ: u(undefined, a(r("RefForSize"))) },
        { json: "slug", js: "slug", typ: u(undefined, u(null, "")) },
    ], "any"),
    "ImageSource": o([
        { json: "url", js: "url", typ: u(undefined, "") },
        { json: "bytes", js: "bytes", typ: u(undefined, a(0)) },
        { json: "base64", js: "base64", typ: u(undefined, "") },
    ], false),
    "RefForColor": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "RefForStyle": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "RefForSize": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "ErrorsSchema": o([
        { json: "code", js: "code", typ: r("ApiErrorCode") },
        { json: "response", js: "response", typ: r("ApiErrorResponse") },
    ], "any"),
    "ApiErrorResponse": o([
        { json: "error_code", js: "error_code", typ: r("ApiErrorCode") },
        { json: "error_message", js: "error_message", typ: "" },
    ], "any"),
    "ExportSchema": o([
        { json: "field", js: "field", typ: r("ExportField") },
        { json: "format", js: "format", typ: r("ExportFormat") },
        { json: "group_by", js: "group_by", typ: r("GroupBy") },
    ], "any"),
    "FiltersSchema": o([
        { json: "collection", js: "collection", typ: r("CollectionFilters") },
        { json: "style", js: "style", typ: r("StyleFilters") },
        { json: "user", js: "user", typ: r("UserFilters") },
    ], "any"),
    "CollectionFilters": o([
        { json: "styles", js: "styles", typ: u(undefined, r("StyleFilters")) },
    ], "any"),
    "StyleFilters": o([
        { json: "categories", js: "categories", typ: u(undefined, u(a(r("RefForCategory")), null)) },
        { json: "core", js: "core", typ: u(undefined, u(true, null)) },
        { json: "country_of_origin", js: "country_of_origin", typ: u(undefined, u(a(""), null)) },
        { json: "new_colors", js: "new_colors", typ: u(undefined, u(true, null)) },
        { json: "new_styles", js: "new_styles", typ: u(undefined, u(true, null)) },
        { json: "numbers", js: "numbers", typ: u(undefined, u(a(""), null)) },
        { json: "pricelists", js: "pricelists", typ: u(undefined, u(a(r("RefForPriceList")), null)) },
        { json: "refs", js: "refs", typ: u(undefined, u(a(r("RefForStyle")), null)) },
        { json: "service_item", js: "service_item", typ: u(undefined, u(true, null)) },
        { json: "status", js: "status", typ: u(undefined, u(a(""), null)) },
    ], "any"),
    "RefForCategory": o([
        { json: "id", js: "id", typ: u(undefined, 3.14) },
        { json: "external_id", js: "external_id", typ: u(undefined, "") },
        { json: "slug", js: "slug", typ: u(undefined, "") },
    ], false),
    "UserFilters": o([
        { json: "groups", js: "groups", typ: u(undefined, u(a(r("RefForGroup")), null)) },
        { json: "roles", js: "roles", typ: u(undefined, u(a(r("Role")), null)) },
    ], "any"),
    "Image": o([
        { json: "color", js: "color", typ: r("ColorSummary") },
        { json: "external_checksum", js: "external_checksum", typ: u(undefined, u(null, "")) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "position", js: "position", typ: 0 },
        { json: "updated_at", js: "updated_at", typ: "" },
        { json: "uploaded_at", js: "uploaded_at", typ: "" },
        { json: "uploaded_by", js: "uploaded_by", typ: u(undefined, u(3.14, null)) },
        { json: "url", js: "url", typ: "" },
    ], "any"),
    "NestedStyleSummary": o([
        { json: "colors", js: "colors", typ: a(r("NestedColorSummary")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
    ], "any"),
    "NestedColorSummary": o([
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "primary_image", js: "primary_image", typ: u(undefined, u(null, r("ImageSummary"))) },
        { json: "sizes", js: "sizes", typ: a(r("NestedSizeSummary")) },
    ], "any"),
    "NestedSizeSummary": o([
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
    ], "any"),
    "Organization": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "logo_url", js: "logo_url", typ: u(undefined, u(null, "")) },
        { json: "name", js: "name", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "Price": o([
        { json: "amount", js: "amount", typ: 3.14 },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "currency", js: "currency", typ: "" },
        { json: "end", js: "end", typ: "" },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "list", js: "list", typ: r("PriceListSummary") },
        { json: "start", js: "start", typ: "" },
        { json: "style", js: "style", typ: r("StyleSummary") },
        { json: "type", js: "type", typ: r("PriceType") },
        { json: "uom", js: "uom", typ: u(undefined, u(null, "")) },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "PriceList": o([
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: "" },
        { json: "slug", js: "slug", typ: "" },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "Size": o([
        { json: "color", js: "color", typ: r("ColorSummary") },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "delivery_period", js: "delivery_period", typ: "" },
        { json: "ean_code", js: "ean_code", typ: u(undefined, u(null, "")) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "number", js: "number", typ: "" },
        { json: "position", js: "position", typ: 0 },
        { json: "service_item", js: "service_item", typ: u(undefined, u(true, null)) },
        { json: "slug", js: "slug", typ: "" },
        { json: "status", js: "status", typ: u(undefined, u(null, "")) },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "SortByJsonSchema": o([
        { json: "nested_style", js: "nested_style", typ: r("NestedStyleSortOrder") },
        { json: "user", js: "user", typ: r("UserSortOrder") },
    ], "any"),
    "Style": o([
        { json: "attributes", js: "attributes", typ: a(r("AttributeSummary")) },
        { json: "categories", js: "categories", typ: a(r("Category")) },
        { json: "core", js: "core", typ: u(undefined, u(true, null)) },
        { json: "country_of_origin", js: "country_of_origin", typ: u(undefined, u(null, "")) },
        { json: "created_at", js: "created_at", typ: "" },
        { json: "created_by", js: "created_by", typ: u(undefined, u(3.14, null)) },
        { json: "description", js: "description", typ: r("I18NString") },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "gross_weight", js: "gross_weight", typ: 3.14 },
        { json: "id", js: "id", typ: 3.14 },
        { json: "name", js: "name", typ: r("I18NString") },
        { json: "net_weight", js: "net_weight", typ: 3.14 },
        { json: "number", js: "number", typ: "" },
        { json: "slug", js: "slug", typ: "" },
        { json: "tariff_no", js: "tariff_no", typ: u(undefined, u(null, "")) },
        { json: "unit_volume", js: "unit_volume", typ: 3.14 },
        { json: "updated_at", js: "updated_at", typ: "" },
    ], "any"),
    "UpdateCollection": o([
        { json: "acronym", js: "acronym", typ: u(undefined, u(null, r("I18NString"))) },
        { json: "external_id", js: "external_id", typ: u(undefined, u(null, "")) },
        { json: "image", js: "image", typ: u(undefined, u(r("ImageSource"), null)) },
        { json: "name", js: "name", typ: u(undefined, u(null, r("I18NString"))) },
        { json: "new_colors", js: "new_colors", typ: u(undefined, u(a(r("RefForColor")), null)) },
        { json: "new_styles", js: "new_styles", typ: u(undefined, u(a(r("RefForStyle")), null)) },
        { json: "pricing", js: "pricing", typ: u(undefined, u(a(r("CollectionPricing")), null)) },
        { json: "sizes", js: "sizes", typ: u(undefined, u(a(r("RefForSize")), null)) },
        { json: "slug", js: "slug", typ: u(undefined, u(null, "")) },
    ], "any"),
    "Environment": [
        "development",
        "production",
        "staging",
    ],
    "Role": [
        "Active",
        "Administrator",
        "Editor",
        "Viewer",
    ],
    "PriceType": [
        "Retail",
        "Unit",
    ],
    "ApiErrorCode": [
        "ApplicationNotReady",
        "Base64DecodeError",
        "CsvError",
        "DbBuildError",
        "DbCreatePoolError",
        "DbError",
        "DbPoolError",
        "EmptySlugDisallowed",
        "ExpiredToken",
        "ExplicitIdCreationDisallowed",
        "ExternalIdAlreadyExists",
        "ExternalIdNotFound",
        "ExternalIdReferenceUnsupported",
        "ExternalRequestError",
        "FailedPasswordHashing",
        "FailedPasswordValidation",
        "IdAlreadyExists",
        "IdNotFound",
        "ImageAlreadyExists",
        "ImageBackendMisconfigured",
        "ImageUploadsUnavailable",
        "InvalidDbRoleId",
        "InvalidEntityRef",
        "InvalidHttpHeaderValue",
        "InvalidToken",
        "InvalidUserCredentials",
        "IoError",
        "JsonError",
        "MissingEntityRefPathParameter",
        "MissingPermissions",
        "PathJsonError",
        "PathRejection",
        "QueryParsingError",
        "QueryRejection",
        "SlugAlreadyExists",
        "SlugNotFound",
        "SlugReferenceUnsupported",
        "UnverifiedEmail",
        "UrlParseError",
        "UserEmailAlreadyExists",
        "UserEmailNotFound",
        "XlsxError",
    ],
    "ExportField": [
        "attribute",
        "category_name",
        "color_external_id",
        "color_name",
        "color_number",
        "core",
        "country_of_origin",
        "delivery_period",
        "ean_code",
        "gross_weight",
        "images",
        "new_color",
        "new_style",
        "primary_image",
        "retail_price_amount",
        "retail_price_currency",
        "retail_price_list",
        "service_item",
        "size_number",
        "size_type",
        "style_description",
        "style_external_id",
        "style_name",
        "style_number",
        "tariff_no",
        "unit_price_amount",
        "unit_price_currency",
        "unit_price_list",
        "unit_volume",
    ],
    "ExportFormat": [
        "csv",
        "json",
        "xlsx",
    ],
    "GroupBy": [
        "category",
        "color",
        "image",
        "price_list",
        "size",
        "style",
    ],
    "NestedStyleSortOrder": [
        "delivery_period:asc",
        "delivery_period:desc",
        "name:asc",
        "number:asc",
    ],
    "UserSortOrder": [
        "email:asc",
        "last_sign_in:asc",
        "last_sign_in:desc",
        "name:asc",
    ],
};
