import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";
import { Combobox } from "@headlessui/react";
import { classNames } from "../../utils";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { ItemFilters } from "../../types/filters";
import { Trans } from "@lingui/macro";

interface Props {
  allCountryCodes: string[];
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
}

export default function CountryCombobox({
  allCountryCodes,
  itemFilters,
  setItemFilters,
}: Props) {
  const [query, setQuery] = useState("");
  const [selectedItem, setSelectedItem] = useState(null);

  useEffect(() => {
    if (!!selectedItem) {
      setItemFilters({
        ...itemFilters,
        countryCodes: Array.from(
          new Set([...itemFilters.countryCodes, selectedItem]),
        ),
      });
      setSelectedItem(null);
      setQuery("");
    }
  }, [selectedItem, itemFilters, setItemFilters]);

  const queryResults = (
    query === ""
      ? allCountryCodes
      : allCountryCodes.filter((code) => {
          return code.toLowerCase().includes(query.toLowerCase());
        })
  ).filter((countryCode) => !itemFilters.countryCodes.includes(countryCode));

  return (
    <Combobox as="div" value={selectedItem} onChange={setSelectedItem}>
      <Combobox.Label className="block text-sm font-medium text-gray-700">
        <Trans>Country of origin</Trans>
      </Combobox.Label>
      <div className="relative mt-1">
        <Combobox.Input
          className="w-full rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm"
          onChange={(event) => setQuery(event.target.value)}
          displayValue={(code: string) => (code ? code : "")}
        />
        <Combobox.Button className="absolute inset-y-0 right-0 flex items-center rounded-r-md px-2 focus:outline-none">
          <ChevronUpDownIcon
            className="h-5 w-5 text-gray-400"
            aria-hidden="true"
          />
        </Combobox.Button>

        {queryResults.length > 0 && (
          <Combobox.Options className="absolute z-10 mt-1 max-h-56 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
            {queryResults.map((country) => (
              <CountryComboboxOption key={country} country={country} />
            ))}
          </Combobox.Options>
        )}
      </div>
    </Combobox>
  );
}

interface OptionProps {
  country: string;
}

function CountryComboboxOption({ country }: OptionProps) {
  return (
    <Combobox.Option
      key={country}
      value={country}
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
            <span
              className={classNames(
                "ml-3 truncate",
                selected ? "font-semibold" : "",
              )}
            >
              {country}
            </span>
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
  );
}
