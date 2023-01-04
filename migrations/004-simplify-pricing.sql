DROP FUNCTION replace_prices;
DROP TYPE dateprice;

ALTER TABLE priceset ADD COLUMN amount numeric ( 10, 2); -- TODO: NOT NULL!
ALTER TABLE priceset ADD COLUMN "start" date; -- TODO: NOT NULL!
ALTER TABLE priceset ADD COLUMN "end" date; -- TODO: NOT NULL!

CREATE TABLE price_table_deduplicated AS SELECT DISTINCT ON (
    price.priceset_id, price."start", price."end"
) * FROM price;
DELETE FROM price;
INSERT INTO price SELECT * FROM price_table_deduplicated;
DROP TABLE price_table_deduplicated;

INSERT INTO priceset (
    organization_id,
    type,
    currency,
    uom,
    list_id,
    external_id,
    style_id,
    created_by,
    created_at,
    updated_at,
    amount,
    "start",
    "end"
)
SELECT
    priceset.organization_id,
    priceset.type,
    priceset.currency,
    priceset.uom,
    priceset.list_id,
    concat('temp-', priceset.id, '-', price."start", '-', price."end") AS external_id,
    priceset.style_id,
    priceset.created_by,
    priceset.created_at,
    priceset.updated_at,
    price.amount,
    price."start",
    price."end"
FROM priceset INNER JOIN price ON price.priceset_id = priceset.id;

DELETE FROM priceset WHERE external_id NOT LIKE 'temp-%';

ALTER TABLE priceset ALTER COLUMN amount SET NOT NULL;
ALTER TABLE priceset ALTER COLUMN "start" SET NOT NULL;
ALTER TABLE priceset ALTER COLUMN "end" SET NOT NULL;

DROP TABLE price;
ALTER TABLE priceset RENAME TO price;
