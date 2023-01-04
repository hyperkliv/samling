-- PriceList related queries
--
-- Entities:
--
--: PriceListRow(external_id?, created_by?)
--: PriceListSummaryRow(external_id?)
--
--
-- Queries:
--
--! list_pricelists : PriceListRow
SELECT pricelist.*
FROM
    pricelist
INNER JOIN (
    SELECT group_pricelist.pricelist_id
    FROM group_pricelist
    INNER JOIN group_user
        ON group_user.group_id = group_pricelist.group_id
    WHERE
        group_user.user_id = :requester_id
    GROUP BY group_pricelist.pricelist_id
) AS requester_pricelists ON requester_pricelists.pricelist_id = pricelist.id
WHERE
    pricelist.organization_id = :organization_id
ORDER BY
    pricelist.name;

--! list_pricelist_summaries (collection_ids?) : PriceListSummaryRow
WITH collection_pricelist AS (
    SELECT pricelist.id
    FROM pricelist
    INNER JOIN price ON price.list_id = pricelist.id
    INNER JOIN color ON color.style_id = price.style_id
    INNER JOIN size ON size.color_id = color.id
    INNER JOIN size_collection ON size_collection.size_id = size.id
    WHERE
        :collection_ids::int[] IS NULL OR size_collection.collection_id = any(
            :collection_ids
        )
    GROUP BY pricelist.id
)

SELECT
    pricelist.id,
    pricelist.name,
    pricelist.slug,
    pricelist.external_id
FROM
    pricelist
INNER JOIN collection_pricelist
    ON collection_pricelist.id = pricelist.id
INNER JOIN (
    SELECT group_pricelist.pricelist_id
    FROM group_pricelist
    INNER JOIN group_user
        ON group_user.group_id = group_pricelist.group_id
    WHERE
        group_user.user_id = :requester_id
    GROUP BY group_pricelist.pricelist_id
) AS requester_pricelists ON requester_pricelists.pricelist_id = pricelist.id
WHERE
    pricelist.organization_id = :organization_id
ORDER BY
    pricelist.name;

--
--! get_pricelist (id?, external_id?, slug?) : PriceListRow
SELECT pricelist.*
FROM
    pricelist
WHERE
    pricelist.organization_id = :organization_id
    AND (
        (:id::int IS NULL OR pricelist.id = :id)
        AND (:external_id::text IS NULL OR pricelist.external_id = :external_id)
        AND (:slug::text IS NULL OR pricelist.slug = :slug)
    );

--
--! get_pricelist_id (id?, external_id?, slug?)
SELECT pricelist.id
FROM
    pricelist
WHERE
    pricelist.organization_id = :organization_id
    AND (
        (:id::int IS NULL OR pricelist.id = :id)
        AND (:external_id::text IS NULL OR pricelist.external_id = :external_id)
        AND (:slug::text IS NULL OR pricelist.slug = :slug)
    );

--
--! insert_pricelist (external_id?)
INSERT INTO pricelist (
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
--! update_pricelist (name?, slug?, external_id?)
UPDATE pricelist
SET
    name = coalesce(:name, name),
    slug = coalesce(:slug, slug),
    external_id = coalesce(:external_id, external_id)
WHERE
    id = :id;

--
--! delete_pricelist
DELETE FROM pricelist
WHERE
    organization_id = :organization_id
    AND id = :id;

--
--! allowed_pricelist_ids
SELECT DISTINCT group_pricelist.pricelist_id FROM group_pricelist
INNER JOIN group_user ON group_user.group_id = group_pricelist.group_id
INNER JOIN user_organization ON user_organization.user_id = group_user.user_id
WHERE
    user_organization.organization_id = :organization_id
    AND group_user.user_id = :user_id;
