-- We try to ensure every table has `created_at` and `updated_at` columns, which can help immensely with debugging
-- and auditing.
--
-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select create_updated_at_trigger('<table name>');
--
-- after a `CREATE TABLE`.
CREATE OR REPLACE FUNCTION set_updated_at ()
RETURNS trigger
AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_updated_at_trigger (tablename regclass)
RETURNS void
AS $$
BEGIN
  EXECUTE format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
END;
$$
LANGUAGE plpgsql;

-- NOTE This table is only meant to contain one row!
CREATE TABLE "migrations" (
    "revision" integer NOT NULL
);

CREATE TABLE "user" (
    "id" serial PRIMARY KEY,
    "name" text NOT NULL,
    "email" text NOT NULL UNIQUE,
    "password_hash" text,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    "profile_image" text
);

SELECT create_updated_at_trigger('"user"');

CREATE TABLE "organization" (
    "id" serial PRIMARY KEY,
    "name" text NOT NULL,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    "logo_url" text
);

SELECT create_updated_at_trigger('organization');

CREATE TABLE "user_organization" (
    "user_id" integer NOT NULL REFERENCES "user" ("id") ON DELETE CASCADE,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    CONSTRAINT "user_organization_uq" UNIQUE ("user_id", "organization_id")
);

-- NOTE: Roles are defined in application code
CREATE TABLE "user_organization_role" (
    "user_id" integer NOT NULL REFERENCES "user" ("id") ON DELETE CASCADE,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "role_id" integer NOT NULL,
    CONSTRAINT "user_organization_role_uq" UNIQUE (
        "user_id", "organization_id", "role_id"
    )
);

-- NOTE: Roles are defined in application code
CREATE TABLE "global_user_role" (
    "user_id" integer NOT NULL REFERENCES "user" ("id") ON DELETE CASCADE,
    "role_id" integer NOT NULL,
    CONSTRAINT "global_user_role_uq" UNIQUE ("user_id", "role_id")
);

CREATE TABLE "group" (
    "id" serial PRIMARY KEY,
    "slug" text NOT NULL,
    "external_id" text,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "name" text NOT NULL,
    "description" text NOT NULL,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "group_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "group_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);
SELECT create_updated_at_trigger('group');

CREATE TABLE "group_user" (
    "user_id" integer NOT NULL REFERENCES "user" ("id") ON DELETE CASCADE,
    "group_id" integer NOT NULL REFERENCES "group" ("id") ON DELETE CASCADE,
    CONSTRAINT "group_user_uq" UNIQUE ("user_id", "group_id")
);

CREATE TABLE "collection" (
    "id" serial PRIMARY KEY,
    "organization_id" integer REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "slug" text NOT NULL,
    "external_id" text,
    "name" jsonb NOT NULL DEFAULT '{}',
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    "image_url" text,
    "acronym" jsonb NOT NULL DEFAULT '{}',
    CONSTRAINT "collection_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "collection_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('collection');

CREATE TABLE "group_collection" (
    "collection_id" integer NOT NULL REFERENCES "collection" ("id") ON DELETE CASCADE,
    "group_id" integer NOT NULL REFERENCES "group" ("id") ON DELETE CASCADE,
    CONSTRAINT "group_collection_uq" UNIQUE ("collection_id", "group_id")
);

CREATE TABLE "style" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "slug" text NOT NULL,
    "external_id" text,
    "number" text NOT NULL,
    "name" jsonb NOT NULL DEFAULT '{}',
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    "description" jsonb NOT NULL DEFAULT '{}',
    "core" boolean,
    "country_of_origin" text,
    "tariff_no" text,
    CONSTRAINT "style_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "style_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('style');

CREATE TABLE "color" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "slug" text NOT NULL,
    "external_id" text,
    "style_id" integer NOT NULL REFERENCES "style" ("id") ON DELETE CASCADE,
    "number" text NOT NULL,
    "name" jsonb NOT NULL DEFAULT '{}',
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "color_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "color_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('color');

CREATE TABLE "size" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "slug" text NOT NULL,
    "external_id" text,
    "color_id" integer NOT NULL REFERENCES "color" ("id") ON DELETE CASCADE,
    "number" text NOT NULL,
    "name" jsonb NOT NULL DEFAULT '{}',
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    "service_item" boolean,
    "delivery_period" date,
    CONSTRAINT "size_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "size_slug_organization_id_uq" UNIQUE ("slug", "organization_id")
);

SELECT create_updated_at_trigger('size');

CREATE TABLE "size_collection" (
    "size_id" integer NOT NULL REFERENCES "size" ("id") ON DELETE CASCADE,
    "collection_id" integer NOT NULL REFERENCES "collection" (
        "id"
    ) ON DELETE CASCADE,
    "position" integer NOT NULL
);

CREATE TABLE "new_collection_style" (
    "collection_id" integer NOT NULL REFERENCES "collection" (
        "id"
    ) ON DELETE CASCADE,
    "style_id" integer NOT NULL REFERENCES "style" ("id") ON DELETE CASCADE,
    "is_new" boolean NOT NULL,
    CONSTRAINT "new_collection_style_collection_id_style_id_uq" UNIQUE (
        "collection_id", "style_id"
    )
);

CREATE TABLE "new_collection_color" (
    "collection_id" integer NOT NULL REFERENCES "collection" (
        "id"
    ) ON DELETE CASCADE,
    "color_id" integer NOT NULL REFERENCES "color" ("id") ON DELETE CASCADE,
    "is_new" boolean NOT NULL,
    CONSTRAINT "new_collection_style_collection_id_color_id_uq" UNIQUE (
        "collection_id", "color_id"
    )
);

CREATE TABLE "image" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "url" text NOT NULL,
    "external_id" text,
    "external_checksum" text,
    "position" int NOT NULL,
    "color_id" integer NOT NULL REFERENCES "color" ("id") ON DELETE CASCADE,
    "uploaded_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "uploaded_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "image_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    )
);

SELECT create_updated_at_trigger('image');

CREATE TABLE "attributetype" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "name" jsonb NOT NULL DEFAULT '{}',
    "slug" text NOT NULL,
    "external_id" text,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "attributetype_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "attributetype_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('attributetype');

CREATE TABLE "attribute" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "type_id" integer NOT NULL REFERENCES "attributetype" (
        "id"
    ) ON DELETE CASCADE,
    "title" jsonb NOT NULL DEFAULT '{}',
    "description" jsonb NOT NULL DEFAULT '{}',
    "slug" text NOT NULL,
    "external_id" text,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "attribute_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "attribute_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('attribute');

CREATE TABLE "style_attribute" (
    "style_id" integer NOT NULL REFERENCES "style" ("id") ON DELETE CASCADE,
    "attribute_id" integer NOT NULL REFERENCES "attribute" (
        "id"
    ) ON DELETE CASCADE,
    "position" integer NOT NULL,
    CONSTRAINT "style_id_attribute_id_uq" UNIQUE ("style_id", "attribute_id")
);

CREATE TYPE pricelistcategory AS ENUM (
    'PreBook',
    'AtOnce',
    'Blanket',
    'Direct',
    'B2C'
);

CREATE TABLE "pricelist" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "name" text NOT NULL,
    "slug" text NOT NULL,
    "external_id" text,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    "category" pricelistcategory NOT NULL,
    CONSTRAINT "pricelist_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "pricelist_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('pricelist');

CREATE TABLE "group_pricelist" (
    "pricelist_id" integer NOT NULL REFERENCES "pricelist" ("id") ON DELETE CASCADE,
    "group_id" integer NOT NULL REFERENCES "group" ("id") ON DELETE CASCADE,
    CONSTRAINT "group_pricelist_uq" UNIQUE ("pricelist_id", "group_id")
);

CREATE TYPE pricetype AS ENUM (
    'Unit',
    'Retail'
);

CREATE TYPE dateprice AS (
    "amount" numeric (10, 2),
    "start" date,
    "end" date
);

CREATE TABLE "priceset" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "type" pricetype NOT NULL,
    "currency" text NOT NULL,
    "uom" text,
    "list_id" integer NOT NULL REFERENCES "pricelist" ("id") ON DELETE CASCADE,
    "external_id" text,
    "style_id" integer NOT NULL REFERENCES "style" ("id") ON DELETE CASCADE,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "price_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    )
);

SELECT create_updated_at_trigger('priceset');

CREATE TABLE "price" (
    "priceset_id" integer NOT NULL REFERENCES "priceset" (
        "id"
    ) ON DELETE CASCADE,
    "amount" numeric(10, 2) NOT NULL,
    "start" date NOT NULL,
    "end" date NOT NULL
);

CREATE TABLE "collectionpricing" (
    "id" serial PRIMARY KEY,
    "collection_id" integer NOT NULL REFERENCES "collection" ("id") ON DELETE CASCADE,
    "pricelist_category" pricelistcategory NOT NULL,
    "price_date" date NOT NULL,
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "collection_pricelist_category_uq" UNIQUE (
        "collection_id", "pricelist_category"
    )
);

CREATE TYPE collectionpricingvalue AS (
    "pricelist_category" pricelistcategory,
    "price_date" date
);

CREATE TABLE "category" (
    "id" serial PRIMARY KEY,
    "organization_id" integer NOT NULL REFERENCES "organization" (
        "id"
    ) ON DELETE CASCADE,
    "slug" text NOT NULL,
    "external_id" text,
    "name" jsonb NOT NULL DEFAULT '{}',
    "created_by" integer REFERENCES "user" ("id") ON DELETE SET NULL,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT "category_external_id_organization_id_uq" UNIQUE (
        "external_id", "organization_id"
    ),
    CONSTRAINT "category_slug_organization_id_uq" UNIQUE (
        "slug", "organization_id"
    )
);

SELECT create_updated_at_trigger('category');

CREATE TABLE "style_category" (
    "style_id" integer NOT NULL REFERENCES "style" ("id") ON DELETE CASCADE,
    "category_id" integer NOT NULL REFERENCES "category" (
        "id"
    ) ON DELETE CASCADE,
    "position" integer NOT NULL,
    CONSTRAINT "style_id_category_id_uq" UNIQUE ("style_id", "category_id")
);

-- Set migrate revision
CREATE OR REPLACE FUNCTION set_migrate_revision (int)
RETURNS int
AS $$
    DELETE FROM migrations;
    INSERT INTO migrations VALUES ($1) RETURNING $1;
$$
LANGUAGE sql;

-- Takes a style ID and an array of category IDs and associates them in the style_category table. Existing associations for the given style ID are deleted.
CREATE OR REPLACE FUNCTION associate_style_categories (int, int[])
RETURNS TABLE (
    category_id int
)
AS $$
    DELETE FROM style_category WHERE style_id = $1;

    INSERT INTO style_category (
        style_id,
        category_id,
        position
    )
    SELECT
        $1,
        t.category_id,
        t.position
    FROM
        unnest($2) WITH ORDINALITY AS t (category_id, position)
    RETURNING
        category_id;

$$
LANGUAGE sql;

-- Takes a style ID and an array of attribute IDs and associates them in the style_attribute table. Existing associations for the given style ID are deleted.
CREATE OR REPLACE FUNCTION associate_style_attributes (int, int[])
RETURNS TABLE (
    attribute_id int
)
AS $$
    DELETE FROM style_attribute WHERE style_id = $1;

    INSERT INTO style_attribute (
        style_id,
        attribute_id,
        position
    )
    SELECT
        $1,
        t.attribute_id,
        t.position
    FROM
        unnest($2) WITH ORDINALITY AS t (attribute_id, position)
    RETURNING
    attribute_id;

$$
LANGUAGE sql;

-- Takes a collection ID and an array of size IDs and associates them in the size_collection table. Existing associations for the given collection ID are deleted.
CREATE OR REPLACE FUNCTION associate_collection_sizes (int, int[])
RETURNS int
AS $$
    DELETE FROM size_collection WHERE collection_id = $1;

    INSERT INTO size_collection (
        collection_id,
        size_id,
        position
    )
    SELECT
        $1,
        t.size_id,
        t.position
    FROM
        unnest($2) WITH ORDINALITY AS t (size_id, position);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;

-- Takes a collection ID and an array of style IDs and associates them in the new_collection_style table. Existing associations for the given collection ID are deleted.
CREATE OR REPLACE FUNCTION set_new_collection_styles (int, int[])
RETURNS int
AS $$
    DELETE FROM new_collection_style WHERE collection_id = $1;

    INSERT INTO new_collection_style (
        collection_id,
        style_id,
        is_new
    )
    SELECT
        $1,
        t.style_id,
        true
    FROM
        unnest($2) AS t (style_id);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;

-- Takes a collection ID and an array of color IDs and associates them in the new_collection_color table. Existing associations for the given collection ID are deleted.
CREATE OR REPLACE FUNCTION set_new_collection_colors (int, int[])
RETURNS int
AS $$
    DELETE FROM new_collection_color WHERE collection_id = $1;

    INSERT INTO new_collection_color (
        collection_id,
        color_id,
        is_new
    )
    SELECT
        $1,
        t.color_id,
        true
    FROM
        unnest($2) AS t (color_id);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;

-- Takes a group ID and an array of user IDs and associates them in the group_user table. Existing associations for the given group ID are deleted.
CREATE OR REPLACE FUNCTION replace_group_users (int, int[])
RETURNS int
AS $$
    DELETE FROM group_user WHERE group_id = $1;

    INSERT INTO group_user (
        group_id,
        user_id
    )
    SELECT
        $1,
        t.user_id
    FROM
        unnest($2) AS t (user_id);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;

-- Takes a group ID and an array of collection IDs and associates them in the group_collection table. Existing associations for the given group ID are deleted.
CREATE OR REPLACE FUNCTION replace_group_collections (int, int[])
RETURNS int
AS $$
    DELETE FROM group_collection WHERE group_id = $1;

    INSERT INTO group_collection (
        group_id,
        collection_id
    )
    SELECT
        $1,
        t.collection_id
    FROM
        unnest($2) AS t (collection_id);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;

-- Takes a group ID and an array of pricelist IDs and associates them in the group_pricelist table. Existing associations for the given group ID are deleted.
CREATE OR REPLACE FUNCTION replace_group_pricelists (int, int[])
RETURNS int
AS $$
    DELETE FROM group_pricelist WHERE group_id = $1;

    INSERT INTO group_pricelist (
        group_id,
        pricelist_id
    )
    SELECT
        $1,
        t.pricelist_id
    FROM
        unnest($2) AS t (pricelist_id);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;


-- Takes a priceset ID and an array of dateprice composite type values, and creates them. Existing prices for the given priceset ID are deleted.
CREATE OR REPLACE FUNCTION replace_prices (int, dateprice[])
RETURNS int
AS $$
    DELETE FROM price WHERE priceset_id = $1;

    INSERT INTO price
    SELECT
        $1 AS priceset_id,
        p."amount",
        p."start",
        p."end"
    FROM
        unnest($2::dateprice[]) p;

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;


-- Takes a collection ID and an array of collectionpricing composite type values, and creates them. Existing collectionpricing entries for the given collection ID are deleted.
CREATE OR REPLACE FUNCTION replace_collection_pricing (int, collectionpricingvalue[])
RETURNS int
AS $$
    DELETE FROM collectionpricing WHERE collection_id = $1;

    INSERT INTO collectionpricing (collection_id, pricelist_category, price_date)
    SELECT
        $1 AS collection_id,
        cpv.pricelist_category,
        cpv.price_date
    FROM
        unnest($2::collectionpricingvalue[]) cpv;

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;


-- Ensure that superuser group exists and has access to all collections and pricelists
CREATE OR REPLACE FUNCTION ensure_superuser_access (org_id int)
RETURNS int
AS $$
DECLARE superuser_group_id int;
BEGIN
    INSERT INTO "group" (
        organization_id,
        name,
        description,
        slug,
        external_id
    ) VALUES (org_id, 'Superusers', 'Has full access to pricelists and collections within this organization.', 'superusers', 'superusers')
    -- TODO: Is it possible to also check for organization_id+slug conflict?
    ON CONFLICT ON CONSTRAINT group_external_id_organization_id_uq DO UPDATE SET
        name = EXCLUDED.name,
        slug = EXCLUDED.slug,
        external_id = EXCLUDED.external_id
    RETURNING
        id INTO superuser_group_id;

    INSERT INTO group_collection (group_id, collection_id)
        SELECT superuser_group_id AS group_id, id AS collection_id FROM collection ON CONFLICT DO NOTHING;
    INSERT INTO group_pricelist (group_id, pricelist_id)
        SELECT superuser_group_id AS group_id, id AS pricelist_id FROM pricelist ON CONFLICT DO NOTHING;

    RETURN superuser_group_id;
END
$$
LANGUAGE plpgsql;

-- Used to speed up joins between collection, pricelist and extracting the corresponding price_date
CREATE MATERIALIZED VIEW collection_pricelist_date AS
SELECT DISTINCT
    collectionpricing.collection_id,
    priceset.list_id,
    collectionpricing.price_date
FROM collectionpricing
INNER JOIN
    size_collection ON size_collection.collection_id = collectionpricing.collection_id
INNER JOIN size ON size.id = size_collection.size_id
INNER JOIN color ON color.id = size.color_id
INNER JOIN priceset ON priceset.style_id = color.style_id
INNER JOIN pricelist ON pricelist.id = priceset.list_id
WHERE pricelist.category = collectionpricing.pricelist_category
ORDER BY
    collectionpricing.collection_id, priceset.list_id, collectionpricing.price_date;
