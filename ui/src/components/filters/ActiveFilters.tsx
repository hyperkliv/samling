import { t } from "@lingui/macro";
import { Trans } from "@lingui/macro";
import { Dispatch, SetStateAction, useMemo } from "react";
import { useLocalize } from "../../i18n";
import { cloudflareImageUrl, getStyleDisplayImage } from "../../images";
import { CategorySummary, CollectionItem } from "../../types/api";
import { ItemFilters } from "../../types/filters";
import LoadingIndicator from "../LoadingIndicator";

interface ActiveFilter {
  itemFiltersKey: "styleSlugs" | "categorySlugs" | "countryCodes" | "newItems";
  title: string;
  value: string;
  image: string | null;
  label: string | null;
}

interface Props {
  itemFilters: ItemFilters;
  items: CollectionItem[];
  allItems: CollectionItem[];
  allCategories: CategorySummary[];
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
}

export default function ActiveFilters({
  itemFilters,
  setItemFilters,
  allItems,
  allCategories,
}: Props) {
  const { i18nDbText } = useLocalize();

  const activeFilters: ActiveFilter[] = useMemo(() => {
    const styleFilters = itemFilters.styleSlugs.map((styleSlug) => {
      const item = allItems.find((needle) => needle.style.slug === styleSlug);
      if (!item) {
        return {
          itemFiltersKey: "styleSlugs",
          title: t`Style`,
          value: styleSlug,
          image: null,
          label: null,
        } as ActiveFilter;
      } else {
        const image = getStyleDisplayImage(item.style);
        return {
          itemFiltersKey: "styleSlugs",
          title: t`Style`,
          value: styleSlug,
          image: image ? cloudflareImageUrl(image.url, "thumbnail") : "",
          label: `${item.style.number} ${i18nDbText(item.style.name)}`,
        } as ActiveFilter;
      }
    });

    const categoryFilters = itemFilters.categorySlugs.map((categorySlug) => {
      const category = allCategories.find(
        (needle) => needle.slug === categorySlug,
      );
      return {
        itemFiltersKey: "categorySlugs",
        title: t`Category`,
        value: categorySlug,
        label: category ? i18nDbText(category.name) : null,
      } as ActiveFilter;
    });

    const countryFilters = itemFilters.countryCodes.map((countryCode) => {
      return {
        itemFiltersKey: "countryCodes",
        title: t`Country`,
        value: countryCode,
        label: countryCode,
      } as ActiveFilter;
    });

    const newFilters = itemFilters.newItems.map((newItem) => {
      return {
        itemFiltersKey: "newItems",
        title: t`New`,
        value: newItem,
        label: newItem === "styles" ? t`styles` : t`colors`,
      } as ActiveFilter;
    });

    return [
      ...styleFilters,
      ...categoryFilters,
      ...countryFilters,
      ...newFilters,
    ];
  }, [itemFilters, allCategories, allItems, i18nDbText]);

  function removeFilter(activeFilter: ActiveFilter) {
    const newItemFilters = {
      ...itemFilters,
      [activeFilter.itemFiltersKey]: itemFilters[
        activeFilter.itemFiltersKey
      ].filter((value) => value !== activeFilter.value),
    };
    setItemFilters(newItemFilters);
  }

  return (
    <div>
      <div className="my-3 sm:flex sm:items-center">
        <h3 className="text-sm font-medium text-gray-700">
          <Trans>
            Filters
            <span className="sr-only">, active</span>
          </Trans>
          <span className="hidden print:inline">:</span>
        </h3>

        <div
          aria-hidden="true"
          className="hidden h-5 w-px bg-gray-300 sm:ml-4 sm:block"
        />
        <div className="mt-2 sm:mt-0 sm:ml-4">
          <div className="-m-1 flex flex-wrap items-center">
            {activeFilters.map((activeFilter) => (
              <span
                key={activeFilter.value}
                className="m-1 inline-flex items-center rounded-full border border-gray-200 bg-white py-1.5 pl-3 pr-2 text-sm font-medium text-gray-900"
              >
                <span className="mr-1">{activeFilter.title}:</span>
                {activeFilter.image ? (
                  <span className="mr-1 ml-1">
                    <img
                      className="max-h-6"
                      loading="lazy"
                      src={activeFilter.image}
                      alt={t`${activeFilter.title} filter ${activeFilter.label}`}
                    />
                  </span>
                ) : (
                  ""
                )}
                {activeFilter.label ? (
                  <span>{activeFilter.label}</span>
                ) : (
                  <LoadingIndicator textColor="gray" fillColor="green" />
                )}
                <button
                  type="button"
                  onClick={() => removeFilter(activeFilter)}
                  className="ml-1 inline-flex h-4 w-4 flex-shrink-0 rounded-full p-1 text-gray-400 hover:bg-gray-200 hover:text-gray-500"
                >
                  <span className="sr-only">
                    <Trans>Remove filter for {activeFilter.label}</Trans>
                  </span>
                  <svg
                    className="h-2 w-2"
                    stroke="currentColor"
                    fill="none"
                    viewBox="0 0 8 8"
                  >
                    <path
                      strokeLinecap="round"
                      strokeWidth="1.5"
                      d="M1 1l6 6m0-6L1 7"
                    />
                  </svg>
                </button>
              </span>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
