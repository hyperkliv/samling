import { ThColumnProps } from "../../../types/columns";
import { classNames } from "../../../utils";

export default function GenericThCell({ column, columnIndex }: ThColumnProps) {
  return (
    <th
      key={columnIndex}
      scope="col"
      className={classNames(
        "py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900",
        columnIndex === 0 ? "pl-6" : "",
      )}
    >
      {column.title}
    </th>
  );
}
