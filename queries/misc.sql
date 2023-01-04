-- Miscellaneous queries
--
-- Queries:
--
--! migrate_revision
SELECT migrations.revision
FROM
    migrations;

--! set_migrate_revision
SELECT set_migrate_revision(:revision);
