import { Dispatch, SetStateAction, useMemo } from "react";
import { PriceListSummary } from "../../types/api";
import MultipleCombobox from "../admin/MultipleCombobox";

interface AssociatedPriceListsEditorProps {
  priceLists: PriceListSummary[];
  setPriceLists: Dispatch<SetStateAction<PriceListSummary[]>>;
  availablePriceLists: PriceListSummary[];
}

export default function AssociatedPriceListsEditor({
  priceLists,
  setPriceLists,
  availablePriceLists: availablePriceListsOriginal,
}: AssociatedPriceListsEditorProps) {
  const availablePriceLists = useMemo(
    () => [...priceLists, ...availablePriceListsOriginal],
    [priceLists, availablePriceListsOriginal],
  );
  return (
    <MultipleCombobox<PriceListSummary>
      allItems={availablePriceLists}
      selectedItems={priceLists}
      setSelectedItems={setPriceLists}
      toFilterItem={(priceList) => ({
        id: priceList.id,
        title: priceList.name,
      })}
      numSelectedVisible={10}
    />
  );
}
