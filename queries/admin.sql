-- Admin related queries
--
-- Entities:
--
--: StringFilterChoiceRow()
--
--: EntityFilterChoiceRow(subtitle?, image?)
--
--
-- Queries:
--
--! select_style_filter_choices : EntityFilterChoiceRow
SELECT
    style.id,
    style."name" AS title,
    json_build_object('en', style."number") AS subtitle,
    to_json(main_image.json_data) AS image
FROM style
LEFT JOIN (
    SELECT
        color.style_id,
        row_number()
        OVER (PARTITION BY color.style_id ORDER BY image.uploaded_at DESC)
        AS rowno,
        jsonb_build_object(
            'id',
            image.id,
            'external_id',
            image.external_id,
            'url',
            image.url
        ) AS json_data
    FROM color
    INNER JOIN image ON image.color_id = color.id
    WHERE image.position = 1
) AS main_image ON main_image.style_id = style.id AND main_image.rowno = 1
WHERE style.organization_id = :organization_id
ORDER BY title;

--! select_category_filter_choices : EntityFilterChoiceRow
SELECT
    category.id,
    category."name" AS title,
    NULL::json AS subtitle,
    NULL::json AS image
FROM category WHERE category.organization_id = :organization_id
ORDER BY title;


--! select_status_filter_choices : StringFilterChoiceRow
SELECT DISTINCT size.status AS title FROM size
WHERE size.organization_id = :organization_id
ORDER BY title;
