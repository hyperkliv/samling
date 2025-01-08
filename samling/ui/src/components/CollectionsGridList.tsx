import { ExclamationTriangleIcon } from "@heroicons/react/24/outline";
import { Plural, Trans } from "@lingui/macro";
import { useLocalize } from "../i18n";
import { cloudflareImageUrl } from "../images";
import { CollectionSummary } from "../types/api";
import LocaleLink from "./LocaleLink";

type Props = {
  collectionList: CollectionSummary[];
};

export default function CollectionsGridList({ collectionList }: Props) {
  const { i18nDbText } = useLocalize();
  return (
    <div className="px-4 mt-6 sm:px-6 lg:px-8">
      {collectionList.length === 0 ? (
        <NoCollections />
      ) : (
        <ul className="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8">
          {collectionList.map((collection) => (
            <li key={collection.id} className="relative">
              <LocaleLink to={`/app/collections/${collection.slug}`}>
                <div className="group aspect-w-10 aspect-h-7 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
                  {collection.image_url ? (
                    <img
                      src={cloudflareImageUrl(collection.image_url, "medium")}
                      loading="lazy"
                      alt=""
                      className="pointer-events-none object-cover group-hover:opacity-75"
                    />
                  ) : (
                    ""
                  )}
                  <button
                    type="button"
                    className="absolute inset-0 focus:outline-none"
                  >
                    <span className="sr-only">
                      <Trans>
                        View details for {i18nDbText(collection.name)}
                      </Trans>
                    </span>
                  </button>
                </div>
                <p className="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">
                  {i18nDbText(collection.name)}
                </p>
                <p className="pointer-events-none block text-sm font-medium text-gray-500">
                  <Plural
                    value={collection.num_styles}
                    one="# style"
                    other="# styles"
                  />
                </p>
              </LocaleLink>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function NoCollections() {
  return (
    <div className="grid place-content-center align-middle h-screen">
      <div className="block max-w-md rounded-lg border-2 border-dashed border-gray-300 p-12 text-center">
        <ExclamationTriangleIcon className="w-6 h-6 mx-auto text-gray-400" />
        <h3 className="mt-2 text-sm font-medium text-gray-900">
          <Trans>No collections</Trans>
        </h3>
        <p className="mt-1 text-sm text-gray-500">
          <Trans>
            There are no collections available to you. Talk an administrator to
            get access.
          </Trans>
        </p>
      </div>
    </div>
  );
}
