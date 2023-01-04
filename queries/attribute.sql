-- Attribute related queries
--
-- Entities:
--
--: AttributeRow(external_id?, created_by?)
--
--
-- Queries:
--
--! list_attributes : AttributeRow
SELECT
    attribute.*,
    to_jsonb(attributetype.*) AS "type"
FROM
    attribute
INNER JOIN attributetype
    ON attributetype.id = attribute.type_id
WHERE
    attribute.organization_id = :organization_id
ORDER BY
    attribute.updated_at DESC;

--
--! get_attribute (id?, external_id?, slug?) : AttributeRow
SELECT
    attribute.*,
    to_jsonb(attributetype.*) AS "type"
FROM
    attribute
INNER JOIN attributetype
    ON attributetype.id = attribute.type_id
WHERE
    attribute.organization_id = :organization_id
    AND ((attribute.id = coalesce(:id, -1))
        OR (
            attribute.external_id = coalesce(:external_id, '___NON_EXISTING___')
        )
        OR (attribute.slug = coalesce(:slug, '___NON_EXISTING___')));

--
--! get_attribute_id (id?, external_id?, slug?)
SELECT attribute.id
FROM
    attribute
WHERE
    attribute.organization_id = :organization_id
    AND ((attribute.id = coalesce(:id, -1))
        OR (
            attribute.external_id = coalesce(:external_id, '___NON_EXISTING___')
        )
        OR (attribute.slug = coalesce(:slug, '___NON_EXISTING___')));

--
--! insert_attribute (external_id?)
INSERT INTO attribute (
    title,
    description,
    type_id,
    slug,
    external_id,
    organization_id,
    created_by)
VALUES (
    :title,
    :description,
    :type_id,
    :slug,
    :external_id,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_attribute (type_id?, title?, description?, slug?, external_id?)
UPDATE
attribute
SET
    type_id = coalesce(:type_id, type_id),
    title = coalesce(:title, title),
    description = coalesce(:description, description),
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id)
WHERE
    id = :id;

--
--! delete_attribute
DELETE FROM attribute
WHERE organization_id = :organization_id
      AND id = :id;

--! associate_style_attributes
SELECT *
FROM
    associate_style_attributes(:style_id, :attribute_ids);
