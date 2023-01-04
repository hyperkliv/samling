import { Dispatch, SetStateAction } from "react";
import { Switch } from "@headlessui/react";
import { classNames } from "../../utils";
import { Trans } from "@lingui/macro";
import { ItemFilters } from "../../types/filters";

interface Props {
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
}

export default function CoreFilter({ itemFilters, setItemFilters }: Props) {
  function setCore(newValue: boolean) {
    setItemFilters({ ...itemFilters, core: newValue });
  }

  return (
    <Switch.Group>
      <Switch.Label className="block text-sm font-medium text-gray-700">
        <Trans>Core</Trans>
      </Switch.Label>
      <Switch
        checked={itemFilters.core}
        onChange={setCore}
        className={classNames(
          itemFilters.core ? "bg-indigo-600" : "bg-gray-200",
          "relative inline-flex h-6 w-11 my-3 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
        )}
      >
        <span className="sr-only">
          <Trans>Core styles</Trans>
        </span>
        <span
          aria-hidden="true"
          className={classNames(
            itemFilters.core ? "translate-x-5" : "translate-x-0",
            "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out",
          )}
        />
      </Switch>
    </Switch.Group>
  );
}
