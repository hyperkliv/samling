-- Item style related queries
--
-- Entities:
--
--: StyleRow(external_id?, created_by?, core?, country_of_origin?, tariff_no?)
--: NestedStyleRow(external_id?, created_by?, core?, country_of_origin?, tariff_no?, is_new?)
--: NestedStyleSummaryRow()
--: StyleRefs(external_id?)
--
--
-- Queries:
--
--! select_styles (ids?) : StyleRow
SELECT
    style.*,
    coalesce(joined_categories.json_data, '[]') AS "categories",
    coalesce(joined_attributes.json_data, '[]') AS "attributes"
FROM
    style
LEFT JOIN (
    SELECT
        style_attribute.style_id,
        json_agg(
            json_build_object(
                'id',
                "attribute".id,
                'title',
                "attribute".title,
                'description',
                "attribute".description,
                'slug',
                "attribute".slug,
                'external_id',
                "attribute".external_id,
                'type',
                json_build_object(
                    'id',
                    attributetype.id,
                    'name',
                    attributetype.name,
                    'slug',
                    attributetype.slug,
                    'external_id',
                    attributetype.external_id
                )
            )
            ORDER BY
                attributetype.name,
                "attribute".title
        ) FILTER (
            WHERE
            "attribute".id IS NOT NULL
        ) AS json_data
    FROM
        attribute
    INNER JOIN attributetype ON attributetype.id = "attribute".type_id
    INNER JOIN style_attribute ON "attribute".id = style_attribute.attribute_id
    WHERE
        "attribute".organization_id = :organization_id
    GROUP BY
        style_attribute.style_id
    ) AS joined_attributes ON joined_attributes.style_id = style.id
LEFT JOIN (
    SELECT
        style_category.style_id,
        json_agg(category.*) FILTER (
            WHERE
            category.id IS NOT NULL
        ) AS "json_data"
    FROM
        category
    INNER JOIN style_category ON category.id = style_category.category_id
    WHERE
        category.organization_id = :organization_id
    GROUP BY
        style_category.style_id
    ) AS joined_categories ON joined_categories.style_id = style.id
WHERE
    style.organization_id = :organization_id
    AND :ids::int[] IS NULL OR style.id = any(:ids)
ORDER BY
    style.number;

--! select_collection_styles_nested (ids?, pricelist_ids_to_display?, statuses?) : NestedStyleRow
WITH new_styles AS (
    SELECT
        new_collection_style.style_id,
        new_collection_style.is_new
    FROM
        new_collection_style
    WHERE
        new_collection_style.collection_id = :collection_id
),

new_colors AS (
    SELECT
        new_collection_color.color_id,
        new_collection_color.is_new
    FROM
        new_collection_color
    WHERE
        new_collection_color.collection_id = :collection_id
)

SELECT
    style.*,
    new_styles.is_new,
    coalesce(joined_colors.json_data, '[]') AS "colors",
    coalesce(joined_prices.json_data, '[]') AS "prices",
    coalesce(joined_attributes.json_data, '[]') AS "attributes",
    coalesce(joined_categories.json_data, '[]') AS "categories"
FROM
    style
LEFT JOIN new_styles ON new_styles.style_id = style.id
INNER JOIN (
        SELECT
            color.style_id,
            json_agg(
                json_build_object(
                    'id',
                    color.id,
                    'number',
                    color.number,
                    'name',
                    color.name,
                    'slug',
                    color.slug,
                    'external_id',
                    color.external_id,
                    'sizes', coalesce(joined_sizes.json_data, '[]'),
                    'images', coalesce(joined_images.json_data, '[]'),
                    'is_new',
                    new_colors.is_new
                )
                ORDER BY
                    color.number
            ) FILTER (
                WHERE
                color.id IS NOT NULL
            ) AS json_data
        FROM
            color
        LEFT JOIN new_colors ON new_colors.color_id = color.id
        INNER JOIN (
            SELECT
                size.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        size.id,
                        'number',
                        size.number,
                        'position',
                        size.position,
                        'name',
                        size.name,
                        'service_item',
                        size.service_item,
                        'delivery_period',
                        size.delivery_period,
                        'ean_code',
                        size.ean_code,
                        'status',
                        size.status,
                        'slug',
                        size.slug,
                        'external_id',
                        size.external_id
                    )
                    ORDER BY
                        size.number
                ) FILTER (
                    WHERE
                    size.id IS NOT NULL
                ) AS json_data
            FROM
                size
            INNER JOIN color ON size.color_id = color.id
            INNER JOIN style ON color.style_id = style.id
            INNER JOIN size_collection ON size_collection.size_id = size.id
            WHERE
                size.organization_id = :organization_id
                AND size_collection.collection_id = :collection_id
                AND (
                    :statuses::text[] IS NULL OR size.status = any(:statuses)
                )
            GROUP BY
                size.color_id
            ) AS joined_sizes ON joined_sizes.color_id = color.id
        LEFT JOIN (
            SELECT
                image.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        image.id,
                        'url',
                        image.url,
                        'external_id',
                        image.external_id
                    )
                    ORDER BY
                        image.position
                ) AS json_data
            FROM
                image
            WHERE
                image.organization_id = :organization_id
            GROUP BY
                image.color_id
            ) AS joined_images ON joined_images.color_id = color.id
        WHERE
            color.organization_id = :organization_id
        GROUP BY
            color.style_id
    ) AS joined_colors ON joined_colors.style_id = style.id
LEFT JOIN (
        SELECT
            price.style_id,
            json_agg(
                json_build_object(
                    'id',
                    price.id,
                    'type',
                    price.type,
                    'uom',
                    price.uom,
                    'currency',
                    price.currency,
                    'amount',
                    price.amount,
                    'start',
                    price.start,
                    'end',
                    price.end,
                    'list',
                    json_build_object(
                        'id',
                        pricelist.id,
                        'slug',
                        pricelist.slug,
                        'external_id',
                        pricelist.external_id,
                        'name',
                        pricelist.name
                    )
                )
                ORDER BY
                    price.type,
                    pricelist.name,
                    price."start"
            ) FILTER (
                WHERE
                price.id IS NOT NULL
            ) AS json_data
        FROM
            price
        INNER JOIN pricelist ON pricelist.id = price.list_id
        INNER JOIN collection_pricelist ON
            collection_pricelist.pricelist_id = pricelist.id
        WHERE
            price.organization_id = :organization_id
            AND (
                :pricelist_ids_to_display::int[] IS NULL
                OR pricelist.id = any(:pricelist_ids_to_display)
            )
            AND collection_pricelist.collection_id = :collection_id
            AND (
                collection_pricelist.price_date
                BETWEEN price."start" AND price."end"
            )
        GROUP BY
            price.style_id
    ) AS joined_prices ON joined_prices.style_id = style.id
LEFT JOIN (
        SELECT
            style_attribute.style_id,
            json_agg(
                json_build_object(
                    'id',
                    "attribute".id,
                    'title',
                    "attribute".title,
                    'description',
                    "attribute".description,
                    'slug',
                    "attribute".slug,
                    'external_id',
                    "attribute".external_id,
                    'type',
                    json_build_object(
                        'id',
                        attributetype.id,
                        'name',
                        attributetype.name,
                        'slug',
                        attributetype.slug,
                        'external_id',
                        attributetype.external_id
                    )
                )
                ORDER BY
                    attributetype.name,
                    "attribute".title
            ) FILTER (
                WHERE
                "attribute".id IS NOT NULL
            ) AS json_data
        FROM
            attribute
        INNER JOIN attributetype ON attributetype.id = "attribute".type_id
        INNER JOIN style_attribute ON "attribute".id = style_attribute.attribute_id
        WHERE
            "attribute".organization_id = :organization_id
        GROUP BY
            style_attribute.style_id
    ) AS joined_attributes ON joined_attributes.style_id = style.id
LEFT JOIN (
        SELECT
            style_category.style_id,
            json_agg(
                json_build_object(
                    'id',
                    category.id,
                    'slug',
                    category.slug,
                    'name',
                    category.name,
                    'external_id',
                    category.external_id
                )
            ) FILTER (
                WHERE
                category.id IS NOT NULL
            ) AS "json_data"
        FROM
            category
        INNER JOIN style_category ON category.id = style_category.category_id
        WHERE
            category.organization_id = :organization_id
        GROUP BY
            style_category.style_id
    ) AS joined_categories ON joined_categories.style_id = style.id
WHERE
    (:ids::int[] IS NULL OR style.id = any(:ids))
ORDER BY
    style.number;

--! select_nested_style_summaries (ids?, categories?, statuses?) : NestedStyleSummaryRow
SELECT
    style.id,
    style.name,
    style.number,
    coalesce(joined_colors.json_data, '[]') AS "colors"
FROM
    style
INNER JOIN (
        SELECT
            color.style_id,
            json_agg(
                json_build_object(
                    'id',
                    color.id,
                    'number',
                    color.number,
                    'name',
                    color.name,
                    'sizes', coalesce(joined_sizes.json_data, '[]'),
                    'primary_image', primary_image.json_data
                )
                ORDER BY
                    color.number
            ) FILTER (
                WHERE
                color.id IS NOT NULL
            ) AS json_data
        FROM
            color
        INNER JOIN (
            SELECT
                size.color_id,
                json_agg(
                    json_build_object(
                        'id',
                        size.id,
                        'number',
                        size.number,
                        'name',
                        size.name
                    )
                    ORDER BY
                        size.number
                ) FILTER (
                    WHERE
                    size.id IS NOT NULL
                ) AS json_data
            FROM
                size
            INNER JOIN color ON size.color_id = color.id
            WHERE
                size.organization_id = :organization_id
                AND (:statuses::text[] IS NULL OR size.status = any(:statuses))
            GROUP BY
                size.color_id
            ) AS joined_sizes ON joined_sizes.color_id = color.id
        LEFT JOIN (
            -- We utilize distinct here because there might be multiple rows with
            -- `position = 1` for a given `color_id`.
            SELECT DISTINCT ON (image.color_id)
                image.color_id,
                json_build_object(
                    'id',
                    image.id,
                    'url',
                    image.url,
                    'external_id',
                    image.external_id
                ) AS json_data
            FROM
                image
            WHERE
                image.organization_id = :organization_id
                AND image.position = 1
            ) AS primary_image ON primary_image.color_id = color.id
        WHERE
            color.organization_id = :organization_id
        GROUP BY
            color.style_id
    ) AS joined_colors ON joined_colors.style_id = style.id
LEFT JOIN (
    SELECT
        style_category.style_id,
        array_agg(style_category.category_id) AS category_ids
    FROM style_category
    GROUP BY style_category.style_id
) AS style_categories ON style_categories.style_id = style.id
WHERE
    style.organization_id = :organization_id
    AND (:ids::int[] IS NULL OR style.id = any(:ids))
    AND (:categories::int[] IS NULL OR style_categories.category_ids && :categories)
ORDER BY
    style.number;

--
--! get_style_id (id?, external_id?, slug?)
SELECT style.id
FROM
    style
WHERE
    style.organization_id = :organization_id
    AND (
        (style.id = coalesce(:id, -1))
        OR (
            style.external_id = coalesce(:external_id, '___NON_EXISTING___')
            OR (style.slug = coalesce(:slug, '___NON_EXISTING___'))
        )
    );

--
-- Get all references for the given id, external_id or slug
--
--! get_style_refs (id?, external_id?, slug?) : StyleRefs
SELECT
    style.id,
    style.external_id,
    style.slug
FROM
    style
WHERE
    style.organization_id = :organization_id
    AND (
        (style.id = coalesce(:id, -1))
        OR (
            style.external_id = coalesce(:external_id, '___NON_EXISTING___')
            OR (style.slug = coalesce(:slug, '___NON_EXISTING___'))
        )
    );

--
--! insert_style (external_id?, core?, country_of_origin?, tariff_no?)
INSERT INTO
style (
    organization_id,
    slug,
    external_id,
    number,
    name,
    description,
    core,
    country_of_origin,
    tariff_no,
    net_weight,
    gross_weight,
    unit_volume,
    created_by
)
VALUES
(
    :organization_id,
    :slug,
    :external_id,
    :number,
    :name,
    :description,
    :core,
    :country_of_origin,
    :tariff_no,
    :net_weight,
    :gross_weight,
    :unit_volume,
    :created_by
)
RETURNING
id;

--
--! update_style (slug?, external_id?, number?, name?, description?, core?, country_of_origin?, tariff_no?, net_weight?, gross_weight?, unit_volume?)
UPDATE
style
SET
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id),
    number = coalesce(:number, number),
    name = coalesce(:name, name),
    description = coalesce(:description, description),
    core = coalesce(:core, core),
    country_of_origin = coalesce(:country_of_origin, country_of_origin),
    tariff_no = coalesce(:tariff_no, tariff_no),
    net_weight = coalesce(:net_weight, net_weight),
    gross_weight = coalesce(:gross_weight, gross_weight),
    unit_volume = coalesce(:unit_volume, unit_volume)
WHERE
    id = :id
RETURNING
id;

--! delete_style
DELETE FROM
style
WHERE
    organization_id = :organization_id
    AND id = :id
RETURNING
id;
