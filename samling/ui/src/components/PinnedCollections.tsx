import { Fragment } from "react";
import { Menu, Transition } from "@headlessui/react";
import { EllipsisVerticalIcon } from "@heroicons/react/24/solid";
import { classNames } from "../utils";
import { Plural, Trans } from "@lingui/macro";
import LocaleLink from "./LocaleLink";
import { CollectionSummary } from "../types/api";
import { useLocalize } from "../i18n";
import { cloudflareImageUrl } from "../images";

type Props = {
  pinnedCollections: CollectionSummary[];
};

export default function PinnedCollections({ pinnedCollections }: Props) {
  return (
    <div className="px-4 mt-6 sm:px-6 lg:px-8">
      <h2 className="text-gray-500 text-xs font-medium uppercase tracking-wide">
        <Trans>Pinned Collections</Trans>
      </h2>
      <ul className="grid grid-cols-1 gap-4 sm:gap-6 sm:grid-cols-2 xl:grid-cols-4 mt-3">
        {pinnedCollections.map((collection) => (
          <PinnedCollection collection={collection} />
        ))}
      </ul>
    </div>
  );
}

interface PinnedCollectionProps {
  collection: CollectionSummary;
}

function PinnedCollection({ collection }: PinnedCollectionProps) {
  const { i18nDbText } = useLocalize();
  const imageUrl = collection.image_url
    ? cloudflareImageUrl(collection.image_url, "thumbnail")
    : "";

  return (
    <li
      key={collection.id}
      className="relative col-span-1 flex shadow-sm rounded-md"
    >
      <div
        className="bg-gray-100 flex-shrink-0 flex items-center justify-center w-20 text-gray-800 text-sm font-medium rounded-l-md"
        style={{ backgroundImage: `url(${imageUrl})`, backgroundSize: "cover" }}
      >
        <LocaleLink to={`/app/collections/${collection.slug}`}>
          {i18nDbText(collection.acronym)}
        </LocaleLink>
      </div>
      <div className="flex-1 flex items-center justify-between border-t border-r border-b border-gray-200 bg-white rounded-r-md truncate">
        <div className="flex-1 px-4 py-2 text-sm truncate">
          <LocaleLink
            to={`/app/collections/${collection.slug}`}
            className="text-gray-900 font-medium hover:text-gray-600"
          >
            {i18nDbText(collection.name)}
          </LocaleLink>
          <p className="text-gray-500">
            <Plural
              value={collection.num_styles}
              one="# style"
              other="# styles"
            />
          </p>
        </div>
        <Menu as="div" className="flex-shrink-0 pr-2">
          <Menu.Button className="w-8 h-8 bg-white inline-flex items-center justify-center text-gray-400 rounded-full hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            <span className="sr-only">
              <Trans>Open options</Trans>
            </span>
            <EllipsisVerticalIcon className="w-5 h-5" aria-hidden="true" />
          </Menu.Button>
          <Transition
            as={Fragment}
            enter="transition ease-out duration-100"
            enterFrom="transform opacity-0 scale-95"
            enterTo="transform opacity-100 scale-100"
            leave="transition ease-in duration-75"
            leaveFrom="transform opacity-100 scale-100"
            leaveTo="transform opacity-0 scale-95"
          >
            <Menu.Items className="z-10 mx-3 origin-top-right absolute right-10 top-3 w-48 mt-1 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 divide-y divide-gray-200 focus:outline-none">
              <div className="py-1">
                <Menu.Item>
                  {({ active }) => (
                    <LocaleLink
                      to={`/app/collections/${collection.slug}`}
                      className={classNames(
                        active ? "bg-gray-100 text-gray-900" : "text-gray-700",
                        "block px-4 py-2 text-sm",
                      )}
                    >
                      <Trans>View</Trans>
                    </LocaleLink>
                  )}
                </Menu.Item>
              </div>
              <div className="py-1">
                <Menu.Item>
                  {({ active }) => (
                    <LocaleLink
                      to={`/app/collections/${collection.slug}/unpin`}
                      className={classNames(
                        active ? "bg-gray-100 text-gray-900" : "text-gray-700",
                        "block px-4 py-2 text-sm",
                      )}
                    >
                      <Trans>Remove from pinned</Trans>
                    </LocaleLink>
                  )}
                </Menu.Item>
                <Menu.Item>
                  {({ active }) => (
                    <LocaleLink
                      to={`/app/collections/${collection.slug}/share`}
                      className={classNames(
                        active ? "bg-gray-100 text-gray-900" : "text-gray-700",
                        "block px-4 py-2 text-sm",
                      )}
                    >
                      <Trans>Share</Trans>
                    </LocaleLink>
                  )}
                </Menu.Item>
              </div>
            </Menu.Items>
          </Transition>
        </Menu>
      </div>
    </li>
  );
}
