import { Dispatch, SetStateAction, useMemo } from "react";
import {
  CategorySummary,
  CollectionItem,
  CollectionWithItems,
  NestedStyleSortOrder,
  PriceListSummary,
} from "../types/api";
import StyleFilter from "./filters/StyleFilter";
import CategoryFilter from "./filters/CategoryFilter";
import { ItemFilters } from "../types/filters";
import ActiveFilters from "./filters/ActiveFilters";
import _ from "lodash";
import CountryFilter from "./filters/CountryFilter";
import CollectionTableSortMenu from "./CollectionTableSortMenu";
import NewFilter from "./filters/NewFilter";
import CollectionTablePriceListsMenu from "./CollectionTablePriceListsMenu";
import ActivePriceLists from "./ActivePriceLists";
import CoreFilter from "./filters/CoreFilter";
import ServiceItemFilter from "./filters/ServiceItemFilter";
import ExportForm from "./ExportForm";

interface Props {
  collection: CollectionWithItems;
  items: CollectionItem[];
  allItems: CollectionItem[];
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
  sortOrder: NestedStyleSortOrder;
  setSortOrder: Dispatch<SetStateAction<NestedStyleSortOrder>>;
  allPriceLists: PriceListSummary[];
  priceLists: PriceListSummary[];
  setPriceLists: Dispatch<SetStateAction<PriceListSummary[]>>;
}

export default function CollectionFilters({
  collection,
  items,
  allItems,
  itemFilters,
  setItemFilters,
  sortOrder,
  setSortOrder,
  allPriceLists,
  priceLists,
  setPriceLists,
}: Props) {
  const allCategories = useMemo(() => {
    return _.sortBy(
      _.uniqBy(
        allItems.reduce((categories, item) => {
          item.style.categories.forEach((cat) => {
            categories.push(cat);
          });
          return categories;
        }, [] as CategorySummary[]),
        "slug",
      ),
      ["slug"],
    );
  }, [allItems]);

  const allCountryCodes = useMemo(() => {
    return _.sortedUniq(
      allItems
        .reduce((codes, item) => {
          if (!!item.style.country_of_origin) {
            codes.push(item.style.country_of_origin);
          }
          return codes;
        }, [] as string[])
        .sort(),
    );
  }, [allItems]);

  return (
    <div className="bg-white mx-4 sm:mx-6 lg:mx-8">
      <section className="print:hidden">
        <div className="mt-6 sm:flex flex-wrap gap-x-4">
          <div className="sm:w-70 shrink-0 grow">
            <StyleFilter
              allItems={allItems}
              itemFilters={itemFilters}
              setItemFilters={setItemFilters}
            />
          </div>
          <div className=" sm:w-48 shrink-0 grow">
            <CategoryFilter
              allCategories={allCategories}
              itemFilters={itemFilters}
              setItemFilters={setItemFilters}
            />
          </div>
          <div className="sm:w-30 shrink-0 grow-0">
            <CountryFilter
              allCountryCodes={allCountryCodes}
              itemFilters={itemFilters}
              setItemFilters={setItemFilters}
            />
          </div>
          <div className="sm:w-40 shrink-0 grow-0">
            <NewFilter
              itemFilters={itemFilters}
              setItemFilters={setItemFilters}
            />
          </div>
          <div className="sm:w-30 shrink-0 grow-0">
            <CoreFilter
              itemFilters={itemFilters}
              setItemFilters={setItemFilters}
            />
          </div>
          <div className="sm:w-38 shrink-0 grow-0">
            <ServiceItemFilter
              itemFilters={itemFilters}
              setItemFilters={setItemFilters}
            />
          </div>
          {/* TODO: Separate Sort By and Price Lists from filters! */}
          <div className="sm:w-52 shrink-0 grow-0">
            <CollectionTableSortMenu
              sortOrder={sortOrder}
              setSortOrder={setSortOrder}
            />
          </div>
          <div className="sm:w-44 shrink-0 grow-0">
            <CollectionTablePriceListsMenu
              allPriceLists={allPriceLists}
              priceLists={priceLists}
              setPriceLists={setPriceLists}
            />
          </div>
          <div className="sm:w-20">
            <ExportForm
              collection={collection}
              priceLists={priceLists}
              itemFilters={itemFilters}
            />
          </div>
        </div>
      </section>

      <section>
        <ActiveFilters
          itemFilters={itemFilters}
          setItemFilters={setItemFilters}
          items={items}
          allItems={allItems}
          allCategories={allCategories}
        />
      </section>

      <section>
        <ActivePriceLists
          priceLists={priceLists}
          setPriceLists={setPriceLists}
        />
      </section>
    </div>
  );
}
