import { i18n } from "@lingui/core";
import { t, Trans } from "@lingui/macro";
import CreateUserModal from "./CreateUserModal";
import DeleteUserModal from "./DeleteUserModal";
import { useState, Fragment, Dispatch, SetStateAction, useMemo } from "react";
import LocaleLink from "../LocaleLink";
import { GroupSummary, Role, User, UserSortOrder } from "../../types/api";
import { useAppSelector } from "../../state/hooks";
import { formatRelative, parseISO } from "date-fns";
import { classNames } from "../../utils";
import {
  Dialog,
  Disclosure,
  Menu,
  Popover,
  Transition,
} from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { ChevronDownIcon, CheckBadgeIcon } from "@heroicons/react/20/solid";
import {
  Active,
  Administrator,
  Editor,
  getRoleDetail,
  Viewer,
} from "../../roles";
import { cloudflareImageUrl } from "../../images";

interface Props {
  allUsers: User[];
  users: User[];
  refreshUsers: () => void;
  sortBy: UserSortOrder;
  setSortBy: Dispatch<SetStateAction<UserSortOrder>>;
  roles: Role[];
  setRoles: Dispatch<SetStateAction<Role[]>>;
  groups: GroupSummary[];
  setGroups: Dispatch<SetStateAction<GroupSummary[]>>;
  allGroups: GroupSummary[];
}

interface OrgUser {
  user: User;
  roles: Role[];
}

interface RoleStat {
  name: string;
  stat: string | number;
  statSuffix?: string;
  role?: Role;
}

export default function UsersTable({
  users,
  allUsers,
  refreshUsers,
  sortBy,
  setSortBy,
  roles,
  setRoles,
  groups,
  setGroups,
  allGroups,
}: Props) {
  const [openCreateUserModal, setCreateUserModalOpen] = useState(false);
  const [userToDelete, setUserToDelete] = useState(null as null | User);
  const { activeOrganization } = useAppSelector((state) => state.user);
  const orgUsers = useMemo(() => {
    return users.map((user) => {
      const roles = user.organizations.find(
        (org) => org.organization.id === activeOrganization?.organization.id,
      )?.roles;
      return {
        roles,
        user,
      } as OrgUser;
    });
  }, [users, activeOrganization]);
  let orgUser = orgUsers.find((orgUser) => orgUser.user.id === 138);
  console.log(orgUser);
  const allOrgUsers = useMemo(() => {
    return allUsers.map((user) => {
      const roles = user.organizations.find(
        (org) => org.organization.id === activeOrganization?.organization.id,
      )?.roles;
      return {
        roles,
        user,
      } as OrgUser;
    });
  }, [allUsers, activeOrganization]);
  const stats: RoleStat[] = [
    {
      name: t`Inactive users`,
      stat: allOrgUsers.filter(
        (orgUser) => !orgUser.roles.includes(Role.Active),
      ).length,
    },
    {
      name: t`Active users`,
      stat: allOrgUsers.filter((orgUser) => orgUser.roles.includes(Role.Active))
        .length,
      role: Role.Active,
    },
    {
      name: i18n._(Viewer.titlePlural),
      stat: allOrgUsers.filter((orgUser) => orgUser.roles.includes(Role.Viewer))
        .length,
      role: Role.Viewer,
    },
    {
      name: i18n._(Editor.titlePlural),
      stat: allOrgUsers.filter((orgUser) => orgUser.roles.includes(Role.Editor))
        .length,
      role: Role.Editor,
    },
    {
      name: i18n._(Administrator.titlePlural),
      stat: allOrgUsers.filter((orgUser) =>
        orgUser.roles.includes(Role.Administrator),
      ).length,
      role: Role.Administrator,
    },
  ];

  return (
    <>
      <CreateUserModal
        open={openCreateUserModal}
        setOpen={setCreateUserModalOpen}
        onCreate={refreshUsers}
      />
      <DeleteUserModal
        user={userToDelete}
        setUser={setUserToDelete}
        onDelete={refreshUsers}
      />
      <>
        <div className="sm:flex sm:items-center">
          <div className="sm:flex-auto my-2">
            <h1 className="text-2xl font-semibold text-gray-900">
              <Trans>Users</Trans>
            </h1>
            <p className="mt-2 text-sm text-gray-700 typo max-w-lg">
              <Trans>These users have access to the system.</Trans>
            </p>
            <dl className="mt-5 grid grid-cols-1 md:grid-cols-2 gap-5 lg:grid-cols-3 xl:grid-cols-5">
              {stats.map((item) => (
                <div
                  key={item.name}
                  className="overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:p-6"
                >
                  <dt className="truncate text-sm font-medium text-gray-500">
                    {item.name}
                  </dt>
                  <dd className="mt-1 text-3xl font-semibold tracking-tight text-gray-900">
                    {!!item.role ? (
                      <button onClick={() => setRoles([item.role as Role])}>
                        {item.stat}
                      </button>
                    ) : (
                      <>{item.stat}</>
                    )}
                    {item.statSuffix ? (
                      <span className="pl-2 font-normal text-sm">
                        {item.statSuffix}
                      </span>
                    ) : (
                      ""
                    )}
                  </dd>
                </div>
              ))}
            </dl>
          </div>
          <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
            <button
              onClick={() => {
                setCreateUserModalOpen(true);
              }}
              type="button"
              className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
            >
              <Trans>Add user</Trans>
            </button>
          </div>
        </div>
        <UserTableFilters
          sortBy={sortBy}
          setSortBy={setSortBy}
          roles={roles}
          setRoles={setRoles}
          groups={groups}
          setGroups={setGroups}
          allGroups={allGroups}
        />
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
                        <Trans>User</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Last sign in</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Roles</Trans>
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        <Trans>Groups</Trans>
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
                    {orgUsers.map(({ user, roles }) => (
                      <tr key={user.id}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
                          <div className="flex items-center">
                            <div className="h-8 w-8 mr-4">
                            {user.profile_image && (
                              <img
                                className="h-8 w-auto"
                                src={cloudflareImageUrl(user.profile_image, "thumbnail")}
                                alt={t`${user.name} profile`}
                              />
                            )}
                            </div>
                            <div className="overflow-hidden overflow-ellipsis">
                              <div className="font-medium text-gray-900">
                                {user.name}
                              </div>
                              <div className="text-gray-500">{user.email}</div>
                            </div>
                          </div>
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          <div className="flex items-center space-x-2">
                            {user.last_sign_in
                              ? formatRelative(
                                  parseISO(user.last_sign_in),
                                  new Date(),
                                )
                              : ""}
                          </div>
                        </td>
                        <td className="whitespace-nowrap text-sm text-gray-500">
                          <div className="flex items-center">
                            {roles.map((role) => (
                              <span
                                key={role}
                                className="inline-flex items-center rounded-full border-2 border-gray-100 bg-gray-100 m-2 px-2.5 py-0.5 text-xs font-medium text-gray-800"
                              >
                                {i18n._(getRoleDetail(role).title)}
                              </span>
                            ))}
                          </div>
                        </td>
                        <td className="whitespace-nowrap text-sm text-gray-500">
                          <div className="flex flex-wrap items-center">
                            {user.groups.map((group) => (
                              <span
                                key={group.id}
                                className="inline-flex items-center rounded-full border-2 border-gray-100 m-2 px-2.5 py-0.5 text-xs font-medium text-gray-800"
                              >
                                {group.name}
                              </span>
                            ))}
                          </div>
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <LocaleLink
                            to={`/app/admin/users/${user.id}`}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            {t`Edit`}
                            <span className="sr-only">, {user.name}</span>
                          </LocaleLink>
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <button
                            onClick={() => {
                              setUserToDelete(user);
                            }}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            <Trans>Delete</Trans>
                            <span className="sr-only">, {user.name}</span>
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

interface UserSortOption {
  name: string;
  value: UserSortOrder;
}

interface UserTableFilterOption {
  value: string;
  label: string;
}

interface UserTableFilter {
  id: string;
  name: string;
  onChange: (value: string, checked: boolean) => void;
  activeOptions: string[];
  options: UserTableFilterOption[];
}

interface UserTableFilterProps {
  sortBy: UserSortOrder;
  setSortBy: Dispatch<SetStateAction<UserSortOrder>>;
  roles: Role[];
  setRoles: Dispatch<SetStateAction<Role[]>>;
  groups: GroupSummary[];
  allGroups: GroupSummary[];
  setGroups: Dispatch<SetStateAction<GroupSummary[]>>;
}

function UserTableFilters({
  sortBy,
  setSortBy,
  roles,
  setRoles,
  groups,
  setGroups,
  allGroups,
}: UserTableFilterProps) {
  const [open, setOpen] = useState(false);

  const sortOptions: UserSortOption[] = [
    { name: t`Name`, value: UserSortOrder.NameAsc },
    { name: t`E-mail address`, value: UserSortOrder.EmailAsc },
    { name: t`Newest sign in`, value: UserSortOrder.LastSignInDesc },
    { name: t`Oldest sign in`, value: UserSortOrder.LastSignInAsc },
  ];

  const filters: UserTableFilter[] = [
    {
      id: "roles",
      name: t`Roles`,
      onChange: (value, checked) => {
        if (checked) {
          setRoles(Array.from(new Set([value as Role, ...roles])));
        } else {
          setRoles(roles.filter((role) => role !== (value as Role)));
        }
      },
      activeOptions: roles,
      options: [
        { value: Role.Active, label: i18n._(Active.title) },
        { value: Role.Viewer, label: i18n._(Viewer.title) },
        { value: Role.Editor, label: i18n._(Editor.title) },
        { value: Role.Administrator, label: i18n._(Administrator.title) },
      ],
    },
    {
      id: "groups",
      name: t`Groups`,
      onChange: (value, checked) => {
        const group = allGroups.find((g) => g.id.toString() === value);
        if (!!group) {
          if (checked) {
            setGroups(Array.from(new Set([group, ...groups])));
          } else {
            setGroups(groups.filter((g) => g.id !== group.id));
          }
        }
      },
      activeOptions: groups.map((g) => g.id.toString()),
      options: allGroups.map((group) => ({
        value: group.id.toString(),
        label: group.name,
      })),
    },
  ];

  return (
    <div>
      {/* Mobile filter dialog */}
      <Transition.Root show={open} as={Fragment}>
        <Dialog as="div" className="relative z-40 sm:hidden" onClose={setOpen}>
          <Transition.Child
            as={Fragment}
            enter="transition-opacity ease-linear duration-300"
            enterFrom="opacity-0"
            enterTo="opacity-100"
            leave="transition-opacity ease-linear duration-300"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="fixed inset-0 bg-black bg-opacity-25" />
          </Transition.Child>

          <div className="fixed inset-0 z-40 flex">
            <Transition.Child
              as={Fragment}
              enter="transition ease-in-out duration-300 transform"
              enterFrom="translate-x-full"
              enterTo="translate-x-0"
              leave="transition ease-in-out duration-300 transform"
              leaveFrom="translate-x-0"
              leaveTo="translate-x-full"
            >
              <Dialog.Panel className="relative ml-auto flex h-full w-full max-w-xs flex-col overflow-y-auto bg-white py-4 pb-6 shadow-xl">
                <div className="flex items-center justify-between px-4">
                  <h2 className="text-lg font-medium text-gray-900">Filters</h2>
                  <button
                    type="button"
                    className="-mr-2 flex h-10 w-10 items-center justify-center rounded-md bg-white p-2 text-gray-400 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                    onClick={() => setOpen(false)}
                  >
                    <span className="sr-only">Close menu</span>
                    <XMarkIcon className="h-6 w-6" aria-hidden="true" />
                  </button>
                </div>

                {/* Filters */}
                <form className="mt-4">
                  {filters.map((section) => (
                    <Disclosure
                      as="div"
                      key={section.name}
                      className="border-t border-gray-200 px-4 py-6"
                    >
                      {({ open }) => (
                        <>
                          <h3 className="-mx-2 -my-3 flow-root">
                            <Disclosure.Button className="flex w-full items-center justify-between bg-white px-2 py-3 text-sm text-gray-400">
                              <span className="font-medium text-gray-900">
                                {section.name}
                              </span>
                              <span className="ml-6 flex items-center">
                                <ChevronDownIcon
                                  className={classNames(
                                    open ? "-rotate-180" : "rotate-0",
                                    "h-5 w-5 transform",
                                  )}
                                  aria-hidden="true"
                                />
                              </span>
                            </Disclosure.Button>
                          </h3>
                          <Disclosure.Panel className="pt-6">
                            <div className="space-y-6">
                              {section.options.map((option, optionIdx) => (
                                <div
                                  key={option.value}
                                  className="flex items-center"
                                >
                                  <input
                                    id={`filter-mobile-${section.id}-${optionIdx}`}
                                    name={`${section.id}`}
                                    onChange={(evt) => {
                                      section.onChange(
                                        option.value,
                                        evt.target.checked,
                                      );
                                    }}
                                    type="checkbox"
                                    className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                                  />
                                  <label
                                    htmlFor={`filter-mobile-${section.id}-${optionIdx}`}
                                    className="ml-3 text-sm text-gray-500"
                                  >
                                    {option.label}
                                  </label>
                                </div>
                              ))}
                            </div>
                          </Disclosure.Panel>
                        </>
                      )}
                    </Disclosure>
                  ))}
                </form>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </Dialog>
      </Transition.Root>

      <div>
        <section aria-labelledby="filter-heading" className="my-6">
          <h2 id="filter-heading" className="sr-only">
            Product filters
          </h2>

          <div className="flex items-center justify-between">
            <Menu as="div" className="relative inline-block text-left">
              <div>
                <Menu.Button className="group inline-flex justify-center text-sm font-medium text-gray-700 hover:text-gray-900">
                  Sort
                  <ChevronDownIcon
                    className="-mr-1 ml-1 h-5 w-5 flex-shrink-0 text-gray-400 group-hover:text-gray-500"
                    aria-hidden="true"
                  />
                </Menu.Button>
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
                <Menu.Items className="absolute left-0 z-10 mt-2 w-40 origin-top-left rounded-md bg-white shadow-2xl ring-1 ring-black ring-opacity-5 focus:outline-none">
                  <div className="py-1">
                    {sortOptions.map((option) => (
                      <Menu.Item key={option.value}>
                        {({ active }) => (
                          <button
                            onClick={() => setSortBy(option.value)}
                            className={classNames(
                              active ? "bg-gray-100" : "",
                              "block w-full text-left px-4 py-2 text-sm font-medium text-gray-900",
                            )}
                          >
                            {option.name}
                            {option.value === sortBy ? (
                              <CheckBadgeIcon className="inline w-4 h-4 ml-1" />
                            ) : (
                              ""
                            )}
                          </button>
                        )}
                      </Menu.Item>
                    ))}
                  </div>
                </Menu.Items>
              </Transition>
            </Menu>

            <button
              type="button"
              className="inline-block text-sm font-medium text-gray-700 hover:text-gray-900 sm:hidden"
              onClick={() => setOpen(true)}
            >
              Filters
            </button>

            <Popover.Group className="hidden sm:flex sm:items-baseline sm:space-x-8">
              {filters.map((section, sectionIdx) => (
                <Popover
                  as="div"
                  key={section.name}
                  id={`desktop-menu-${sectionIdx}`}
                  className="relative inline-block text-left"
                >
                  <div>
                    <Popover.Button className="group inline-flex items-center justify-center text-sm font-medium text-gray-700 hover:text-gray-900">
                      <span>{section.name}</span>
                      {section.options.filter((f) =>
                        section.activeOptions.includes(f.value),
                      ).length > 0 ? (
                        <span className="ml-1.5 rounded bg-gray-200 py-0.5 px-1.5 text-xs font-semibold tabular-nums text-gray-700">
                          {
                            section.options.filter((f) =>
                              section.activeOptions.includes(f.value),
                            ).length
                          }
                        </span>
                      ) : null}
                      <ChevronDownIcon
                        className="-mr-1 ml-1 h-5 w-5 flex-shrink-0 text-gray-400 group-hover:text-gray-500"
                        aria-hidden="true"
                      />
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
                    <Popover.Panel className="absolute right-0 z-10 mt-2 origin-top-right rounded-md bg-white p-4 shadow-2xl ring-1 ring-black ring-opacity-5 focus:outline-none">
                      <form className="space-y-4">
                        {section.options.map((option, optionIdx) => (
                          <div key={option.value} className="flex items-center">
                            <input
                              id={`filter-${section.id}-${optionIdx}`}
                              name={`${section.id}`}
                              defaultChecked={section.activeOptions.includes(
                                option.value,
                              )}
                              onChange={(evt) => {
                                section.onChange(
                                  option.value,
                                  evt.target.checked,
                                );
                              }}
                              type="checkbox"
                              className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                            />
                            <label
                              htmlFor={`filter-${section.id}-${optionIdx}`}
                              className="ml-3 whitespace-nowrap pr-6 text-sm font-medium text-gray-900"
                            >
                              {option.label}
                            </label>
                          </div>
                        ))}
                      </form>
                    </Popover.Panel>
                  </Transition>
                </Popover>
              ))}
            </Popover.Group>
          </div>
        </section>
      </div>
    </div>
  );
}
