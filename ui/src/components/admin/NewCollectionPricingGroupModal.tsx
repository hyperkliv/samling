import { Dialog, Transition } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { t } from "@lingui/macro";
import { Dispatch, FormEvent, Fragment, SetStateAction, useState } from "react";
import { CollectionPricing, PriceListSummary } from "../../types/api";
import MultipleCombobox from "./MultipleCombobox";

export default function NewCollectionPricingGroupModal({
  open,
  setOpen,
  addPricingItems,
  availablePriceLists,
}: {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  addPricingItems: (pricingItems: CollectionPricing[]) => void;
  availablePriceLists: PriceListSummary[];
}) {
  const [date, setDate] = useState("");
  const [priceLists, setPriceLists] = useState([] as PriceListSummary[]);

  function onSubmitForm(evt: FormEvent<HTMLFormElement>) {
    evt.preventDefault();
    evt.stopPropagation();
    if (!!date) {
      addPricingItems(priceLists.map((list) => ({ date, list })));
      setPriceLists([]);
      setOpen(false);
    } else {
      window.alert(t`Please select a date.`);
      return;
    }
  }

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog as="div" className="relative z-10" onClose={() => setOpen(false)}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-2xl sm:p-6">
                <div className="absolute top-0 right-0 hidden pt-4 pr-4 sm:block">
                  <button
                    type="button"
                    className="rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    onClick={() => setOpen(false)}
                  >
                    <span className="sr-only">Close</span>
                    <XMarkIcon className="h-6 w-6" aria-hidden="true" />
                  </button>
                </div>
                <div>
                  <div className="px-4 py-5 sm:p-6">
                    <Dialog.Title
                      as="h3"
                      className="text-lg font-medium leading-6 text-gray-900"
                    >
                      {t`Add collection pricing date`}
                    </Dialog.Title>
                    <div className="mt-2 max-w-xl text-sm text-gray-500">
                      <p>{t`Make your selection of items to add here. Manual adjustments can be made once added.`}</p>
                    </div>
                    <form onSubmit={onSubmitForm}>
                      <div className="my-5">
                        <label className="text-sm" htmlFor={"new-pricing-date"}>
                          {t`Pricing date`}
                        </label>
                        <input
                          type="date"
                          className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                          name="new-pricing-date"
                          id="new-pricing-date"
                          value={date}
                          onChange={(evt) => setDate(evt.target.value)}
                        />
                      </div>
                      <div className="my-5">
                        <MultipleCombobox
                          title={t`Price lists`}
                          allItems={availablePriceLists}
                          selectedItems={priceLists}
                          setSelectedItems={setPriceLists}
                          toFilterItem={(priceList) => ({
                            id: priceList.id,
                            title: priceList.name,
                          })}
                        />
                      </div>
                      <div className="mt-6 sm:mt-6 sm:flex sm:flex-row-reverse">
                        <button
                          type="submit"
                          className="bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500 mt-3 inline-flex w-full items-center justify-center rounded-md border border-transparent px-4 py-2 font-medium text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
                        >
                          {t`Add`}
                        </button>
                        <button
                          type="button"
                          className="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:w-auto sm:text-sm"
                          onClick={() => setOpen(false)}
                        >
                          {t`Cancel`}
                        </button>
                      </div>
                    </form>
                  </div>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}
