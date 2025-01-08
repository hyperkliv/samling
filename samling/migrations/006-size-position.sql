ALTER TABLE "size" ADD COLUMN "position" smallint;
UPDATE "size" SET "position" = (
    CASE
        WHEN "number" LIKE '%¼'
            THEN cast(substr("number", 1, length("number") - 1) AS smallint) * 10 + 2
        WHEN "number" LIKE '%½'
            THEN cast(substr("number", 1, length("number") - 1) AS smallint) * 10 + 5
        WHEN "number" LIKE '%¾'
            THEN cast(substr("number", 1, length("number") - 1) AS smallint) * 10 + 8
        WHEN "number" IN ('', 'OS', 'ONESIZE') THEN 0
        WHEN "number" = 'S' THEN 1
        WHEN "number" = 'M' THEN 2
        WHEN "number" = 'L' THEN 3
        WHEN "number" = 'XL' THEN 4
        WHEN "number" = 'XXL' THEN 5
        WHEN "number" = 'XXXL' THEN 6
        ELSE
            cast("number" AS smallint) * 10
    END
);
ALTER TABLE "size" ALTER COLUMN "position" SET NOT NULL;
ALTER TABLE "size" ALTER COLUMN "position" SET DEFAULT 0;
