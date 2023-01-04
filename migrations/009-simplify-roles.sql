DROP TABLE global_user_role;

ALTER TABLE user_organization ADD COLUMN role_ids int[] NOT NULL DEFAULT ARRAY[]::int[];

UPDATE user_organization
SET role_ids = (
    SELECT array_agg(user_organization_role.role_id) AS role_ids
    FROM user_organization_role
    WHERE
        user_organization_role.user_id = user_organization.user_id
        AND user_organization_role.organization_id = user_organization.organization_id
    GROUP BY user_organization_role.user_id, user_organization_role.organization_id
);

DROP TABLE user_organization_role;
