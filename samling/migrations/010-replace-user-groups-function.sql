-- Takes a user ID and an array of group IDs and associates them in the group_user table. Existing associations for the given user ID are deleted.
CREATE OR REPLACE FUNCTION replace_user_groups (int, int[])
RETURNS int
AS $$
    DELETE FROM group_user WHERE user_id = $1;

    INSERT INTO group_user (
        user_id,
        group_id
    )
    SELECT
        $1,
        t.group_id
    FROM
        unnest($2) AS t (group_id);

    SELECT coalesce(array_length($2, 1), 0);

$$
LANGUAGE sql;
