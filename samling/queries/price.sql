-- Price related queries
--
-- Entities:
--
--: PriceRow(uom?, external_id?, created_by?)
--
--
-- Queries:
--
--! select_prices (ids?, external_ids?) : PriceRow
SELECT
    price.*,
    jsonb_build_object(
        'id',
        style.id,
        'external_id',
        style.external_id,
        'slug',
        style.slug,
        'number',
        style.number,
        'name',
        style.name
    ) AS "style",
    jsonb_build_object(
        'id',
        pricelist.id,
        'external_id',
        pricelist.external_id,
        'slug',
        pricelist.slug,
        'name',
        pricelist.name
    ) AS "list"
FROM
    price
INNER JOIN style ON style.id = price.style_id
INNER JOIN pricelist ON pricelist.id = price.list_id
WHERE
    price.organization_id = :organization_id
    AND (:ids::int[] IS NULL OR price.id = any(:ids))
    AND (:external_ids::text[] IS NULL OR price.external_id = any(:external_ids))
ORDER BY
    price.updated_at DESC;

--
--! get_price_id (id?, external_id?)
SELECT price.id
FROM
    price
WHERE
    price.organization_id = :organization_id
    AND (:id::int IS NULL OR price.id = :id)
    AND (:external_id::text IS NULL OR price.external_id = :external_id);

--
--! insert_price (uom?, external_id?)
INSERT INTO price (
    type,
    uom,
    currency,
    amount,
    "start",
    "end",
    style_id,
    list_id,
    external_id,
    organization_id,
    created_by)
VALUES (
    :type,
    :uom,
    :currency,
    :amount,
    :start,
    :end,
    :style_id,
    :list_id,
    :external_id,
    :organization_id,
    :created_by)
RETURNING
id;

--
--! update_price (type?, uom?, currency?, amount?, start?, end?, style_id?, list_id?, external_id?)
UPDATE
price
SET
    type = coalesce(:type, type),
    uom = coalesce(:uom, uom),
    currency = coalesce(:currency, currency),
    amount = coalesce(:amount, amount),
    "start" = coalesce(:start, "start"),
    "end" = coalesce(:end, "end"),
    style_id = coalesce(:style_id, style_id),
    list_id = coalesce(:list_id, list_id),
    external_id = coalesce(:external_id, external_id)
WHERE
    id = :id;

--
--! delete_price
DELETE FROM price
WHERE organization_id = :organization_id
      AND id = :id;
