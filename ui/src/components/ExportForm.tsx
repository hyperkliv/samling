import { Fragment, useState } from "react";
import { Disclosure, Popover, Transition } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import { t, Trans } from "@lingui/macro";
import { useAppSelector } from "../state/hooks";
import api from "../api";
import {
  CollectionWithItems,
  ExportField,
  GroupBy,
  ExportFormat,
  PriceListSummary,
} from "../types/api";
import { useLocalize } from "../i18n";
import { ItemFilters, makeCollectionFilters } from "../types/filters";
import { classNames } from "../utils";
import LoadingIndicator from "./LoadingIndicator";

interface Props {
  collection: CollectionWithItems;
  priceLists: PriceListSummary[];
  itemFilters: ItemFilters;
}

interface ExportFormatEntry {
  format: ExportFormat;
  label: string;
  description?: string;
  checked: boolean;
}

interface GroupByEntry {
  groupBy: GroupBy;
  label: string;
  disabled: boolean;
  checked: boolean;
}

interface ExportFieldEntry {
  field: ExportField;
  label: string;
  description?: string;
  checked: boolean;
}

const allExportFormatEntries: ExportFormatEntry[] = [
  {
    format: ExportFormat.Xlsx,
    label: t`Excel`,
    description: t`Uses the newer .xlsx format.`,
    checked: true,
  },
  {
    format: ExportFormat.Csv,
    label: t`CSV`,
    description: t`Often useful for ERP system imports.`,
    checked: false,
  },
  {
    format: ExportFormat.Json,
    label: t`JSON`,
    description: t`Useful for integration with other systems.`,
    checked: false,
  },
];

const allGroupByEntries: GroupByEntry[] = [
  { groupBy: GroupBy.Style, label: t`Style`, disabled: true, checked: true },
  { groupBy: GroupBy.Color, label: t`Color`, disabled: false, checked: false },
  { groupBy: GroupBy.Size, label: t`Size`, disabled: false, checked: false },
  {
    groupBy: GroupBy.Category,
    label: t`Category`,
    disabled: false,
    checked: false,
  },
  {
    groupBy: GroupBy.PriceList,
    label: t`Price list`,
    disabled: false,
    checked: false,
  },
  {
    groupBy: GroupBy.Image,
    label: t`Image`,
    disabled: false,
    checked: false,
  },
];

const allFieldEntries: ExportFieldEntry[] = [
  { field: ExportField.StyleNumber, label: t`Style number`, checked: true },
  { field: ExportField.StyleName, label: t`Style name`, checked: true },
  {
    field: ExportField.StyleDescription,
    label: t`Style description`,
    checked: true,
  },
  { field: ExportField.NewStyle, label: t`New style`, checked: true },
  { field: ExportField.Core, label: t`Core`, checked: true },
  {
    field: ExportField.CountryOfOrigin,
    label: t`Country of origin`,
    checked: true,
  },
  { field: ExportField.ColorNumber, label: t`Color number`, checked: true },
  { field: ExportField.ColorName, label: t`Color name`, checked: true },
  { field: ExportField.NewColor, label: t`New color`, checked: true },
  { field: ExportField.SizeType, label: t`Size type`, checked: true },
  { field: ExportField.SizeNumber, label: t`Size number`, checked: true },
  { field: ExportField.EanCode, label: t`EAN code`, checked: true },
  { field: ExportField.ServiceItem, label: t`Service item`, checked: true },
  {
    field: ExportField.RetailPriceAmount,
    label: t`Retail price amount`,
    checked: true,
  },
  {
    field: ExportField.RetailPriceCurrency,
    label: t`Retail price currency`,
    checked: true,
  },
  {
    field: ExportField.RetailPriceList,
    label: t`Retail price list`,
    checked: true,
  },
  {
    field: ExportField.UnitPriceAmount,
    label: t`Unit price amount`,
    checked: true,
  },
  {
    field: ExportField.UnitPriceCurrency,
    label: t`Unit price currency`,
    checked: true,
  },
  {
    field: ExportField.UnitPriceList,
    label: t`Unit price list`,
    checked: true,
  },
  { field: ExportField.CategoryName, label: t`Category name`, checked: true },
  {
    field: ExportField.Attribute,
    label: t`Attributes`,
    description: t`One column per attribute type will be generated.`,
    checked: true,
  },
  {
    field: ExportField.DeliveryPeriod,
    label: t`Delivery period`,
    checked: true,
  },
  { field: ExportField.TariffNo, label: t`Tariff no`, checked: true },
  { field: ExportField.GrossWeight, label: t`Gross weight`, checked: true },
  { field: ExportField.UnitVolume, label: t`Unit volume`, checked: true },
  { field: ExportField.PrimaryImage, label: t`Primary image`, checked: true },
  { field: ExportField.Images, label: t`Images`, checked: false },
  {
    field: ExportField.StyleExternalid,
    label: t`Style external id`,
    checked: false,
  },
  {
    field: ExportField.ColorExternalid,
    label: t`Color external id`,
    checked: false,
  },
];

export default function ExportForm({
  collection,
  priceLists,
  itemFilters,
}: Props) {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const { i18nDbText } = useLocalize();

  const [exportFormat, setExportFormat] = useState(ExportFormat.Xlsx);
  const [groupByEntries, setGroupByEntries] = useState(() => [
    ...allGroupByEntries,
  ]);
  const [fieldEntries, setFieldEntries] = useState(() => [...allFieldEntries]);
  const [downloading, setDownloading] = useState(false);

  function downloadFile() {
    if (!!token && !!activeOrganization) {
      setDownloading(true);
      api
        .downloadExportFile(
          token,
          activeOrganization.organization.id,
          collection.slug,
          exportFormat,
          groupByEntries.filter((g) => g.checked).map((g) => g.groupBy),
          fieldEntries.filter((f) => f.checked).map((f) => f.field),
          makeCollectionFilters(
            itemFilters,
            priceLists.map((pl) => pl.slug),
          ),
        )
        .then(async (res) => {
          const blob = await res.blob();
          // TODO: Why is content-disposition this empty?
          const contentDisposition = res.headers.get("Content-Disposition");
          console.info(contentDisposition);
          const href = window.URL.createObjectURL(blob);
          Object.assign(document.createElement("a"), {
            href,
            download: `${i18nDbText(
              collection.name,
            )}.${exportFormat.toLowerCase()}`,
          }).click();
        })
        .finally(() => {
          setDownloading(false);
        });
    }
  }

  return (
    <Popover as="div" className="relative inline-block text-left">
      <div className="block text-sm font-medium text-gray-700 whitespace-pre">
        {" "}
      </div>
      <div>
        <Popover.Button className="inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-100">
          <Trans>Export</Trans>
          <ChevronDownIcon className="-mr-1 ml-2 h-5 w-5" aria-hidden="true" />
        </Popover.Button>
      </div>

      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <Popover.Panel className="absolute md:right-0 z-10 m-4 w-96 md:origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
          <div className="p-6">
            <div>
              <h3 className="text-2xl font-medium leading-6 text-gray-900">
                <Trans>Export</Trans>
              </h3>
              <p className="mt-4 text-sm text-gray-500">
                <Trans>
                  Use this form to export the current selection of items to
                  file.
                </Trans>
              </p>
            </div>
            <div className="mt-6">
              <form
                onSubmit={(evt) => {
                  downloadFile();
                  evt.preventDefault();
                }}
              >
                <fieldset className="mt-6">
                  <legend className="contents text-base font-medium text-gray-900">
                    <Trans>Format</Trans>
                  </legend>
                  <p className="text-sm text-gray-500">
                    <Trans>Which format you want to export to.</Trans>
                  </p>
                  <div className="mt-2 space-y-4">
                    {allExportFormatEntries.map((entry) => (
                      <div
                        key={entry.format}
                        className="relative flex items-start"
                      >
                        <div className="flex h-5 items-center">
                          <input
                            id={`export-format-${entry.format}`}
                            name="format"
                            value={entry.format}
                            type="radio"
                            checked={entry.format === exportFormat}
                            onChange={(evt) =>
                              evt.target.checked
                                ? setExportFormat(entry.format)
                                : ""
                            }
                            className="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500 disabled:text-gray-400"
                          />
                        </div>
                        <div className="ml-3 text-sm">
                          <label
                            htmlFor={`export-format-${entry.format}`}
                            className="font-medium text-gray-700"
                          >
                            {entry.label}
                          </label>
                          {entry.description ? (
                            <p className="text-gray-500 text-xs">
                              {entry.description}
                            </p>
                          ) : (
                            ""
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </fieldset>
                <fieldset className="mt-6">
                  <legend className="contents text-base font-medium text-gray-900">
                    <Trans>Group by</Trans>
                  </legend>
                  <p className="text-sm text-gray-500">
                    <Trans>
                      Tick the data types that you want to appear on their own
                      separate line in the exported file.
                    </Trans>
                  </p>
                  <div className="mt-2 space-y-4">
                    {groupByEntries.map((entry) => (
                      <div
                        key={entry.groupBy}
                        className="relative flex items-start"
                      >
                        <div className="flex h-5 items-center">
                          <input
                            id={`export-group-by-${entry.groupBy}`}
                            name="group-by"
                            value={entry.groupBy}
                            type="checkbox"
                            checked={entry.checked}
                            onChange={(evt) =>
                              setGroupByEntries(
                                groupByEntries.map((g) =>
                                  g.groupBy === entry.groupBy
                                    ? { ...g, checked: evt.target.checked }
                                    : g,
                                ),
                              )
                            }
                            disabled={entry.disabled}
                            className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500 disabled:text-gray-400"
                          />
                        </div>
                        <div className="ml-3 text-sm">
                          <label
                            htmlFor={`export-group-by-${entry.groupBy}`}
                            className="font-medium text-gray-700"
                          >
                            {entry.label}
                          </label>
                        </div>
                      </div>
                    ))}
                  </div>
                </fieldset>
                <fieldset className="mt-6">
                  <legend className="contents text-base font-medium text-gray-900">
                    <Trans>Fields</Trans>
                  </legend>
                  <p className="text-sm text-gray-500">
                    <Trans>
                      Which fields you want to include in the exported file.
                    </Trans>
                  </p>
                  <Disclosure>
                    {({ open }) => (
                      <>
                        <Disclosure.Button className="flex w-full items-start justify-between text-left text-gray-700 my-2">
                          <span className="text-sm font-normal">
                            <Trans>
                              {fieldEntries.filter((fe) => fe.checked).length}{" "}
                              of {fieldEntries.length} fields selected
                            </Trans>
                          </span>
                          <span className="ml-6 flex h-7 items-center">
                            <ChevronDownIcon
                              className={classNames(
                                open ? "-rotate-180" : "rotate-0",
                                "h-6 w-6 transform",
                              )}
                              aria-hidden="true"
                            />
                          </span>
                        </Disclosure.Button>
                        <Disclosure.Panel>
                          <div className="mt-2 space-y-4">
                            {fieldEntries.map((entry) => (
                              <div
                                key={entry.field}
                                className="relative flex items-start"
                              >
                                <div className="flex h-5 items-center">
                                  <input
                                    id={`export-fields-${entry.field}`}
                                    name="fields"
                                    value={entry.field}
                                    type="checkbox"
                                    onChange={(evt) =>
                                      setFieldEntries(
                                        fieldEntries.map((field) =>
                                          field.field === entry.field
                                            ? {
                                                ...field,
                                                checked: evt.target.checked,
                                              }
                                            : field,
                                        ),
                                      )
                                    }
                                    checked={entry.checked}
                                    className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                                  />
                                </div>
                                <div className="ml-3 text-sm">
                                  <label
                                    htmlFor={`export-fields-${entry.field}`}
                                    className="font-medium text-gray-700"
                                  >
                                    {entry.label}
                                  </label>
                                  {entry.description ? (
                                    <p className="text-gray-500 text-xs">
                                      {entry.description}
                                    </p>
                                  ) : (
                                    ""
                                  )}
                                </div>
                              </div>
                            ))}
                          </div>
                        </Disclosure.Panel>
                      </>
                    )}
                  </Disclosure>
                </fieldset>
                <div className="flex my-4">
                  <button
                    type="submit"
                    disabled={downloading}
                    className="h-10 rounded-md border border-transparent py-3 w-40 bg-indigo-600 disabled:bg-gray-300 flex items-center justify-center text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:ring-transparent"
                  >
                    {downloading ? (
                      <span className="mr-2">
                        <LoadingIndicator
                          textColor="gray"
                          fillColor="gray"
                          size={6}
                        />
                      </span>
                    ) : (
                      " "
                    )}
                    {downloading ? (
                      <Trans>Downloading...</Trans>
                    ) : (
                      <Trans>Download</Trans>
                    )}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </Popover.Panel>
      </Transition>
    </Popover>
  );
}
