-- Admin related queries
--
-- Entities:
--
--: StringFilterChoiceRow()
--
--: EntityFilterChoiceRow(subtitle?, image?)
--
--: AttributeTableData(created_by?, external_id?)
--: AttributetypeTableData(created_by?, external_id?)
--: CategoryTableData(created_by?, external_id?)
--: CollectionPricelistTableData(created_by?)
--: CollectionTableData(created_by?, external_id?, image_url?)
--: ColorTableData(created_by?, external_id?)
--: GroupCollectionTableData()
--: GroupPricelistTableData()
--: GroupTableData(created_by?, external_id?)
--: GroupUserTableData()
--: ImageTableData(external_checksum?, external_id?, position?, uploaded_by?)
--: NewCollectionColorTableData()
--: NewCollectionStyleTableData()
--: OrganizationTableData(created_by?, logo_url?)
--: PricelistTableData(created_by?, external_id?)
--: PriceTableData(created_by?, external_id?, uom?)
--: SizeCollectionTableData()
--: SizeTableData(created_by?, external_id?, delivery_period?, ean_code?, service_item?, status?)
--: StyleAttributeTableData()
--: StyleCategoryTableData()
--: StyleTableData(created_by?, external_id?, core?, country_of_origin?, tariff_no?)
--: UserOrganizationTableData()
--: UserTableData(last_sign_in?, password_hash?, profile_image?)
--
--
-- Queries:
--
--! select_style_filter_choices : EntityFilterChoiceRow
SELECT
    style.id,
    style."name" AS title,
    json_build_object('en', style."number") AS subtitle,
    to_json(main_image.json_data) AS image
FROM style
LEFT JOIN (
    SELECT
        color.style_id,
        row_number()
        OVER (PARTITION BY color.style_id ORDER BY image.uploaded_at DESC)
        AS rowno,
        jsonb_build_object(
            'id',
            image.id,
            'external_id',
            image.external_id,
            'url',
            image.url
        ) AS json_data
    FROM color
    INNER JOIN image ON image.color_id = color.id
    WHERE image.position = 1
) AS main_image ON main_image.style_id = style.id AND main_image.rowno = 1
WHERE style.organization_id = :organization_id
ORDER BY title;

--! select_category_filter_choices : EntityFilterChoiceRow
SELECT
    category.id,
    category."name" AS title,
    NULL::json AS subtitle,
    NULL::json AS image
FROM category WHERE category.organization_id = :organization_id
ORDER BY title;


--! select_status_filter_choices : StringFilterChoiceRow
SELECT DISTINCT size.status AS title FROM size
WHERE size.organization_id = :organization_id
ORDER BY title;

--! select_user_org_data: UserTableData
SELECT "user".*
FROM "user"
INNER JOIN user_organization ON user_organization.user_id = "user".id
WHERE user_organization.organization_id = :organization_id;

--! select_organization_org_data: OrganizationTableData
SELECT organization.*
FROM organization WHERE organization.id = :organization_id;

--! select_group_org_data: GroupTableData
SELECT "group".*
FROM "group" WHERE "group".organization_id = :organization_id;

--! select_attributetype_org_data: AttributetypeTableData
SELECT attributetype.*
FROM attributetype WHERE attributetype.organization_id = :organization_id;

--! select_attribute_org_data: AttributeTableData
SELECT "attribute".*
FROM "attribute" WHERE "attribute".organization_id = :organization_id;

--! select_category_org_data: CategoryTableData
SELECT category.*
FROM category WHERE category.organization_id = :organization_id;

--! select_collection_org_data: CollectionTableData
SELECT collection.*
FROM collection WHERE collection.organization_id = :organization_id;

--! select_style_org_data: StyleTableData
SELECT style.*
FROM style WHERE style.organization_id = :organization_id;

--! select_color_org_data: ColorTableData
SELECT color.*
FROM color WHERE color.organization_id = :organization_id;

--! select_size_org_data: SizeTableData
SELECT size.*
FROM size WHERE size.organization_id = :organization_id;

--! select_image_org_data: ImageTableData
SELECT image.*
FROM image WHERE image.organization_id = :organization_id;

--! select_pricelist_org_data: PricelistTableData
SELECT pricelist.*
FROM pricelist WHERE pricelist.organization_id = :organization_id;

--! select_price_org_data: PriceTableData
SELECT price.*
FROM price WHERE price.organization_id = :organization_id;

--! select_user_organization_org_data: UserOrganizationTableData
SELECT user_organization.*
FROM user_organization
WHERE user_organization.organization_id = :organization_id;

--! select_collection_pricelist_org_data: CollectionPricelistTableData
SELECT collection_pricelist.*
FROM collection_pricelist
INNER JOIN collection ON collection.id = collection_pricelist.collection_id
WHERE collection.organization_id = :organization_id;

--! select_group_collection_org_data: GroupCollectionTableData
SELECT group_collection.*
FROM group_collection
INNER JOIN "group" ON "group".id = group_collection.group_id
WHERE "group".organization_id = :organization_id;

--! select_group_pricelist_org_data: GroupPricelistTableData
SELECT group_pricelist.*
FROM group_pricelist
INNER JOIN "group" ON "group".id = group_pricelist.group_id
WHERE "group".organization_id = :organization_id;

--! select_group_user_org_data: GroupUserTableData
SELECT group_user.*
FROM group_user
INNER JOIN "group" ON "group".id = group_user.group_id
WHERE "group".organization_id = :organization_id;

--! select_new_collection_style_org_data: NewCollectionStyleTableData
SELECT new_collection_style.*
FROM new_collection_style
INNER JOIN collection ON collection.id = new_collection_style.collection_id
WHERE collection.organization_id = :organization_id;

--! select_new_collection_color_org_data: NewCollectionColorTableData
SELECT new_collection_color.*
FROM new_collection_color
INNER JOIN collection ON collection.id = new_collection_color.collection_id
WHERE collection.organization_id = :organization_id;

--! select_size_collection_org_data: SizeCollectionTableData
SELECT size_collection.*
FROM size_collection
INNER JOIN collection ON collection.id = size_collection.collection_id
WHERE collection.organization_id = :organization_id;

--! select_style_attribute_org_data: StyleAttributeTableData
SELECT style_attribute.*
FROM style_attribute
INNER JOIN "attribute" ON "attribute".id = style_attribute.attribute_id
WHERE "attribute".organization_id = :organization_id;

--! select_style_category_org_data: StyleCategoryTableData
SELECT style_category.*
FROM style_category
INNER JOIN category ON category.id = style_category.category_id
WHERE category.organization_id = :organization_id;

-- TODO: Remove if it will not be used!
-- --! select_organization_data : OrganizationData
-- SELECT json_build_object(
--     'user', (
--         SELECT json_agg(row_to_json("user".*))
--         FROM "user"
--         INNER JOIN user_organization ON user_organization.user_id = "user".id
--         WHERE user_organization.organization_id = :organization_id
--     ),
--     'organization', (
--         SELECT json_agg(row_to_json(organization.*))
--         FROM organization WHERE organization.id = :organization_id
--     ),
--     'group', (
--         SELECT json_agg(row_to_json("group".*))
--         FROM "group" WHERE "group".organization_id = :organization_id
--     ),
--     'attributetype', (
--         SELECT json_agg(row_to_json(attributetype.*))
--         FROM attributetype WHERE attributetype.organization_id = :organization_id
--     ),
--     'attribute', (
--         SELECT json_agg(row_to_json("attribute".*))
--         FROM "attribute" WHERE "attribute".organization_id = :organization_id
--     ),
--     'category', (
--         SELECT json_agg(row_to_json(category.*))
--         FROM category WHERE category.organization_id = :organization_id
--     ),
--     'collection', (
--         SELECT json_agg(row_to_json(collection.*))
--         FROM collection WHERE collection.organization_id = :organization_id
--     ),
--     'style', (
--         SELECT json_agg(row_to_json(style.*))
--         FROM style WHERE style.organization_id = :organization_id
--     ),
--     'color', (
--         SELECT json_agg(row_to_json(color.*))
--         FROM color WHERE color.organization_id = :organization_id
--     ),
--     'size', (
--         SELECT json_agg(row_to_json(size.*))
--         FROM size WHERE size.organization_id = :organization_id
--     ),
--     'image', (
--         SELECT json_agg(row_to_json(image.*))
--         FROM image WHERE image.organization_id = :organization_id
--     ),
--     'pricelist', (
--         SELECT json_agg(row_to_json(pricelist.*))
--         FROM pricelist WHERE pricelist.organization_id = :organization_id
--     ),
--     'price', (
--         SELECT json_agg(row_to_json(price.*))
--         FROM price WHERE price.organization_id = :organization_id
--     ),
--     'user_organization', (
--         SELECT json_agg(row_to_json(user_organization.*))
--         FROM user_organization
--         WHERE user_organization.organization_id = :organization_id
--     ),
--     'collection_pricelist', (
--         SELECT json_agg(row_to_json(collection_pricelist.*))
--         FROM collection_pricelist
--         INNER JOIN collection ON collection.id = collection_pricelist.collection_id
--         WHERE collection.organization_id = :organization_id
--     ),
--     'group_collection', (
--         SELECT json_agg(row_to_json(group_collection.*))
--         FROM group_collection
--         INNER JOIN "group" ON "group".id = group_collection.group_id
--         WHERE "group".organization_id = :organization_id
--     ),
--     'group_pricelist', (
--         SELECT json_agg(row_to_json(group_pricelist.*))
--         FROM group_pricelist
--         INNER JOIN "group" ON "group".id = group_pricelist.group_id
--         WHERE "group".organization_id = :organization_id
--     ),
--     'group_user', (
--         SELECT json_agg(row_to_json(group_user.*))
--         FROM group_user
--         INNER JOIN "group" ON "group".id = group_user.group_id
--         WHERE "group".organization_id = :organization_id
--     ),
--     'new_collection_style', (
--         SELECT json_agg(row_to_json(new_collection_style.*))
--         FROM new_collection_style
--         INNER JOIN collection ON collection.id = new_collection_style.collection_id
--         WHERE collection.organization_id = :organization_id
--     ),
--     'new_collection_color', (
--         SELECT json_agg(row_to_json(new_collection_color.*))
--         FROM new_collection_color
--         INNER JOIN collection ON collection.id = new_collection_color.collection_id
--         WHERE collection.organization_id = :organization_id
--     ),
--     'size_collection', (
--         SELECT json_agg(row_to_json(size_collection.*))
--         FROM size_collection
--         INNER JOIN collection ON collection.id = size_collection.collection_id
--         WHERE collection.organization_id = :organization_id
--     ),
--     'style_attribute', (
--         SELECT json_agg(row_to_json(style_attribute.*))
--         FROM style_attribute
--         INNER JOIN "attribute" ON "attribute".id = style_attribute.attribute_id
--         WHERE "attribute".organization_id = :organization_id
--     ),
--     'style_category', (
--         SELECT json_agg(row_to_json(style_category.*))
--         FROM style_category
--         INNER JOIN category ON category.id = style_category.category_id
--         WHERE category.organization_id = :organization_id
--     )
-- ) AS "data";
