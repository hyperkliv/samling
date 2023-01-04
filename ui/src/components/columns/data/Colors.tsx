import { Popover, Transition } from "@headlessui/react";
import { HandThumbUpIcon, SparklesIcon } from "@heroicons/react/24/outline";
import { t, Trans } from "@lingui/macro";
import { match } from "oxide.ts";
import { getWeek, getYear, parseISO } from "date-fns";
import { Fragment } from "react";
import { useLocalize } from "../../../i18n";
import { cloudflareImageUrl } from "../../../images";
import { CollectionItem, NestedColor, NestedSize } from "../../../types/api";
import { TdColumnProps } from "../../../types/columns";
import { classNames } from "../../../utils";
import GenericTdCell from "./GenericTdCell";
import LocaleLink from "../../LocaleLink";
import { uniqBy } from "lodash";

export default function Colors({ item, columnIndex }: TdColumnProps) {
  return (
    <GenericTdCell columnIndex={columnIndex}>
      <div className="grid grid-cols-[20%_30%_16%_18%_12%]">
        {item.style.colors.map((color) => (
          <Color key={color.id} item={item} color={color} />
        ))}
      </div>
    </GenericTdCell>
  );
}

interface ColorProps {
  item: CollectionItem;
  color: NestedColor;
}

function Color({ item, color }: ColorProps) {
  const { i18nDbText } = useLocalize();
  const displayImage = color.images[0];
  const alt = t`Image for ${i18nDbText(item.style.name)} / ${i18nDbText(
    color.name,
  )}`;
  return (
    <>
      <div className="mx-4">
        <div className="h-12 w-12">
          {displayImage ? (
            <LocaleLink to={`${item.style.slug}/${color.slug}`}>
              <img
                className="h-12 max-w-12"
                loading="lazy"
                src={cloudflareImageUrl(displayImage.url, "thumbnail")}
                alt={alt}
              />
            </LocaleLink>
          ) : (
            ""
          )}
        </div>
      </div>
      <div className="mx-4">
        <LocaleLink to={`${item.style.slug}/${color.slug}`}>
          <div className="font-medium text-gray-900">
            {i18nDbText(color.name)}
          </div>
          <div className="text-gray-500">{color.number}</div>
        </LocaleLink>
      </div>
      <div className="inline-flex items-center middle mx-4 whitespace-nowrap">
        <Sizes key={color.id} sizes={color.sizes} />
      </div>
      <div className="inline-flex items-center middle mx-4">
        <DeliveryPeriod key={color.id} sizes={color.sizes} />
      </div>
      <div className="inline-flex items-center middle mx-4">
        {color.sizes.every((size) => size.service_item === true) ? (
          <div className="mx-1">
            <ServiceItemIconWithPopover />
          </div>
        ) : (
          ""
        )}
        {!item.style.is_new && color.is_new ? (
          <div className="mx-1">
            <ColorIsNewIconWithPopover />
          </div>
        ) : (
          ""
        )}
      </div>
    </>
  );
}

function ServiceItemIconWithPopover() {
  return (
    <Popover>
      {({ open }) => (
        <>
          <Popover.Button
            className={classNames(
              open ? "text-gray-900" : "text-gray-500",
              "rounded-xl text-base font-medium hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
            )}
          >
            <HandThumbUpIcon className="h-6 w-6" title={t`Service item`} />
          </Popover.Button>

          <Transition
            as={Fragment}
            enter="transition ease-out duration-200"
            enterFrom="opacity-0 translate-y-1"
            enterTo="opacity-100 translate-y-0"
            leave="transition ease-in duration-150"
            leaveFrom="opacity-100 translate-y-0"
            leaveTo="opacity-0 translate-y-1"
          >
            <Popover.Panel className="absolute z-10 bg-white">
              <div className="rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 p-4">
                <p className="text-base font-medium text-gray-900">
                  <Trans>Service item</Trans>
                </p>
                <p className="mt-1 text-sm text-gray-500">
                  <Trans>
                    This is a service item, meaning that it's our intention to
                    never run out of stock with this style.
                  </Trans>
                </p>
              </div>
            </Popover.Panel>
          </Transition>
        </>
      )}
    </Popover>
  );
}

function ColorIsNewIconWithPopover() {
  return (
    <Popover>
      {({ open }) => (
        <>
          <Popover.Button
            className={classNames(
              open ? "text-gray-900" : "text-gray-500",
              "rounded-xl text-base font-medium hover:text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
            )}
          >
            <SparklesIcon className="h-6 w-6" title={t`New color`} />
          </Popover.Button>

          <Transition
            as={Fragment}
            enter="transition ease-out duration-200"
            enterFrom="opacity-0 translate-y-1"
            enterTo="opacity-100 translate-y-0"
            leave="transition ease-in duration-150"
            leaveFrom="opacity-100 translate-y-0"
            leaveTo="opacity-0 translate-y-1"
          >
            <Popover.Panel className="absolute z-10 bg-white">
              <div className="rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 p-4">
                <p className="text-base font-medium text-gray-900">
                  <Trans>New</Trans>
                </p>
                <p className="mt-1 text-sm text-gray-500">
                  <Trans>This color first appears in this collection.</Trans>
                </p>
              </div>
            </Popover.Panel>
          </Transition>
        </>
      )}
    </Popover>
  );
}

interface SizeProps {
  sizes: NestedSize[];
}

export function Sizes({ sizes }: SizeProps) {
  let text = getSizeRange(sizes);
  if (
    text.indexOf("½") === -1 &&
    !!sizes.find((size) => size.number.indexOf("½") !== -1)
  ) {
    text += " (½)";
  }
  return (
    <span className="inline-flex items-center middle rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-800">
      {text}
    </span>
  );
}

function getSizeRange(sizes: NestedSize[]) {
  const sortedSizes = sizes.sort(function (a, b) {
    return a.number.localeCompare(b.number, undefined, {
      numeric: true,
      sensitivity: "base",
    });
  });
  if (sortedSizes.length > 1) {
    const first = sortedSizes[0];
    const last = sortedSizes[sortedSizes.length - 1];
    return `${first.number}-${last.number}`;
  } else {
    return sortedSizes[0].number;
  }
}

interface DeliveryPeriodProps {
  sizes: NestedSize[];
}

export function DeliveryPeriod({ sizes }: DeliveryPeriodProps) {
  const deliveryPeriodStrings = uniqBy(
    sizes.map((size) => size.delivery_period),
    "delivery_period",
  ).filter((date) => !!date) as string[];
  const deliveryPeriods = deliveryPeriodStrings.map((dp) => parseISO(dp));
  const text = match(deliveryPeriods.length, [
    [0, ""],
    [1, `${yearWeek(deliveryPeriods[0])}`],
    () =>
      t`${yearWeek(deliveryPeriods[0])} (+${deliveryPeriods.length - 1} more)`,
  ]);
  return (
    <span className="inline-flex items-center middle rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-800">
      {text}
    </span>
  );
}

function yearWeek(date: Date): string {
  const year = getYear(date);
  const week = getWeek(date);
  return `${year}W${week}`;
}
