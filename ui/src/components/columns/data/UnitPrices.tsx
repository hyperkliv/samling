import { TdColumnProps } from "../../../types/columns";
import { classNames } from "../../../utils";
import GenericTdCell from "./GenericTdCell";

export default function Prices({ item, columnIndex }: TdColumnProps) {
  return (
    <GenericTdCell columnIndex={columnIndex}>
      <ul>
        {item.style.prices
          .filter((price) => price.type === "Unit")
          .map((price, index) => (
            <li key={index} className="col-span-1 flex rounded-md py-2">
              <div
                className={classNames(
                  "bg-gray-400",
                  "flex-shrink-0 flex items-center justify-center text-white text-sm font-medium px-2 rounded-l-md print:hidden",
                )}
              >
                {price.list.name}
              </div>
              <div className="truncate rounded-r-md border-t border-r border-b print:border-0 border-gray-200 bg-white">
                <div className="truncate px-2 text-sm">
                  <span className="font-medium text-gray-900 pr-1">
                    {price.amount}
                  </span>
                  <span className="text-gray-500">{price.currency}</span>
                  <span className="text-gray-500 hidden print:inline">
                    {" "}
                    ({price.list.name})
                  </span>
                </div>
              </div>
            </li>
          ))}
      </ul>
    </GenericTdCell>
  );
}
