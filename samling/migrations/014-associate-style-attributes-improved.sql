-- Takes a style ID and an array of attribute IDs and associates them in the style_attribute table. Existing associations for the given style ID are deleted.
CREATE OR REPLACE FUNCTION associate_style_attributes (int, int[])
RETURNS TABLE (
    attribute_id int
)
AS $$
    DELETE FROM style_attribute WHERE style_id = $1 AND NOT attribute_id = any($2);

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
    ON CONFLICT ON CONSTRAINT style_id_attribute_id_uq DO UPDATE SET
        position = EXCLUDED.position
    RETURNING
    attribute_id;

$$
LANGUAGE sql;
