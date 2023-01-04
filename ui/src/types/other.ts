import { t } from "@lingui/macro";
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

export const ALL_SORT_ORDER_ALTERNATIVES: StyleSortOrderAlternative[] = [
  { title: t`Number`, apiReference: NestedStyleSortOrder.NumberAsc },
  { title: t`Name`, apiReference: NestedStyleSortOrder.NameAsc },
  {
    title: t`Delivery period`,
    apiReference: NestedStyleSortOrder.DeliveryPeriodAsc,
  },
  {
    title: t`Delivery period (descending)`,
    apiReference: NestedStyleSortOrder.DeliveryPeriodDesc,
  },
];
