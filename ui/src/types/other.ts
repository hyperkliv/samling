import { NestedStyleSortOrder } from "./api";

export interface BreadcrumbItem {
  title: string;
  to: string;
  current: boolean;
}

export interface StyleSortOrderAlternative {
  title: string;
  apiReference: NestedStyleSortOrder;
}
