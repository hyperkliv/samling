-- AttributeType related queries
--
-- Entities:
--
--: AttributeTypeRow(external_id?, created_by?)
--
--
-- Queries:
--
--! list_attribute_types : AttributeTypeRow
SELECT attributetype.*
FROM
    attributetype
WHERE
    attributetype.organization_id = :organization_id
ORDER BY
    attributetype.updated_at DESC;

--
--! get_attribute_type (id?, external_id?, slug?) : AttributeTypeRow
SELECT attributetype.*
FROM
    attributetype
WHERE
    attributetype.organization_id = :organization_id
    AND ((attributetype.id = coalesce(:id, -1))
        OR (
            attributetype.external_id = coalesce(
                :external_id, '___NON_EXISTING___'
            )
        )
        OR (attributetype.slug = coalesce(:slug, '___NON_EXISTING___')));

--
--! get_attribute_type_id (id?, external_id?, slug?)
SELECT attributetype.id
FROM
    attributetype
WHERE
    attributetype.organization_id = :organization_id
    AND ((attributetype.id = coalesce(:id, -1))
        OR (
            attributetype.external_id = coalesce(
                :external_id, '___NON_EXISTING___'
            )
        )
        OR (attributetype.slug = coalesce(:slug, '___NON_EXISTING___')));

--
--! insert_attribute_type (external_id?)
INSERT INTO attributetype (
    name,
    slug,
    external_id,
    organization_id,
    created_by)
VALUES (
    :name,
    :slug,
    :external_id,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_attribute_type (name?, slug?, external_id?)
UPDATE
attributetype
SET
    name = coalesce(:name, name),
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id)
WHERE
    id = :id;

--
--! delete_attribute_type
DELETE FROM attributetype
WHERE organization_id = :organization_id
      AND id = :id;
