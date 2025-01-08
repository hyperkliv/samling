import { ElementType } from "react";
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
