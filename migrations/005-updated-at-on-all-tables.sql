-- Add `updated_at` column to all tables that were missing it
ALTER TABLE "global_user_role"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "group_collection"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "group_pricelist"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "group_user"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "new_collection_color"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "new_collection_style"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "size_collection"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "style_attribute"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "style_category"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "user_organization_role"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();
ALTER TABLE "user_organization"
ADD COLUMN "updated_at" timestamptz NOT NULL DEFAULT now();

SELECT create_updated_at_trigger('group_collection');
SELECT create_updated_at_trigger('group_pricelist');
SELECT create_updated_at_trigger('group_user');
SELECT create_updated_at_trigger('new_collection_color');
SELECT create_updated_at_trigger('new_collection_style');
SELECT create_updated_at_trigger('size_collection');
SELECT create_updated_at_trigger('style_attribute');
SELECT create_updated_at_trigger('style_category');
SELECT create_updated_at_trigger('user_organization_role');
SELECT create_updated_at_trigger('user_organization');
