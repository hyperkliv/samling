import { CollectionFilters } from "./api";

export const newItemsFilterValueChoices = ["styles", "colors"];
export type NewItemsFilterValue = "styles" | "colors";

export interface ItemFilters {
  styleSlugs: string[];
  categorySlugs: string[];
  countryCodes: string[];
  newItems: NewItemsFilterValue[];
  core: boolean;
  serviceItem: boolean;
}

export function itemFiltersFromSearchParams(
  searchParams: URLSearchParams,
): ItemFilters {
  return {
    categorySlugs: searchParams.getAll("category"),
    styleSlugs: searchParams.getAll("style"),
    countryCodes: searchParams.getAll("country"),
    newItems: searchParams
      .getAll("new")
      .filter(
        (value) => value === "styles" || value === "stylesAndColors",
      ) as NewItemsFilterValue[],
    core: searchParams.get("core") === "yes",
    serviceItem: searchParams.get("serviceItem") === "yes",
  };
}

export function itemFiltersToSearchParams(
  itemFilters: ItemFilters,
): Record<string, string | string[]> {
  const d = {
    style: itemFilters.styleSlugs,
    category: itemFilters.categorySlugs,
    country: itemFilters.countryCodes,
    new: itemFilters.newItems,
  } as Record<string, string | string[]>;
  if (itemFilters.core) {
    d.core = "yes";
  }
  if (itemFilters.serviceItem) {
    d.serviceItem = "yes";
  }
  return d;
}

export function makeCollectionFilters(
  itemFilters: ItemFilters,
  priceListSlugs: string[],
): CollectionFilters {
  return {
    styles: {
      categories:
        itemFilters.categorySlugs.length > 0
          ? itemFilters.categorySlugs.map((slug) => ({ slug }))
          : null,
      refs:
        itemFilters.styleSlugs.length > 0
          ? itemFilters.styleSlugs.map((slug) => ({ slug }))
          : null,
      pricelists: priceListSlugs.map((slug) => ({ slug })),
      core: itemFilters.core || null,
      service_item: itemFilters.serviceItem || null,
      new_styles: itemFilters.newItems.includes("styles") || null,
      new_colors: itemFilters.newItems.includes("colors") || null,
      country_of_origin:
        itemFilters.countryCodes.length > 0 ? itemFilters.countryCodes : null,
    },
  };
}
