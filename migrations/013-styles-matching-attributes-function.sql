CREATE OR REPLACE FUNCTION styles_matching_attributes(attribute_ids int[])
RETURNS TABLE (style_id int) AS $$
DECLARE
    type_attrs record;
    where_clauses text[];
    sql text;
BEGIN
    IF attribute_ids IS NULL THEN
        RETURN;
    END IF;
    FOR type_attrs IN
        SELECT
            attributetype.id AS type_id,
            array_agg("attribute".id) AS attr_ids
        FROM "attribute"
        INNER JOIN attributetype ON attributetype.id = "attribute".type_id
        WHERE "attribute".id = any(attribute_ids)
        GROUP BY attributetype.id
    LOOP
        where_clauses := array_append(where_clauses, format('subset.attr_ids && %s', quote_literal(type_attrs.attr_ids)));
    END LOOP;
    IF coalesce(array_length(where_clauses, 1), 0) = 0 THEN
        RETURN;
    ELSE
        sql := format('WITH subset as (
            SELECT style_attribute.style_id, array_agg(style_attribute.attribute_id) AS attr_ids
            FROM
                style_attribute
            WHERE style_attribute.attribute_id = any($1)
            GROUP BY style_attribute.style_id
        ) SELECT style_id FROM subset
        WHERE %s', array_to_string(where_clauses, ' AND '));
        RETURN QUERY EXECUTE sql USING attribute_ids;
    END IF;
END;
$$ LANGUAGE plpgsql;
