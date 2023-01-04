import { Dispatch, SetStateAction } from "react";
import { Switch } from "@headlessui/react";
import { classNames } from "../../utils";
import { Trans } from "@lingui/macro";
import { ItemFilters } from "../../types/filters";

interface Props {
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
}

export default function ServiceItemFilter({
  itemFilters,
  setItemFilters,
}: Props) {
  function setServiceItem(newValue: boolean) {
    setItemFilters({ ...itemFilters, serviceItem: newValue });
  }

  return (
    <Switch.Group>
      <Switch.Label className="block text-sm font-medium text-gray-700">
        <Trans>Service item</Trans>
      </Switch.Label>
      <Switch
        checked={itemFilters.serviceItem}
        onChange={setServiceItem}
        className={classNames(
          itemFilters.serviceItem ? "bg-indigo-600" : "bg-gray-200",
          "relative inline-flex h-6 w-11 my-3 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
        )}
      >
        <span className="sr-only">
          <Trans>Service items</Trans>
        </span>
        <span
          aria-hidden="true"
          className={classNames(
            itemFilters.serviceItem ? "translate-x-5" : "translate-x-0",
            "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out",
          )}
        />
      </Switch>
    </Switch.Group>
  );
}
