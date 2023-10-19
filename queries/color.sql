-- Item color related queries
--
-- Entities:
--
--: ColorRow(external_id?, created_by?, style, images)
--: ColorRefs(external_id?)
--
--
-- Queries:
--
--! list_colors : ColorRow
SELECT
    color.*,
    jsonb_build_object(
        'id',
        style.id,
        'number',
        style.number,
        'name',
        style.name,
        'slug',
        style.slug,
        'external_id',
        style.external_id
    ) AS "style",
    coalesce(
        jsonb_agg(
            jsonb_build_object(
                'id',
                image.id,
                'external_id',
                image.external_id,
                'url',
                image.url
            )
            ORDER BY image.position ASC, image.uploaded_at DESC) FILTER (WHERE image.id IS NOT NULL
        ), '[]'::jsonb) AS "images"
FROM
    color
INNER JOIN style ON style.id = color.style_id
LEFT OUTER JOIN image ON image.color_id = color.id
WHERE
    color.organization_id = :organization_id
GROUP BY
    style.id,
    color.id
ORDER BY
    style.number,
    color.number;

--
--! get_color (id?,external_id?,slug?) : ColorRow
SELECT
    color.*,
    jsonb_build_object(
        'id',
        style.id,
        'number',
        style.number,
        'name',
        style.name,
        'slug',
        style.slug,
        'external_id',
        style.external_id
    ) AS "style",
    coalesce(
        jsonb_agg(
            jsonb_build_object(
                'id',
                image.id,
                'external_id',
                image.external_id,
                'url',
                image.url
            )
            ORDER BY image.position ASC, image.uploaded_at DESC) FILTER (WHERE image.id IS NOT NULL
        ), '[]'::jsonb) AS "images"
FROM
    color
INNER JOIN style ON style.id = color.style_id
LEFT OUTER JOIN image ON image.color_id = color.id
WHERE
    color.organization_id = :organization_id
    AND (
        color.id = coalesce(:id, -1)
        OR color.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR color.slug = coalesce(:slug, '___NON_EXISTING___')
    )
GROUP BY
    style.id,
    color.id;

--
--! get_color_id (id?, external_id?, slug?)
SELECT color.id
FROM
    color
WHERE
    color.organization_id = :organization_id
    AND (
        color.id = coalesce(:id, -1)
        OR color.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR color.slug = coalesce(:slug, '___NON_EXISTING___')
    );

--
-- Get all references for the given id, external_id or slug
--
--! get_color_refs (id?, external_id?, slug?) : ColorRefs
SELECT
    color.id,
    color.external_id,
    color.slug
FROM
    color
WHERE
    color.organization_id = :organization_id
    AND (
        color.id = coalesce(:id, -1)
        OR color.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR color.slug = coalesce(:slug, '___NON_EXISTING___')
    );

--
--! insert_color (external_id?)
INSERT INTO color (
    style_id,
    slug,
    external_id,
    number,
    name,
    organization_id,
    created_by)
VALUES (
    :style_id,
    :slug,
    :external_id,
    :number,
    :name,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_color (slug?, external_id?, number?, name?)
UPDATE
color
SET
    style_id = coalesce(:style_id, style_id),
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id),
    number = coalesce(:number, number),
    name = coalesce(:name, name)
WHERE
    id = :id;

--! delete_color
DELETE FROM color
WHERE organization_id = :organization_id
      AND id = :id;
