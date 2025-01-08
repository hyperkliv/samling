-- Item size related queries
--
-- Entities:
--
--: SizeRow(service_item?, delivery_period?, ean_code?, status?, external_id?, created_by?, color)
--
--
-- Queries:
--
--! list_sizes : SizeRow
SELECT
    size.*,
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
    size
INNER JOIN color ON size.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    size.organization_id = :organization_id
ORDER BY
    size.id;

--
--! get_size_id (id?, external_id?, slug?)
SELECT size.id
FROM
    size
WHERE
    size.organization_id = :organization_id
    AND (
        size.id = coalesce(:id, -1)
        OR size.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR size.slug = coalesce(:slug, '___NON_EXISTING___')
    );

--
--! get_size (id?,external_id?,slug?) : SizeRow
SELECT
    size.*,
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
    size
INNER JOIN color ON size.color_id = color.id
INNER JOIN style ON color.style_id = style.id
WHERE
    size.organization_id = :organization_id
    AND (
        size.id = coalesce(:id, -1)
        OR size.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR size.slug = coalesce(:slug, '___NON_EXISTING___')
    );

--
--! insert_size (service_item?, delivery_period?, ean_code?, status?, external_id?)
INSERT INTO size (
    color_id,
    slug,
    external_id,
    number,
    name,
    service_item,
    delivery_period,
    ean_code,
    status,
    organization_id,
    created_by)
VALUES (
    :color_id,
    :slug,
    :external_id,
    :number,
    :name,
    :service_item,
    :delivery_period,
    :ean_code,
    :status,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_size (position?, service_item?, delivery_period?, ean_code?, status?, slug?, external_id?, number?, name?)
UPDATE
size
SET
    color_id = coalesce(:color_id, color_id),
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id),
    number = coalesce(:number, number),
    position = coalesce(:position, position),
    name = coalesce(:name, name),
    service_item = coalesce(:service_item, service_item),
    delivery_period = coalesce(:delivery_period, delivery_period),
    ean_code = coalesce(:ean_code, ean_code),
    status = coalesce(:status, status)
WHERE
    id = :id;

--! delete_size
DELETE FROM size
WHERE organization_id = :organization_id
      AND id = :id;
