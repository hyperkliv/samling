import { Dispatch, SetStateAction, useEffect, useMemo, useState } from "react";
import { useParams, useSearchParams } from "react-router-dom";
import { useCollectionWithItems } from "../../api";
import _ from "lodash";
import CollectionTable from "../../components/CollectionTable";
import Loading from "../../components/Loading";
import {
  CollectionItem,
  CollectionWithItems,
  NestedStyleSortOrder,
  PriceListSummary,
} from "../../types/api";
import {
  ItemFilters,
  itemFiltersFromSearchParams,
  itemFiltersToSearchParams,
  makeCollectionFilters,
} from "../../types/filters";
import ApiError from "./errors/ApiError";
import { useLocalize } from "../../i18n";
import { useTitle } from "../../hooks";

export default function Collection() {
  const { collectionSlug } = useParams();
  const { i18nDbText } = useLocalize();

  const searchParams = useSearchParams()[0];
  const [itemFilters, setItemFilters] = useState(() =>
    itemFiltersFromSearchParams(searchParams),
  );

  const [sortOrder, setSortOrder] = useState(
    (searchParams.get("sortBy") as NestedStyleSortOrder | null) ||
      NestedStyleSortOrder.NumberAsc,
  );

  const collectionFilters = useMemo(() => {
    let newFilters = makeCollectionFilters(
      itemFilters,
      searchParams.getAll("pricelist"),
    );
    return newFilters;
  }, [itemFilters, searchParams]);
  const collectionRef = useMemo(
    () => ({ slug: collectionSlug }),
    [collectionSlug],
  );
  const [collectionResult] = useCollectionWithItems(
    collectionRef,
    sortOrder,
    collectionFilters,
  );

  const [allItemsCollectionResult] = useCollectionWithItems(
    collectionRef,
    sortOrder,
  );

  const collection = collectionResult.unwrapOr(null);

  const allItemsCollection = allItemsCollectionResult.unwrapOr(null);

  useTitle([i18nDbText(collection?.name)]);

  if (collectionResult.isErr()) {
    return <ApiError error={collectionResult.unwrapErr()} />;
  }

  if (allItemsCollectionResult.isErr()) {
    return <ApiError error={allItemsCollectionResult.unwrapErr()} />;
  }

  if (collection === null || allItemsCollection === null) {
    return <Loading />;
  } else {
    return (
      <CollectionInner
        itemFilters={itemFilters}
        setItemFilters={setItemFilters}
        sortOrder={sortOrder}
        setSortOrder={setSortOrder}
        collection={collection}
        allItems={allItemsCollection.items}
      />
    );
  }
}

interface CollectionInnerProps {
  collection: CollectionWithItems;
  allItems: CollectionItem[];
  itemFilters: ItemFilters;
  setItemFilters: Dispatch<SetStateAction<ItemFilters>>;
  sortOrder: NestedStyleSortOrder;
  setSortOrder: Dispatch<SetStateAction<NestedStyleSortOrder>>;
}

function CollectionInner({
  collection,
  allItems,
  sortOrder,
  setSortOrder,
  itemFilters,
  setItemFilters,
}: CollectionInnerProps) {
  const [searchParams, setSearchParams] = useSearchParams();
  let activePriceLists = searchParams.getAll("pricelist");

  const allPriceLists = useMemo(() => {
    const map: Map<string, PriceListSummary> = new Map();
    allItems.forEach((item) => {
      item.style.prices.forEach((price) => {
        map.set(price.list.slug, price.list);
      });
    });
    return _.sortBy(Array.from(map.values()), ["slug"]);
  }, [allItems]);

  const [priceLists, setPriceLists] = useState(() =>
    allPriceLists.filter((pl) => activePriceLists.includes(pl.slug)),
  );

  useEffect(() => {
    setSearchParams(
      {
        ...itemFiltersToSearchParams(itemFilters),
        pricelist: priceLists.map((pl) => pl.slug),
        sortBy: sortOrder,
      },
      { replace: true },
    );
  }, [itemFilters, priceLists, sortOrder, setSearchParams]);

  return (
    <CollectionTable
      itemFilters={itemFilters}
      allItems={allItems}
      setItemFilters={setItemFilters}
      collection={collection}
      allPriceLists={allPriceLists}
      priceLists={priceLists}
      setPriceLists={setPriceLists}
      sortOrder={sortOrder}
      setSortOrder={setSortOrder}
    />
  );
}
