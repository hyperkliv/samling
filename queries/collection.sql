-- Collection related queries
--
-- Entities:
--
--: CollectionRow(image_url?, external_id?, created_by?)
--: CollectionSummaryRow(image_url?, external_id?, created_by?)
--
--
-- Queries:
--
--! select_collections (id?, external_id?, slug?) : CollectionRow
SELECT
    collection.*,
    coalesce(pricing.json_data, '[]'::json) AS pricing,
    coalesce(sizes.json_data, '[]'::json) AS sizes
FROM
    collection
INNER JOIN (
    SELECT group_collection.collection_id
    FROM group_collection
    INNER JOIN group_user
        ON group_user.group_id = group_collection.group_id
    WHERE
        group_user.user_id = :requester_id
    GROUP BY group_collection.collection_id
    UNION
    SELECT collection.id AS collection_id
    FROM collection
    WHERE collection.created_by = :requester_id
) AS requester_collections ON requester_collections.collection_id = collection.id
LEFT JOIN (
    SELECT
        size_collection.collection_id,
        json_agg(
            size.* ORDER BY size_collection.position
        ) FILTER (WHERE size.id IS NOT NULL) AS json_data
    FROM size
    INNER JOIN size_collection ON size_collection.size_id = size.id
    GROUP BY size_collection.collection_id
) AS sizes ON sizes.collection_id = collection.id
LEFT JOIN LATERAL (
    SELECT
        collection_pricelist.collection_id,
        json_agg(
            json_build_object(
                'list',
                json_build_object(
                    'id',
                    pricelist.id,
                    'external_id',
                    pricelist.external_id,
                    'slug',
                    pricelist.slug,
                    'name',
                    pricelist.name
                ),
                'date',
                collection_pricelist.price_date
            )
        ) FILTER (WHERE collection_pricelist.collection_id IS NOT NULL) AS json_data
    FROM collection_pricelist
    INNER JOIN pricelist
        ON pricelist.id = collection_pricelist.pricelist_id
    WHERE collection_pricelist.collection_id = collection.id
    GROUP BY collection_pricelist.collection_id
) AS pricing ON pricing.collection_id = collection.id
WHERE
    collection.organization_id = :organization_id
    AND (:id::int IS NULL OR collection.id = :id)
    AND (:external_id::text IS NULL OR collection.external_id = :external_id)
    AND (:slug::text IS NULL OR collection.slug = :slug)
ORDER BY
    collection.updated_at DESC;

--
--! select_collection_summaries (id?, external_id?, slug?) : CollectionSummaryRow
SELECT
    collection.*,
    coalesce(stats.num_sizes, 0) AS num_sizes,
    coalesce(stats.num_colors, 0) AS num_colors,
    coalesce(stats.num_styles, 0) AS num_styles,
    coalesce(pricing.json_data, '[]'::json) AS pricing
FROM
    collection
INNER JOIN (
    SELECT group_collection.collection_id
    FROM group_collection
    INNER JOIN group_user
        ON group_user.group_id = group_collection.group_id
    WHERE
        group_user.user_id = :requester_id
    GROUP BY group_collection.collection_id
    UNION
    SELECT collection.id AS collection_id
    FROM collection
    WHERE collection.created_by = :requester_id
) AS requester_collections ON requester_collections.collection_id = collection.id
LEFT JOIN (
    SELECT
        size_collection.collection_id,
        count(DISTINCT size.id) AS num_sizes,
        count(DISTINCT color.id) AS num_colors,
        count(DISTINCT color.style_id) AS num_styles
    FROM color
    INNER JOIN size ON size.color_id = color.id
    INNER JOIN size_collection ON size.id = size_collection.size_id
    GROUP BY size_collection.collection_id
) AS stats ON stats.collection_id = collection.id
LEFT JOIN LATERAL (
    SELECT
        collection_pricelist.collection_id,
        json_agg(
            json_build_object(
                'list',
                json_build_object(
                    'id',
                    pricelist.id,
                    'external_id',
                    pricelist.external_id,
                    'slug',
                    pricelist.slug,
                    'name',
                    pricelist.name
                ),
                'date',
                collection_pricelist.price_date
            )
        ) FILTER (WHERE collection_pricelist.collection_id IS NOT NULL) AS json_data,
        min(collection_pricelist.price_date) AS min_price_date
    FROM collection_pricelist
    INNER JOIN pricelist
        ON pricelist.id = collection_pricelist.pricelist_id
    WHERE collection_pricelist.collection_id = collection.id
    GROUP BY collection_pricelist.collection_id
) AS pricing ON pricing.collection_id = collection.id
WHERE
    collection.organization_id = :organization_id
    AND (:id::int IS NULL OR collection.id = :id)
    AND (:external_id::text IS NULL OR collection.external_id = :external_id)
    AND (:slug::text IS NULL OR collection.slug = :slug)
ORDER BY
    pricing.min_price_date DESC, collection.name ASC;

--
--! get_collection_id (id?, external_id?, slug?)
SELECT collection.id
FROM
    collection
WHERE
    collection.organization_id = :organization_id
    AND (:id::int IS NULL OR collection.id = :id)
    AND (:external_id::text IS NULL OR collection.external_id = :external_id)
    AND (:slug::text IS NULL OR collection.slug = :slug);

--
--! insert_collection (image_url?, external_id?)
INSERT INTO collection (
    acronym,
    name,
    image_url,
    slug,
    external_id,
    organization_id,
    created_by)
VALUES (
    :acronym,
    :name,
    :image_url,
    :slug,
    :external_id,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_collection (acronym?, name?, image_url?, slug?, external_id?)
UPDATE
collection
SET
    acronym = coalesce(:acronym, acronym),
    name = coalesce(:name, name),
    image_url = coalesce(:image_url, image_url),
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id)
WHERE
    id = :id;

--
--! delete_collection
DELETE FROM collection
WHERE organization_id = :organization_id
      AND id = :id;

--
--! associate_collection_sizes
SELECT *
FROM
    associate_collection_sizes(:collection_id, :size_ids);

--
--! replace_collection_pricelists
SELECT *
FROM
    replace_collection_pricelists(:collection_id, :collection_pricelist_relations);

--
--! set_new_collection_styles
SELECT *
FROM
    set_new_collection_styles(:collection_id, :style_ids);

--
--! set_new_collection_colors
SELECT *
FROM
    set_new_collection_colors(:collection_id, :color_ids);
