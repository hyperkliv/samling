import { t } from "@lingui/macro";
import { Dispatch, SetStateAction } from "react";
import { useLocalize } from "../i18n";
import { cloudflareImageUrl } from "../images";
import {
  CollectionItem,
  CollectionWithItems,
  NestedStyleSortOrder,
  PriceListSummary,
} from "../types/api";
import { ItemFilters } from "../types/filters";
import CollectionFilters from "./CollectionFilters";
import Breadcrumbs from "./nav/Breadcrumbs";
import { Column } from "../types/columns";
import Style from "../components/columns/data/Style";
import Colors from "../components/columns/data/Colors";
import RetailPrices from "../components/columns/data/RetailPrices";
import UnitPrices from "../components/columns/data/UnitPrices";
import GenericThCell from "./columns/header/GenericThCell";

interface Props {
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
  collection: CollectionWithItems;
  allItems: CollectionItem[];
  sortOrder: NestedStyleSortOrder;
  setSortOrder: Dispatch<SetStateAction<NestedStyleSortOrder>>;
  allPriceLists: PriceListSummary[];
  priceLists: PriceListSummary[];
  setPriceLists: Dispatch<SetStateAction<PriceListSummary[]>>;
}

export default function CollectionTable({
  collection,
  allItems,
  itemFilters,
  setItemFilters,
  allPriceLists,
  priceLists,
  setPriceLists,
  sortOrder,
  setSortOrder,
}: Props) {
  const { i18nDbText, i18nLink } = useLocalize();
  // TODO: Implement column setting (maybe rename to fields?)
  const columnDefaults = { enabled: true, thComponent: GenericThCell };
  const allCollectionColumns: Column[] = [
    {
      id: "style",
      title: t`Style`,
      tdComponent: Style,
      ...columnDefaults,
    },
    {
      id: `colors`,
      title: t`Colors`,
      tdComponent: Colors,
      ...columnDefaults,
    },
    {
      id: "retailPrices",
      title: t`Retail price`,
      tdComponent: RetailPrices,
      ...columnDefaults,
    },
    {
      id: "unitPrices",
      title: t`Unit prices`,
      tdComponent: UnitPrices,
      ...columnDefaults,
    },
  ];

  return (
    <>
      <Breadcrumbs
        items={[
          { title: t`Collections`, to: i18nLink("/app"), current: false },
          {
            title: i18nDbText(collection.name),
            to: i18nLink(`/app/collections/${collection.slug}`),
            current: true,
          },
        ]}
      />
      <div>
        <div className="my-8 mx-4 sm:mx-6 lg:mx-8">
          {collection.image_url ? (
            <img
              src={cloudflareImageUrl(collection.image_url, "thumbnail")}
              alt=""
              className="h-20 inline rounded-md"
            />
          ) : (
            ""
          )}
          <h1 className="text-3xl pl-4 inline align-middle font-bold tracking-tight text-gray-900">
            {i18nDbText(collection.name)}
          </h1>
        </div>

        <CollectionFilters
          collection={collection}
          items={collection.items}
          allItems={allItems}
          itemFilters={itemFilters}
          setItemFilters={setItemFilters}
          priceLists={priceLists}
          allPriceLists={allPriceLists}
          setPriceLists={setPriceLists}
          sortOrder={sortOrder}
          setSortOrder={setSortOrder}
        />
        <div className="mt-8 flex flex-col">
          <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
            <div className="block min-w-full print:w-auto py-2 align-middle md:px-6 lg:px-8">
              <div className="shadow ring-1 ring-black ring-opacity-5">
                <table className="min-w-full print:w-auto divide-y divide-gray-300">
                  <thead className="bg-gray-50 print:hidden">
                    <tr key={0}>
                      {allCollectionColumns
                        .filter((c) => c.enabled)
                        .map((column, columnIndex) => (
                          <column.thComponent
                            key={columnIndex}
                            columnIndex={columnIndex}
                            column={column}
                          />
                        ))}
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200 bg-white">
                    {collection.items.map((item) => (
                      <tr key={item.style.id} className="break-inside-avoid">
                        {allCollectionColumns
                          .filter((c) => c.enabled)
                          .map((column, columnIndex) => (
                            <column.tdComponent
                              key={`${item.style.id}-${columnIndex}`}
                              columnIndex={columnIndex}
                              column={column}
                              item={item}
                            />
                          ))}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
