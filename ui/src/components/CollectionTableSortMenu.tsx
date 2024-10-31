import { Listbox, Transition } from "@headlessui/react";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";
import { t, Trans } from "@lingui/macro";
import { Dispatch, Fragment, SetStateAction, useMemo } from "react";
import { NestedStyleSortOrder } from "../types/api";
import { StyleSortOrderAlternative } from "../types/other";
import { classNames } from "../utils";
import { useLocalize } from "../i18n";

interface Params {
  sortOrder: NestedStyleSortOrder;
  setSortOrder: Dispatch<SetStateAction<NestedStyleSortOrder>>;
}

function allSortOrderAlternatives(): StyleSortOrderAlternative[] {
  return [
    { title: t`Number`, apiReference: NestedStyleSortOrder.NumberAsc },
    { title: t`Name`, apiReference: NestedStyleSortOrder.NameAsc },
    {
      title: t`Delivery period`,
      apiReference: NestedStyleSortOrder.DeliveryPeriodAsc,
    },
    {
      title: t`Delivery period (descending)`,
      apiReference: NestedStyleSortOrder.DeliveryPeriodDesc,
    },
  ];
}

export default function CollectionTableSortMenu({
  sortOrder,
  setSortOrder,
}: Params) {
  const { locale } = useLocalize();
  const allSortOrderAlternativesMemoed = useMemo(
    () => allSortOrderAlternatives(),
    [locale],
  );
  const activeAlternative = allSortOrderAlternativesMemoed.find(
    (alt) => alt.apiReference === sortOrder,
  ) as StyleSortOrderAlternative;
  const alternatives = allSortOrderAlternativesMemoed;
  return (
    <Listbox value={sortOrder} onChange={setSortOrder}>
      {({ open }) => (
        <>
          <Listbox.Label className="block text-sm font-medium text-gray-700">
            <Trans>Sort by</Trans>
          </Listbox.Label>
          <div className="relative mt-1">
            <Listbox.Button className="relative w-full cursor-default rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 text-left shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm">
              <span className="block truncate">{activeAlternative.title}</span>
              <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                <ChevronUpDownIcon
                  className="h-5 w-5 text-gray-400"
                  aria-hidden="true"
                />
              </span>
            </Listbox.Button>

            <Transition
              show={open}
              as={Fragment}
              leave="transition ease-in duration-100"
              leaveFrom="opacity-100"
              leaveTo="opacity-0"
            >
              <Listbox.Options className="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
                {alternatives.map((alt) => (
                  <Listbox.Option
                    key={alt.apiReference}
                    className={({ active }) =>
                      classNames(
                        active ? "text-white bg-indigo-600" : "text-gray-900",
                        "relative cursor-default select-none py-2 pl-3 pr-9",
                      )
                    }
                    value={alt.apiReference}
                  >
                    {({ selected, active }) => (
                      <>
                        <span
                          className={classNames(
                            selected ? "font-semibold" : "font-normal",
                            "block truncate",
                          )}
                        >
                          {alt.title}
                        </span>

                        {selected ? (
                          <span
                            className={classNames(
                              active ? "text-white" : "text-indigo-600",
                              "absolute inset-y-0 right-0 flex items-center pr-4",
                            )}
                          >
                            <CheckIcon className="h-5 w-5" aria-hidden="true" />
                          </span>
                        ) : null}
                      </>
                    )}
                  </Listbox.Option>
                ))}
              </Listbox.Options>
            </Transition>
          </div>
        </>
      )}
    </Listbox>
  );
}
