-- Item image related queries
--
-- Entities:
--
--: ImageRow(external_id?, external_checksum?, uploaded_by?, color)
--
--
-- Queries:
--
--! list_images : ImageRow
SELECT
    image.*,
    jsonb_build_object(
        'id',
        color.id,
        'style',
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
        ),
        'number',
        color.number,
        'name',
        color.name,
        'slug',
        color.slug,
        'external_id',
        color.external_id
    ) AS "color"
FROM
    image
INNER JOIN color ON image.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    image.organization_id = :organization_id
ORDER BY
    image.id;

--
--! get_image_id (id?, external_id?)
SELECT image.id
FROM
    image
WHERE
    image.organization_id = :organization_id
    AND (
        image.id = coalesce(:id, -1)
        OR image.external_id = coalesce(:external_id, '___NON_EXISTING___')
    );

--
--! get_image_url_by_external_checksum
SELECT image.url
FROM
    image
WHERE
    image.organization_id = :organization_id
    AND image.external_checksum = :external_checksum;

--
--! get_image (id?,external_id?) : ImageRow
SELECT
    image.*,
    jsonb_build_object(
        'id',
        color.id,
        'style',
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
        ),
        'number',
        color.number,
        'name',
        color.name,
        'slug',
        color.slug,
        'external_id',
        color.external_id
    ) AS "color"
FROM
    image
INNER JOIN color ON image.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    image.organization_id = :organization_id
    AND (
        image.id = coalesce(:id, -1)
        OR image.external_id = coalesce(:external_id, '___NON_EXISTING___')
    );

--
--! insert_image (external_id?, external_checksum?)
INSERT INTO image (
    color_id,
    external_id,
    url,
    organization_id,
    uploaded_by,
    external_checksum,
    position)
VALUES (
    :color_id,
    :external_id,
    :url,
    :organization_id,
    :uploaded_by,
    :external_checksum,
    :position)
RETURNING
id;

--
--! update_image (external_id?, external_checksum?, url?, position?)
UPDATE
image
SET
    color_id = coalesce(:color_id, color_id),
    external_id = coalesce(:external_id, external_id),
    url = coalesce(:url, url),
    external_checksum = coalesce(:external_checksum, external_checksum),
    position = coalesce(:position, position)
WHERE
    id = :id;

--! delete_image
DELETE FROM image
WHERE organization_id = :organization_id
      AND id = :id;
