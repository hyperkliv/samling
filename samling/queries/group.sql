-- Groups related queries
--
-- Entities:
--
--: GroupRow(external_id?, created_by?)
--: GroupSummaryRow(external_id?)
--
--
-- Queries:
--
--! select_groups (id?, external_id?, slug?) : GroupRow
SELECT
    "group".*,
    coalesce("users".json_data, '[]'::json) AS "users",
    coalesce(collections.json_data, '[]'::json) AS collections,
    coalesce(pricelists.json_data, '[]'::json) AS price_lists
FROM "group"
LEFT JOIN LATERAL (
    SELECT
        group_user.group_id,
        json_agg(json_build_object(
            'id',
            "user".id,
            'name',
            "user".name,
            'email',
            "user".email,
            'last_sign_in',
            "user".last_sign_in,
            'profile_image',
            "user".profile_image
        )) FILTER (WHERE "user".id IS NOT NULL) AS json_data
    FROM group_user
    INNER JOIN "user" ON "user".id = group_user.user_id
    WHERE group_user.group_id = "group".id
    GROUP BY group_user.group_id
) AS "users" ON "users".group_id = "group".id
LEFT JOIN LATERAL (
    SELECT
        group_collection.group_id,
        json_agg(
            json_build_object(
                'id',
                collection.id,
                'acronym',
                collection.acronym,
                'name',
                collection.name,
                'image_url',
                collection.image_url,
                'external_id',
                collection.external_id,
                'slug',
                collection.slug,
                'created_by',
                collection.created_by,
                'created_at',
                collection.created_at,
                'updated_at',
                collection.updated_at,
                'pricing',
                coalesce(pricing.json_data, '[]'::json),
                'num_styles',
                coalesce(stats.num_styles, 0),
                'num_colors',
                coalesce(stats.num_colors, 0),
                'num_sizes',
                coalesce(stats.num_sizes, 0)
            )
            ORDER BY collection.id DESC
        ) FILTER (WHERE collection.id IS NOT NULL) AS json_data
    FROM group_collection
    INNER JOIN collection ON collection.id = group_collection.collection_id
    LEFT JOIN (
        SELECT
            size_collection.collection_id,
            count(DISTINCT size.id) AS num_sizes,
            count(DISTINCT color.id) AS num_colors,
            count(DISTINCT color.style_id) AS num_styles
        FROM size_collection
        INNER JOIN size ON size.id = size_collection.size_id
        INNER JOIN color ON size.color_id = color.id
        GROUP BY size_collection.collection_id
    ) AS stats ON stats.collection_id = group_collection.collection_id
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
    WHERE group_collection.group_id = "group".id
    GROUP BY group_collection.group_id
) AS collections ON collections.group_id = "group".id
LEFT JOIN LATERAL (
    SELECT
        group_pricelist.group_id,
        json_agg(
            json_build_object(
                'id',
                pricelist.id,
                'name',
                pricelist.name,
                'slug',
                pricelist.slug,
                'external_id',
                pricelist.external_id
            )
        ) FILTER (WHERE pricelist.id IS NOT NULL) AS json_data
    FROM group_pricelist
    INNER JOIN pricelist ON pricelist.id = group_pricelist.pricelist_id
    WHERE group_pricelist.group_id = "group".id
    GROUP BY group_pricelist.group_id
) AS pricelists ON pricelists.group_id = "group".id
WHERE
    "group".organization_id = :organization_id
    AND (:id::int IS NULL OR "group".id = :id)
    AND (:external_id::text IS NULL OR "group".external_id = :external_id)
    AND (:slug::text IS NULL OR "group".slug = :slug)
ORDER BY
    "group".updated_at DESC;

--
--! select_group_summaries (id?, external_id?, slug?) : GroupSummaryRow
SELECT
    "group".id,
    "group".slug,
    "group".external_id,
    "group".name,
    "group".description,
    coalesce(user_counts.n, 0) AS num_users,
    coalesce(collection_counts.n, 0) AS num_collections,
    coalesce(pricelist_counts.n, 0) AS num_price_lists
FROM "group"
LEFT JOIN LATERAL (
    SELECT
        group_user.group_id,
        count(*) AS n
    FROM group_user WHERE group_user.group_id = "group".id GROUP BY group_user.group_id
) AS user_counts ON user_counts.group_id = "group".id
LEFT JOIN LATERAL (
    SELECT
        group_collection.group_id,
        count(*) AS n
    FROM
        group_collection
    WHERE group_collection.group_id = "group".id GROUP BY group_collection.group_id
) AS collection_counts ON collection_counts.group_id = "group".id
LEFT JOIN LATERAL (
    SELECT
        group_pricelist.group_id,
        count(*) AS n
    FROM
        group_pricelist
    WHERE group_pricelist.group_id = "group".id GROUP BY group_pricelist.group_id
) AS pricelist_counts ON pricelist_counts.group_id = "group".id
WHERE
    "group".organization_id = :organization_id
    AND (:id::int IS NULL OR "group".id = :id)
    AND (:external_id::text IS NULL OR "group".external_id = :external_id)
    AND (:slug::text IS NULL OR "group".slug = :slug)
ORDER BY
    "group".updated_at DESC;

--
--! get_group_id (id?, external_id?, slug?)
SELECT "group".id
FROM
    "group"
WHERE
    "group".organization_id = :organization_id
    AND (:id::int IS NULL OR "group".id = :id)
    AND (:external_id::text IS NULL OR "group".external_id = :external_id)
    AND (:slug::text IS NULL OR "group".slug = :slug);

--
--! insert_group (external_id?)
INSERT INTO "group" (
    slug,
    external_id,
    name,
    description,
    organization_id,
    created_by)
VALUES (
    :slug,
    :external_id,
    :name,
    :description,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_group (slug?, external_id?, name?, description?)
UPDATE "group"
SET
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id),
    name = coalesce(:name, name),
    description = coalesce(:description, description)
WHERE
    id = :id;

--! delete_group
DELETE FROM "group"
WHERE
    organization_id = :organization_id
    AND id = :id;

--
--! replace_group_users
SELECT *
FROM
    replace_group_users(:group_id, :user_ids);

--
--! replace_group_collections
SELECT *
FROM
    replace_group_collections(:group_id, :collection_ids);

--
--! replace_group_pricelists
SELECT *
FROM
    replace_group_pricelists(:group_id, :pricelist_ids);

--
--! ensure_superuser_access
SELECT ensure_superuser_access(:organization_id);
