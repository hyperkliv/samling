import { Listbox, Transition } from "@headlessui/react";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";
import { t, Trans } from "@lingui/macro";
import { Dispatch, Fragment, SetStateAction, useMemo } from "react";
import {
  ItemFilters,
  NewItemsFilterValue,
  newItemsFilterValueChoices,
} from "../../types/filters";
import { classNames } from "../../utils";
import { useLocalize } from "../../i18n";

interface Props {
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
}

interface NewAlternative {
  title: string;
  value: NewItemsFilterValue | null;
}

function alternatives(): NewAlternative[] {
  return [
    { title: " ", value: null },
    { title: t`Styles`, value: "styles" },
    { title: t`Colors`, value: "colors" },
  ];
}

export default function NewFilter({ itemFilters, setItemFilters }: Props) {
  const { locale } = useLocalize();
  const alternativesMemoed = useMemo(() => alternatives(), [locale]);
  function maybeSetNewItems(val: string) {
    if (newItemsFilterValueChoices.includes(val)) {
      setItemFilters({
        ...itemFilters,
        newItems: Array.from(
          new Set([...itemFilters.newItems, val]),
        ) as NewItemsFilterValue[],
      });
    }
  }

  return (
    <Listbox value={null} onChange={maybeSetNewItems}>
      {({ open }) => (
        <>
          <Listbox.Label className="block text-sm font-medium text-gray-700">
            <Trans>New</Trans>
          </Listbox.Label>
          <div className="relative mt-1">
            <Listbox.Button className="relative w-full cursor-default rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 text-left shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm">
              <span className="block truncate">- </span>
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
                {alternativesMemoed.map((alt) => (
                  <Listbox.Option
                    key={alt.value}
                    className={({ active }) =>
                      classNames(
                        active ? "text-white bg-indigo-600" : "text-gray-900",
                        "relative cursor-default select-none py-2 pl-3 pr-9",
                      )
                    }
                    value={alt.value}
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
