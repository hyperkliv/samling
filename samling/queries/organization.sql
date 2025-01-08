-- Organization related queries
--
-- Entities:
--
--: OrganizationRow(created_by?, logo_url?)
--
--
-- Queries:

--! get_organization : OrganizationRow
SELECT *
FROM
    organization
WHERE
    organization.id = :id;

--
--! get_organization_id
SELECT organization.id
FROM
    organization
WHERE
    organization.id = :id;

--
--! insert_organization (logo_url?) : OrganizationRow
INSERT INTO organization (
    name,
    logo_url,
    created_by)
VALUES (
    :name,
    :logo_url,
    :created_by)
RETURNING
*;

--
--! update_organization (name?, logo_url?) : OrganizationRow
UPDATE
organization
SET
    name = coalesce(:name, name),
    logo_url = coalesce(:logo_url, logo_url)
WHERE
    id = :id
RETURNING
*;

--
--! delete_organization
DELETE FROM organization
WHERE id = :id
RETURNING
id;

--
--! list_user_organizations : OrganizationRow
SELECT organization.*
FROM
    organization
INNER JOIN
    user_organization ON organization.id = user_organization.organization_id
WHERE
    user_organization.user_id = :user_id;
