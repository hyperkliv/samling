import { Plural, Trans } from "@lingui/macro";
import CreateGroupModal from "./CreateGroupModal";
import { useState } from "react";
import LocaleLink from "../LocaleLink";
import { GroupSummary } from "../../types/api";
import DeleteGroupModal from "./DeleteGroupModal";

interface Props {
  groups: GroupSummary[];
  refreshGroups: () => void;
}

export default function GroupsTable({ groups, refreshGroups }: Props) {
  const [openCreateGroupModal, setCreateGroupModalOpen] = useState(false);
  const [groupToDelete, setGroupToDelete] = useState(
    null as null | GroupSummary,
  );

  return (
    <>
      <CreateGroupModal
        open={openCreateGroupModal}
        setOpen={setCreateGroupModalOpen}
        onCreate={refreshGroups}
      />
      <DeleteGroupModal
        group={groupToDelete}
        setGroup={setGroupToDelete}
        onDelete={refreshGroups}
      />
      <>
        <div className="sm:flex sm:items-center">
          <div className="sm:flex-auto my-2">
            <h1 className="text-2xl font-semibold text-gray-900">
              <Trans>Groups</Trans>
            </h1>
            <p className="mt-2 text-sm text-gray-700 typo max-w-lg">
              <Trans>
                Groups are used to give access to collections and price lists to
                a specific set of users. The price lists and collections that a
                user has been granted to via one or more groups are added
                together. Groups cannot take away access to price lists or
                collections.
              </Trans>
            </p>
          </div>
          <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
            <button
              onClick={() => {
                setCreateGroupModalOpen(true);
              }}
              type="button"
              className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
            >
              <Trans>Add group</Trans>
            </button>
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
                        <Trans>Name</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Users</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Collections</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Price lists</Trans>
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
                    {groups.map((group) => (
                      <tr key={group.id}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
                          <div className="flex items-center">
                            <div>
                              <div className="font-medium text-gray-900">
                                {group.name}
                              </div>
                              <div className="text-gray-500">
                                {group.description}
                              </div>
                            </div>
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            <Plural
                              value={group.num_users}
                              one="# user"
                              other="# users"
                            />
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            <Plural
                              value={group.num_collections}
                              one="# collection"
                              other="# collections"
                            />
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="text-gray-900">
                            <Plural
                              value={group.num_price_lists}
                              one="# price list"
                              other="# price lists"
                            />
                          </div>
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <LocaleLink
                            to={`/app/admin/groups/${group.slug}`}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            <Trans>Edit</Trans>
                            <span className="sr-only">, {group.name}</span>
                          </LocaleLink>
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <button
                            onClick={() => {
                              setGroupToDelete(group);
                            }}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            <Trans>Delete</Trans>
                            <span className="sr-only">, {group.name}</span>
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
