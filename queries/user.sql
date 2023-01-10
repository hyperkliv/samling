-- User related queries
--
-- Entities:
--
--: UserRow(password_hash?, profile_image?, last_sign_in?)
--
--
-- Queries:
--
--! select_users (organization_id?, id?, email?, roles?, groups?) : UserRow
WITH group_summaries AS (
    SELECT
        "group".id,
        "group".slug,
        "group".external_id,
        "group".name,
        "group".description,
        coalesce(user_counts.n, 0) AS num_users,
        coalesce(collection_counts.n, 0) AS num_collections,
        coalesce(pricelist_counts.n, 0) AS num_price_lists
    FROM "group"
    LEFT JOIN LATERAL (
        SELECT
            group_user.group_id,
            count(*) AS n
        FROM group_user
        WHERE group_user.group_id = "group".id
        GROUP BY group_user.group_id
    ) AS user_counts ON user_counts.group_id = "group".id
    LEFT JOIN LATERAL (
        SELECT
            group_collection.group_id,
            count(*) AS n
        FROM
            group_collection
        WHERE group_collection.group_id = "group".id GROUP BY group_collection.group_id
    ) AS collection_counts ON collection_counts.group_id = "group".id
    LEFT JOIN LATERAL (
        SELECT
            group_pricelist.group_id,
            count(*) AS n
        FROM
            group_pricelist
        WHERE group_pricelist.group_id = "group".id GROUP BY group_pricelist.group_id
    ) AS pricelist_counts ON pricelist_counts.group_id = "group".id
    WHERE
        (:organization_id::int IS NULL OR "group".organization_id = :organization_id)
)

SELECT
    "user".*,
    coalesce(organizations.json_data, '[]'::jsonb) AS organizations,
    coalesce("groups".json_data, '[]'::jsonb) AS "groups"
FROM
    "user"
LEFT JOIN (
    SELECT
        user_organization.user_id,
        array_agg(organization.id) AS ids,
        jsonb_agg(
            jsonb_build_object(
                'organization',
                jsonb_build_object(
                    'id',
                    organization.id,
                    'name',
                    organization.name,
                    'logo_url',
                    organization.logo_url
                ),
                'role_ids',
                user_organization.role_ids
            )
        ) AS json_data
    FROM user_organization
    INNER JOIN
        organization ON organization.id = user_organization.organization_id
    GROUP BY user_organization.user_id
) AS organizations ON organizations.user_id = "user".id
LEFT JOIN (
    SELECT
        group_user.user_id,
        array_agg(group_user.group_id) AS group_ids,
        jsonb_agg(group_summaries.*) AS json_data
    FROM group_user
    INNER JOIN group_summaries
        ON group_summaries.id = group_user.group_id
    GROUP BY group_user.user_id
) AS "groups" ON "groups".user_id = "user".id
WHERE
    (:organization_id::int IS NULL OR :organization_id = any(organizations.ids))
    AND (:id::int IS NULL OR "user".id = :id)
    AND (:email::text IS NULL OR lower("user".email) = lower(:email))
    AND (
        :roles::int[] IS NULL
        OR jsonb_path_query_first(
            organizations.json_data,
            '$[*] ? (@.organization.id == $organization_id).role_ids',
            jsonb_build_object('organization_id', :organization_id)
        ) @> to_jsonb(:roles)
    )
    AND (:groups::int[] IS NULL OR "groups".group_ids @> :groups);

--
--! get_org_user_id
SELECT "user".id
FROM
    "user"
INNER JOIN user_organization
    ON user_organization.user_id = "user".id
WHERE
    user_organization.organization_id = :organization_id
    AND "user".id = :user_id;

--
--! get_role_ids
SELECT user_organization.role_ids
FROM
    user_organization
WHERE
    user_organization.user_id = :user_id
    AND user_organization.organization_id = :organization_id;

--
--! insert_user (password_hash?, profile_image?)
INSERT INTO "user" (
    name,
    email,
    password_hash,
    profile_image)
VALUES (
    :name,
    :email,
    :password_hash,
    :profile_image)
RETURNING
id;

--
--! update_user (name?, email?, password_hash?, profile_image?)
UPDATE
"user"
SET
    name = coalesce(:name, name),
    email = coalesce(:email, email),
    password_hash = coalesce(:password_hash, password_hash),
    profile_image = coalesce(:profile_image, profile_image)
WHERE
    id = :id;

--
--! delete_user
DELETE FROM "user"
WHERE id = :id;

--
--! upsert_user_organization (role_ids?)
INSERT INTO user_organization (
    user_id,
    organization_id,
    role_ids
)
VALUES (
    :user_id,
    :organization_id,
    :role_ids
)
ON CONFLICT ON CONSTRAINT user_organization_uq
DO UPDATE SET role_ids = coalesce(excluded.role_ids, user_organization.role_ids);

--
--! get_user_password_hash
SELECT "user".password_hash
FROM
    "user"
WHERE
    "user".id = :user_id;

--
--! update_last_sign_in
UPDATE "user" SET last_sign_in = now() WHERE id = :id;

--
--! replace_user_groups
SELECT *
FROM
    replace_user_groups(:user_id, :group_ids);
