-- Item category related queries
--
-- Entities:
--
--: CategoryRow(external_id?, created_by?)
--
--
-- Queries:
--
--! list_categories : CategoryRow
SELECT *
FROM
    category
WHERE
    category.organization_id = :organization_id;

--
--! get_category_id (id?, external_id?, slug?)
SELECT category.id
FROM
    category
WHERE
    category.organization_id = :organization_id
    AND (
        category.id = coalesce(:id, -1)
        OR category.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR category.slug = coalesce(:slug, '___NON_EXISTING___')
    );

--
--! get_category (id?,external_id?,slug?) : CategoryRow
SELECT *
FROM
    category
WHERE
    category.organization_id = :organization_id
    AND (
        category.id = coalesce(:id, -1)
        OR category.external_id = coalesce(:external_id, '___NON_EXISTING___')
        OR category.slug = coalesce(:slug, '___NON_EXISTING___')
    );

--
--! insert_category (external_id?)
INSERT INTO category (
    slug,
    external_id,
    name,
    organization_id,
    created_by)
VALUES (
    :slug,
    :external_id,
    :name,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_category (slug?, external_id?, name?)
UPDATE category
SET
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id),
    name = coalesce(:name, name)
WHERE
    category.id = :id;

--! delete_category
DELETE FROM category
WHERE
    organization_id = :organization_id
    AND id = :id;

--! associate_style_categories
SELECT *
FROM
    associate_style_categories(:style_id, :category_ids);
