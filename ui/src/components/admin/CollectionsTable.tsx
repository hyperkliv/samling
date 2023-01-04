import { Trans } from "@lingui/macro";
import { useState } from "react";
import LocaleLink from "../LocaleLink";
import { CollectionSummary } from "../../types/api";
import { useLocalize } from "../../i18n";
import DeleteCollectionModal from "./DeleteCollectionModal";
import { cloudflareImageUrl } from "../../images";
import { formatRelative, parseISO } from "date-fns";

interface Props {
  collections: CollectionSummary[];
  refreshCollections: () => void;
}

export default function CollectionsTable({
  collections,
  refreshCollections,
}: Props) {
  const { i18nDbText } = useLocalize();
  const [collectionToDelete, setCollectionToDelete] = useState(
    null as null | CollectionSummary,
  );

  return (
    <>
      <DeleteCollectionModal
        collection={collectionToDelete}
        setCollection={setCollectionToDelete}
        onDelete={refreshCollections}
      />
      <>
        <div className="sm:flex sm:items-center">
          <div className="sm:flex-auto my-2">
            <h1 className="text-2xl font-semibold text-gray-900">
              <Trans>Collections</Trans>
            </h1>
            <p className="mt-2 text-sm text-gray-700 typo max-w-lg">
              <Trans>
                These are all the collections that exist in the system.
              </Trans>
            </p>
          </div>
          <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
            <LocaleLink
              to={`/app/admin/collections/new`}
              type="button"
              className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
            >
              <Trans>Create collection</Trans>
            </LocaleLink>
          </div>
        </div>
        <div className="mt-8 flex flex-col">
          <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
            <div className="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
              <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
                <table className="min-w-full divide-y divide-gray-300">
                  <thead className="bg-gray-50">
                    <tr>
                      <th
                        scope="col"
                        className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6"
                      >
                        <Trans>Collection</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans># styles</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans># colors</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans># sizes</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Created at</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Updated at</Trans>
                      </th>
                      <th
                        scope="col"
                        className="relative py-3.5 pl-3 pr-4 sm:pr-6"
                      >
                        <span className="sr-only">
                          <Trans>Edit</Trans>
                        </span>
                      </th>
                      <th
                        scope="col"
                        className="relative py-3.5 pl-3 pr-4 sm:pr-6"
                      >
                        <span className="sr-only">
                          <Trans>Delete</Trans>
                        </span>
                      </th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200 bg-white">
                    {collections.map((collection) => (
                      <tr key={collection.id}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
                          <LocaleLink
                            to={`/app/admin/collections/${collection.id}`}
                            className="flex items-center space-x-4 text-indigo-600 hover:text-indigo-900"
                          >
                            {collection.image_url && (
                              <div className="w-12 h-8 inline-flex justify-center">
                                <img
                                  src={cloudflareImageUrl(
                                    collection.image_url,
                                    "thumbnail",
                                  )}
                                  loading="lazy"
                                  alt=""
                                  className="pointer-events-none group-hover:opacity-75 object-contain max-w-full max-h-full"
                                />
                              </div>
                            )}
                            <div className="font-medium text-gray-900">
                              {i18nDbText(collection.name)}
                            </div>
                          </LocaleLink>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            {collection.num_styles.toLocaleString()}
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            {collection.num_colors.toLocaleString()}
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            {collection.num_sizes.toLocaleString()}
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            {formatRelative(
                              parseISO(collection.created_at),
                              new Date(),
                            )}
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            {formatRelative(
                              parseISO(collection.updated_at),
                              new Date(),
                            )}
                          </div>
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <LocaleLink
                            to={`/app/admin/collections/${collection.id}`}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            <Trans>
                              Edit
                              <span className="sr-only">
                                , {i18nDbText(collection.name)}
                              </span>
                            </Trans>
                          </LocaleLink>
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <button
                            onClick={() => {
                              setCollectionToDelete(collection);
                            }}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            <Trans>
                              Delete
                              <span className="sr-only">
                                , {i18nDbText(collection.name)}
                              </span>
                            </Trans>
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </>
    </>
  );
}
