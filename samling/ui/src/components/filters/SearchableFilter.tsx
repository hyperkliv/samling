import { Combobox } from "@headlessui/react";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";
import { Dispatch, useMemo, useState } from "react";
import { cloudflareImageUrl } from "../../images";
import { classNames } from "../../utils";

export interface SearchableFilterItem {
  id: number | string;
  title: string;
  subtitle?: string | null;
  imageUrl?: string | null;
}

interface SearchableFilterProps {
  title: string;
  items: SearchableFilterItem[];
  onNewItem: Dispatch<SearchableFilterItem>;
}

export default function SearchableFilter({
  title,
  items,
  onNewItem,
}: SearchableFilterProps) {
  const [query, setQuery] = useState("");

  const anyImage = useMemo(() => {
    return items.some((x) => !!x.imageUrl);
  }, [items]);

  const filteredResults =
    query === ""
      ? items
      : items.filter((item) => {
          return [item.title, item.subtitle || ""]
            .join(" ")
            .toLowerCase()
            .includes(query.toLowerCase());
        });

  return (
    <Combobox as="div" value={null} onChange={onNewItem}>
      <Combobox.Label className="block text-sm font-medium text-gray-700">
        {title}
      </Combobox.Label>
      <div className="relative mt-1">
        <Combobox.Input
          className="w-full rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm"
          onChange={(event) => setQuery(event.target.value)}
          displayValue={(item?: SearchableFilterItem) => item?.title || ""}
        />
        <Combobox.Button className="absolute inset-y-0 right-0 flex items-center rounded-r-md px-2 focus:outline-none">
          <ChevronUpDownIcon
            className="h-5 w-5 text-gray-400"
            aria-hidden="true"
          />
        </Combobox.Button>

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
