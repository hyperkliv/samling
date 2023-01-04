import { Popover, Transition } from "@headlessui/react";
import { CheckBadgeIcon, SparklesIcon } from "@heroicons/react/24/outline";
import { t, Trans } from "@lingui/macro";
import { Fragment } from "react";
import { useLocalize } from "../../../i18n";
import { cloudflareImageUrl, getStyleDisplayImage } from "../../../images";
import { TdColumnProps } from "../../../types/columns";
import { classNames } from "../../../utils";
import LocaleLink from "../../LocaleLink";
import GenericTdCell from "./GenericTdCell";

export default function Style({ columnIndex, item }: TdColumnProps) {
  const { style } = item;
  const { i18nDbText } = useLocalize();
  const displayImage = getStyleDisplayImage(style);
  const alt = t`Image for ${i18nDbText(style.name)}`;

  return (
    <GenericTdCell columnIndex={columnIndex}>
      <div className="grid grid-cols-[20%_34%_14%_22%_6%] space-x-2">
        <div className="max-w-xs flex-shrink-0">
          {displayImage ? (
            <LocaleLink to={style.slug}>
              <img
                className="max-h-20"
                loading="lazy"
                src={cloudflareImageUrl(displayImage.url, "thumbnail")}
                alt={alt}
              />
            </LocaleLink>
          ) : (
            ""
          )}
        </div>
        <div className="flex items-center middle">
          <LocaleLink to={style.slug}>
            <div className="font-medium text-gray-900">
              {i18nDbText(style.name)}
            </div>
            <div className="text-gray-500">{style.number}</div>
          </LocaleLink>
        </div>
        <div className="flex items-center middle">
          {style.core ? (
            <div className="text-gray-500 h-6 w-6">
              <div>
                <CoreIconWithPopover />
              </div>
            </div>
          ) : (
            ""
          )}
          {style.is_new ? (
            <div className="text-gray-500 h-6 w-6">
              <div>
                <StyleIsNewIconWithPopover />
              </div>
            </div>
          ) : (
            ""
          )}
        </div>
        <div className="inline-flex middle flex-col justify-evenly">
          {item.style.categories.map((category) => (
            <div
              className="truncate"
              key={category.id}
              title={i18nDbText(category.name)}
            >
              {i18nDbText(category.name)}
            </div>
          ))}
        </div>
        <div className="inline-flex items-center middle">
          <div className="text-gray-500 h-6 w-6">
            <span className="inline-flex items-center middle rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-800">
              {item.style.country_of_origin}
            </span>
          </div>
        </div>
      </div>
    </GenericTdCell>
  );
}

function CoreIconWithPopover() {
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
            <CheckBadgeIcon className="h-6 w-6" title={t`Core style`} />
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
                  <Trans>Core</Trans>
                </p>
                <p className="mt-1 text-sm text-gray-500">
                  <Trans>
                    This is a Core style, meaning that it represents the brand's
                    core values.
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

function StyleIsNewIconWithPopover() {
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
            <SparklesIcon className="h-6 w-6" title={t`New style`} />
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
                  <Trans>This style first appears in this collection.</Trans>
                </p>
              </div>
            </Popover.Panel>
          </Transition>
        </>
      )}
    </Popover>
  );
}
