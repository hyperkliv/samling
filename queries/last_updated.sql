--! nested_styles_last_modified_at (organization_id, collection_ids?)
SELECT max(t.updated_at) FROM (
    (
        SELECT attribute.updated_at
        FROM
            attribute
        WHERE
            attribute.organization_id = :organization_id
        ORDER BY attribute.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT attributetype.updated_at
        FROM
            attributetype
        WHERE
            attributetype.organization_id = :organization_id
        ORDER BY attributetype.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT category.updated_at
        FROM
            category
        WHERE
            category.organization_id = :organization_id
        ORDER BY category.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT collection.updated_at
        FROM
            collection
        WHERE
            collection.organization_id = :organization_id AND (
                :collection_ids::int[] IS NULL OR collection.id = any(:collection_ids)
            )
        ORDER BY collection.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT color.updated_at
        FROM
            color
        WHERE
            color.organization_id = :organization_id
        ORDER BY color.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT image.updated_at
        FROM
            image
        WHERE
            image.organization_id = :organization_id
        ORDER BY image.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT pricelist.updated_at
        FROM
            pricelist
        WHERE
            pricelist.organization_id = :organization_id
        ORDER BY pricelist.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT price.updated_at
        FROM
            price
        WHERE
            price.organization_id = :organization_id
        ORDER BY price.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT size.updated_at
        FROM
            size
        WHERE
            size.organization_id = :organization_id
        ORDER BY size.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT style.updated_at
        FROM
            style
        WHERE
            style.organization_id = :organization_id
        ORDER BY style.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT collection_pricelist.updated_at
        FROM
            collection_pricelist
        INNER JOIN
            collection ON collection.id = collection_pricelist.collection_id
        WHERE
            collection.organization_id = :organization_id AND (
                :collection_ids::int[] IS NULL OR collection.id = any(:collection_ids)
            )
        ORDER BY collection_pricelist.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT new_collection_style.updated_at
        FROM
            new_collection_style
        INNER JOIN
            collection ON collection.id = new_collection_style.collection_id
        WHERE
            collection.organization_id = :organization_id AND (
                :collection_ids::int[] IS NULL OR collection.id = any(:collection_ids)
            )
        ORDER BY new_collection_style.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT new_collection_color.updated_at
        FROM
            new_collection_color
        INNER JOIN
            collection ON collection.id = new_collection_color.collection_id
        WHERE
            collection.organization_id = :organization_id AND (
                :collection_ids::int[] IS NULL OR collection.id = any(:collection_ids)
            )
        ORDER BY new_collection_color.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT size_collection.updated_at
        FROM
            size_collection
        INNER JOIN
            collection ON collection.id = size_collection.collection_id
        WHERE
            collection.organization_id = :organization_id AND (
                :collection_ids::int[] IS NULL OR collection.id = any(:collection_ids)
            )
        ORDER BY size_collection.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT style_attribute.updated_at
        FROM
            style_attribute
        INNER JOIN
            style ON style.id = style_attribute.style_id
        WHERE
            style.organization_id = :organization_id
        ORDER BY style_attribute.updated_at DESC LIMIT 1
    )
    UNION ALL
    (
        SELECT style_category.updated_at
        FROM
            style_category
        INNER JOIN
            style ON style.id = style_category.style_id
        WHERE
            style.organization_id = :organization_id
        ORDER BY style_category.updated_at DESC LIMIT 1
    )
) AS t;
