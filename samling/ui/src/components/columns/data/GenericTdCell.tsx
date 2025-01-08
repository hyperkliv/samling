import { classNames } from "../../../utils";

interface Props {
  columnIndex: number;
  children: React.ReactNode;
}

export default function GenericTdCell({ columnIndex, children }: Props) {
  return (
    <td
      className={classNames(
        "px-3 py-4 text-sm text-gray-500",
        columnIndex === 0 ? "pl-6" : "",
      )}
    >
      {children}
    </td>
  );
}
