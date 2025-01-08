import _ from "lodash";
import { NestedPrice } from "../../../types/api";
import { TdColumnProps } from "../../../types/columns";
import GenericTdCell from "./GenericTdCell";

export default function Prices({ item, columnIndex }: TdColumnProps) {
  // TODO: Warn if more than one price for a specific currency!
  const uniqueRetailPrices = uniquePrices(
    item.style.prices.filter((price) => price.type === "Retail"),
  );
  return (
    <GenericTdCell columnIndex={columnIndex}>
      <ul>
        {uniqueRetailPrices.map((price, index) => (
          <li key={index} className="col-span-1 flex rounded-md py-2">
            <span className="font-medium text-gray-900 pr-1">
              {price.amount}
            </span>
            <span className="text-gray-500">{price.currency}</span>
          </li>
        ))}
      </ul>
    </GenericTdCell>
  );
}

function uniquePrices(prices: NestedPrice[]): NestedPrice[] {
  return _.uniqWith(
    prices,
    (p1, p2) => p1.amount === p2.amount && p1.currency === p2.currency,
  );
}
