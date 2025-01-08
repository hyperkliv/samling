REFRESH MATERIALIZED VIEW collection_pricelist_date;

-- This will be the new table, which is essentially the same as the materialized view
CREATE TABLE collection_pricelist (
    "collection_id" integer NOT NULL REFERENCES "collection" ("id") ON DELETE CASCADE,
    "pricelist_id" integer NOT NULL REFERENCES "pricelist" ("id") ON DELETE CASCADE,
    "price_date" date NOT NULL,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "collection_pricelist_uq" UNIQUE (
        "collection_id", "pricelist_id"
    )
);

INSERT INTO collection_pricelist (collection_id, pricelist_id, price_date)
SELECT
    collection_pricelist_date.collection_id,
    collection_pricelist_date.list_id,
    collection_pricelist_date.price_date
FROM collection_pricelist_date;

DROP FUNCTION replace_collection_pricing;
DROP MATERIALIZED VIEW collection_pricelist_date;
DROP TABLE collectionpricing;
DROP TYPE collectionpricingvalue;
ALTER TABLE pricelist DROP COLUMN category;
DROP TYPE pricelistcategory;

CREATE TYPE collection_pricelist_relation AS (
    "pricelist_id" integer,
    "price_date" date,
    "created_by" integer
);


-- Takes a collection ID and an array of collection_pricelist_relation composite type values, and updates collection_pricelist accordingly. Existing collection_pricelist entries for the given collection ID are deleted.
CREATE OR REPLACE FUNCTION replace_collection_pricelists (
    int,
    collection_pricelist_relation[]
)
RETURNS int
AS $$
    DELETE FROM collection_pricelist WHERE collection_id = $1 AND pricelist_id NOT IN (SELECT pricelist_id FROM unnest($2::collection_pricelist_relation[]));

    INSERT INTO collection_pricelist (collection_id, pricelist_id, price_date, created_by)
    SELECT
        $1 AS collection_id,
        rel.pricelist_id,
        rel.price_date,
        rel.created_by
    FROM
        unnest($2::collection_pricelist_relation[]) rel
    ON CONFLICT ON CONSTRAINT collection_pricelist_uq
    DO UPDATE SET price_date = excluded.price_date;

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;
