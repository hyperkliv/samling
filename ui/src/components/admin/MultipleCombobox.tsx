import { Combobox } from "@headlessui/react";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";
import { Trans } from "@lingui/macro";
import { Dispatch, SetStateAction, useMemo, useRef, useState } from "react";
import { cloudflareImageUrl } from "../../images";
import { classNames } from "../../utils";
import { SearchableFilterItem } from "../filters/SearchableFilter";

const DEFAULT_NUM_SELECTED_VISIBLE = 3;

interface MultipleComboboxProps<T> {
  title?: string;
  allItems: T[];
  selectedItems: T[];
  setSelectedItems: Dispatch<SetStateAction<T[]>>;
  toFilterItem: (item: T) => SearchableFilterItem;
  numSelectedVisible?: number;
}

export default function MultipleCombobox<T>({
  title,
  allItems,
  selectedItems,
  setSelectedItems,
  toFilterItem,
  numSelectedVisible = DEFAULT_NUM_SELECTED_VISIBLE,
}: MultipleComboboxProps<T>) {
  const [query, setQuery] = useState("");
  const comboboxButton = useRef<HTMLButtonElement>(null);
  const allItemsMap = useMemo(() => {
    const map: Map<T, SearchableFilterItem> = new Map();
    allItems.forEach((item) => {
      map.set(item, toFilterItem(item));
    });
    return map;
  }, [allItems, toFilterItem]);
  const allFilterItemsMap = useMemo(() => {
    return new Map(
      Array.from(allItemsMap.entries()).map(([item, filterItem]) => [
        filterItem,
        item,
      ]),
    );
  }, [allItemsMap]);
  const allFilterItems = useMemo(
    () => Array.from(allItemsMap.values()),
    [allItemsMap],
  );
  const selectedFilterItems = useMemo(
    () =>
      selectedItems
        .map((item) => allItemsMap.get(item))
        .filter((x) => x !== undefined) as SearchableFilterItem[],
    [allItemsMap, selectedItems],
  );

  const anyImage = useMemo(() => {
    return allFilterItems.some((i) => !!i.imageUrl);
  }, [allFilterItems]);

  const filteredResults = useMemo(() => {
    if (query === "") {
      return allFilterItems.filter((_, idx) => idx < 50);
    } else {
      return allFilterItems.reduce((results, item) => {
        if (results.length > 50) {
          return results;
        }
        if (
          [item.title, item.subtitle || ""]
            .join(" ")
            .toLowerCase()
            .includes(query.toLowerCase())
        ) {
          results.push(item);
        }
        return results;
      }, [] as SearchableFilterItem[]);
    }
  }, [query, allFilterItems]);

  function remove(filterItem: SearchableFilterItem) {
    const item = allFilterItemsMap.get(filterItem);
    if (item !== undefined) {
      setSelectedItems(selectedItems.filter((otherItem) => otherItem !== item));
    }
  }

  return (
    <Combobox
      as="div"
      value={selectedFilterItems}
      onChange={(filterItems) => {
        const items = filterItems.flatMap((filterItem) => {
          const found = allFilterItemsMap.get(filterItem);
          // This can happen on module hot reloads for some reason
          return found === undefined ? [] : [found];
        });
        setSelectedItems(items);
        setQuery("");
        comboboxButton.current?.click();
      }}
      multiple
    >
      {title && (
        <Combobox.Label className="block text-sm font-medium text-gray-700">
          {title}
        </Combobox.Label>
      )}
      <div className="relative mt-1">
        <Combobox.Input
          className="w-full rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm"
          displayValue={() => query}
          onChange={(event) => setQuery(event.target.value)}
        />
        <div className="absolute inset-y-0 right-0 flex items-center px-2">
          {selectedFilterItems
            .filter((_, idx) => idx < numSelectedVisible)
            .map((filterItem) => (
              <span
                key={filterItem.id}
                className="inline-flex items-center rounded-full bg-indigo-100 mx-0.5 px-2 py-0.5 text-xs text-indigo-800"
              >
                <span
                  style={{ maxWidth: 150 }}
                  className="overflow-hidden whitespace-nowrap"
                >
                  {filterItem.title}
                </span>
                <button
                  type="button"
                  onClick={() => remove(filterItem)}
                  className="ml-0.5 inline-flex h-4 w-4 flex-shrink-0 items-center justify-center rounded-full text-gray-400 hover:bg-indigo-200 hover:text-gray-500 focus:bg-indigo-500 focus:text-white focus:outline-none"
                >
                  <span className="sr-only">
                    <Trans>Remove filter</Trans>
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
          {selectedFilterItems.length > numSelectedVisible && (
            <span className="mx-0.5 px-2 py-0.5 text-xs text-indigo-800 overflow-hidden whitespace-nowrap">{`+ ${
              selectedFilterItems.length - numSelectedVisible
            }`}</span>
          )}
          <Combobox.Button
            ref={comboboxButton}
            className="flex items-center rounded-r-md focus:outline-none"
          >
            <ChevronUpDownIcon
              className="h-5 w-5 text-gray-400"
              aria-hidden="true"
            />
          </Combobox.Button>
        </div>

        {filteredResults.length > 0 && (
          <Combobox.Options className="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
            {filteredResults.map((item) => (
              <Combobox.Option
                key={item.id}
                value={item}
                className={({ active }) =>
                  classNames(
                    "relative cursor-default select-none py-2 pl-3 pr-9",
                    active ? "bg-indigo-600 text-white" : "text-gray-900",
                  )
                }
              >
                {({ active, selected }) => (
                  <>
                    <div className="flex items-center">
                      {anyImage ? (
                        item.imageUrl ? (
                          <img
                            src={cloudflareImageUrl(item.imageUrl, "thumbnail")}
                            loading="lazy"
                            alt=""
                            className="h-12 w-12 flex-shrink-0 rounded-full"
                          />
                        ) : (
                          <span className="h-12 w-12"></span>
                        )
                      ) : (
                        ""
                      )}
                      <span
                        className={classNames(
                          "ml-3 truncate",
                          selected ? "font-semibold" : "",
                        )}
                      >
                        {item.title}
                      </span>
                      {item.subtitle ? (
                        <span
                          className={classNames(
                            "ml-3 truncate text-gray-300",
                            selected ? "font-semibold" : "",
                          )}
                        >
                          {item.subtitle}
                        </span>
                      ) : (
                        ""
                      )}
                    </div>

                    {selected && (
                      <span
                        className={classNames(
                          "absolute inset-y-0 right-0 flex items-center pr-4",
                          active ? "text-white" : "text-indigo-600",
                        )}
                      >
                        <CheckIcon className="h-5 w-5" aria-hidden="true" />
                      </span>
                    )}
                  </>
                )}
              </Combobox.Option>
            ))}
          </Combobox.Options>
        )}
      </div>
    </Combobox>
  );
}
