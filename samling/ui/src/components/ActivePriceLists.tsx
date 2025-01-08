import { Trans } from "@lingui/macro";
import { Dispatch, SetStateAction } from "react";
import { PriceListSummary } from "../types/api";

interface Props {
  priceLists: PriceListSummary[];
  setPriceLists: Dispatch<SetStateAction<PriceListSummary[]>>;
}

export default function ActivePriceLists({ priceLists, setPriceLists }: Props) {
  function removePriceList(priceList: PriceListSummary) {
    setPriceLists(priceLists.filter((pl) => pl.id !== priceList.id));
  }

  return (
    <div>
      <div className="my-3 sm:flex sm:items-center">
        <h3 className="text-sm font-medium text-gray-700">
          <Trans>
            Price lists
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
            {priceLists.map((priceList) => (
              <span
                key={priceList.id}
                className="m-1 inline-flex items-center rounded-full border border-gray-200 bg-white py-1.5 pl-3 pr-2 text-sm font-medium text-gray-900"
              >
                <span className="mr-1">{priceList.name}</span>
                <button
                  type="button"
                  onClick={() => removePriceList(priceList)}
                  className="ml-1 inline-flex h-4 w-4 flex-shrink-0 rounded-full p-1 text-gray-400 hover:bg-gray-200 hover:text-gray-500"
                >
                  <span className="sr-only">
                    <Trans>Remove {priceList.name}</Trans>
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
