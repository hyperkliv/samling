import { t } from "@lingui/macro";
import { ElementType } from "react";
import Style from "../components/columns/data/Style";
import Colors from "../components/columns/data/Colors";
import RetailPrices from "../components/columns/data/RetailPrices";
import UnitPrices from "../components/columns/data/UnitPrices";
import GenericThCell from "../components/columns/header/GenericThCell";
import { CollectionItem } from "./api";

export interface ThColumnProps {
  columnIndex: number;
  column: Column;
}

export interface TdColumnProps {
  columnIndex: number;
  column: Column;
  item: CollectionItem;
}

export interface Column {
  id: string;
  title: string;
  enabled: boolean;
  thComponent: ElementType;
  tdComponent: ElementType;
}

const COLUMN_DEFAULTS = { enabled: true, thComponent: GenericThCell };

export const ALL_COLLECTION_COLUMNS: Column[] = [
  {
    id: "style",
    title: t`Style`,
    tdComponent: Style,
    ...COLUMN_DEFAULTS,
  },
  {
    id: `colors`,
    title: t`Colors`,
    tdComponent: Colors,
    ...COLUMN_DEFAULTS,
  },
  {
    id: "retailPrices",
    title: t`Retail price`,
    tdComponent: RetailPrices,
    ...COLUMN_DEFAULTS,
  },
  {
    id: "unitPrices",
    title: t`Unit prices`,
    tdComponent: UnitPrices,
    ...COLUMN_DEFAULTS,
  },
];
