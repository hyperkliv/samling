import { useLocalize } from "../../../i18n";
import { i18n } from "@lingui/core";
import { Plural, t, Trans } from "@lingui/macro";
import api, { useGroupSummaries, useUser } from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import { match } from "oxide.ts";
import ApiError from "../errors/ApiError";
import Loading from "../../../components/Loading";
import { useParams } from "react-router-dom";
import { useAppDispatch, useAppSelector } from "../../../state/hooks";
import {
  GroupSummary,
  Role,
  UpdateUser,
  User,
  UserOrganization,
} from "../../../types/api";
import {
  Dispatch,
  DispatchWithoutAction,
  SetStateAction,
  useState,
} from "react";
import LocaleLink from "../../../components/LocaleLink";
import { apiErrorMessage, userMessage } from "../../../state/slices/user";
import { UserMessageLevel } from "../../../types/messages";
import SearchableFilter, {
  SearchableFilterItem,
} from "../../../components/filters/SearchableFilter";
import Input from "../../../components/forms/Input";
import { allRoles, getRoleDetail } from "../../../roles";
import { useTitle } from "../../../hooks";

export default function AdminEditGroup() {
  const { userId } = useParams();
  const { i18nLink } = useLocalize();
  const [userResult, refreshUser] = useUser(
    userId ? parseInt(userId, 10) : undefined,
  );
  const user = userResult.unwrapOr(null);
  useTitle([t`Admin`, user?.name]);

  const { token, activeOrganization } = useAppSelector((state) => state.user);

  return match(userResult, {
    Err: (error) => <ApiError error={error} />,
    Ok: () =>
      user === null ? (
        <Loading />
      ) : (
        <>
          <Breadcrumbs
            items={[
              { title: t`Admin`, to: i18nLink("/app/admin"), current: false },
              {
                title: t`Users`,
                to: i18nLink(`/app/admin/users`),
                current: false,
              },
              {
                title: t`Edit ${user.name}`,
                to: i18nLink(`/app/admin/users/${userId}`),
                current: false,
              },
            ]}
          />
          <UserEditForm
            user={user}
            onSuccess={refreshUser}
            apiToken={token as string}
            organizationId={activeOrganization?.organization.id as number}
          />
        </>
      ),
  });
}

interface UserEditFormProps {
  user: User;
  apiToken: string;
  organizationId: number;
  onSuccess: () => void;
}

function UserEditForm({
  apiToken,
  organizationId,
  user,
  onSuccess,
}: UserEditFormProps) {
  const dispatch = useAppDispatch();
  const activeUserOrganization = user.organizations.find(
    (org) => org.organization.id === organizationId,
  ) as UserOrganization;

  const [name, setName] = useState(user.name);
  const [email, setEmail] = useState(user.email);
  const [password, setPassword] = useState("");

  const [groups, setGroups] = useState(user.groups);
  const [roles, setRoles] = useState(activeUserOrganization.roles);

  const [allGroups] = useGroupSummaries();

  function submitForm() {
    let updateUser = {
      name,
      email,
      password: password || null,
      groups: groups.map((group) => ({ id: group.id })),
      roles,
    } as UpdateUser;
    api
      .updateUser(apiToken, organizationId, user.id, updateUser)
      .then((res) => {
        match(res, {
          Ok: (user) => {
            dispatch(
              userMessage({
                level: UserMessageLevel.Info,
                title: t`Success!`,
                body: t`The user "${user.name}" was successfully updated.`,
                opts: { dismissAfter: 3000 },
              }),
            );
            onSuccess();
          },
          Err: (error) => {
            dispatch(apiErrorMessage(error));
          },
        });
      });
  }

  return (
    <form
      onSubmit={(e) => {
        submitForm();
        e.preventDefault();
      }}
      className="space-y-8 mx-12 my-8"
    >
      <div className="space-y-8 divide-y divide-gray-200">
        <div>
          <div>
            <h2 className="text-3xl font-medium leading-6 text-gray-900">
              {user.name}
            </h2>
            <p className="mt-1 text-sm text-gray-500">
              <Trans>Make your changes, then hit save when done.</Trans>
            </p>
          </div>

          <div className="mt-10">
            <div>
              <h3 className="text-lg font-medium leading-6 text-gray-900">
                <Trans>Basic settings</Trans>
              </h3>
            </div>
          </div>
          <div className="mt-2 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div className="sm:col-span-4">
              <Input
                id="name"
                placeholder={t`Jane Doe`}
                title={t`Name`}
                value={name || ""}
                setValue={setName}
                required={true}
              />
            </div>
            <div className="sm:col-span-4">
              <Input
                id="email"
                placeholder={t`jane.doe@example.com`}
                title={t`E-mail address`}
                value={email || ""}
                setValue={setEmail}
                required={true}
              />
            </div>
            <div className="sm:col-span-4">
              <Input
                id="password"
                placeholder={t`Enter new password here to change...`}
                title={t`Password`}
                value={password || ""}
                setValue={setPassword}
                required={false}
              />
            </div>
          </div>

          <div>
            <RolesEditTable roles={roles} setRoles={setRoles} />
          </div>

          <div>
            <GroupsEditTable
              groups={groups}
              setGroups={setGroups}
              allGroups={allGroups.unwrapOr([]) || []}
            />
          </div>
        </div>
      </div>
      <div className="py-5 sticky bottom-0 bg-white">
        <div className="flex justify-start">
          <LocaleLink
            to="/app/admin"
            type="button"
            className="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <Trans>Cancel</Trans>
          </LocaleLink>
          <button
            type="submit"
            className="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <Trans>Save</Trans>
          </button>
        </div>
      </div>
    </form>
  );
}

interface RoleEditTableProps {
  roles: Role[];
  setRoles: Dispatch<SetStateAction<Role[]>>;
}

function RolesEditTable({ roles, setRoles }: RoleEditTableProps) {
  const searchableItems = allRoles
    .filter((roleDetail) => !roles.includes(roleDetail.id))
    .map(
      (roleDetail) =>
        ({
          id: roleDetail.id,
          title: i18n._(roleDetail.title),
          subtitle: i18n._(roleDetail.description),
        } as SearchableFilterItem),
    );

  function onNewItem(newItem: SearchableFilterItem) {
    setRoles([...roles, newItem.id as Role]);
  }

  function remove(toRemove: Role) {
    setRoles(roles.filter((role) => role !== toRemove));
  }

  return (
    <div className="mt-10">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900">
          <Trans>Roles</Trans>
        </h3>
        <p className="mt-1 text-sm text-gray-500">
          <Trans>The roles that are assigned to this user.</Trans>
        </p>
      </div>
      {roles.length === 0 ? (
        <div className="text-sm text-gray-500 my-2">
          <Trans>List is empty.</Trans>
        </div>
      ) : (
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
                <Trans>Description</Trans>
              </th>
              <th scope="col" className="relative py-3.5 pl-3 pr-4 sm:pr-6">
                <span className="sr-only">Edit</span>
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white">
            {roles.map((role) => (
              <RoleTableRow
                key={role}
                role={role}
                removeRole={() => remove(role)}
              />
            ))}
          </tbody>
        </table>
      )}
      <SearchableFilter
        title={t`Add group`}
        items={searchableItems}
        onNewItem={onNewItem}
      />
    </div>
  );
}

interface RoleTableRowProps {
  role: Role;
  removeRole: DispatchWithoutAction;
}

function RoleTableRow({ role, removeRole }: RoleTableRowProps) {
  const roleDetail = getRoleDetail(role);
  return (
    <tr>
      <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
        <div className="flex items-center">
          <div className="ml-4">
            <div className="font-medium text-gray-900">
              {i18n._(roleDetail.title)}
            </div>
          </div>
        </div>
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        {i18n._(roleDetail.description)}
      </td>
      <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
        <button
          onClick={removeRole}
          className="text-indigo-600 hover:text-indigo-900"
        >
          <Trans>
            Remove<span className="sr-only">, {i18n._(roleDetail.title)}</span>
          </Trans>
        </button>
      </td>
    </tr>
  );
}

interface GroupsEditTableProps {
  groups: GroupSummary[];
  setGroups: Dispatch<SetStateAction<GroupSummary[]>>;
  allGroups: GroupSummary[];
}

function GroupsEditTable({
  groups,
  setGroups,
  allGroups,
}: GroupsEditTableProps) {
  const groupIds = groups.map((g) => g.id);
  const searchableItems = allGroups
    .filter((group) => !groupIds.includes(group.id))
    .map(
      (group) =>
        ({
          id: group.id,
          title: group.name,
          subtitle: group.description,
        } as SearchableFilterItem),
    );

  function onNewItem(newItem: SearchableFilterItem) {
    const newGroup = allGroups.find(
      (group) => group.id === newItem.id,
    ) as GroupSummary;
    setGroups([...groups, newGroup]);
  }

  function remove(toRemove: GroupSummary) {
    setGroups(groups.filter((group) => group.id !== toRemove.id));
  }

  return (
    <div className="mt-10">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900">
          <Trans>Groups</Trans>
        </h3>
        <p className="mt-1 text-sm text-gray-500">
          <Trans>The groups that this user belong to.</Trans>
        </p>
      </div>
      {groups.length === 0 ? (
        <div className="text-sm text-gray-500 my-2">
          <Trans>List is empty.</Trans>
        </div>
      ) : (
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
                <Trans># collections</Trans>
              </th>
              <th
                scope="col"
                className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
              >
                <Trans># price lists</Trans>
              </th>
              <th scope="col" className="relative py-3.5 pl-3 pr-4 sm:pr-6">
                <span className="sr-only">Edit</span>
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white">
            {groups.map((group) => (
              <GroupTableRow
                key={group.id}
                group={group}
                removeGroup={() => remove(group)}
              />
            ))}
          </tbody>
        </table>
      )}
      <SearchableFilter
        title={t`Add group`}
        items={searchableItems}
        onNewItem={onNewItem}
      />
    </div>
  );
}

interface GroupTableRowProps {
  group: GroupSummary;
  removeGroup: DispatchWithoutAction;
}

function GroupTableRow({ group, removeGroup }: GroupTableRowProps) {
  return (
    <tr>
      <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
        <div className="flex items-center">
          <div className="ml-4">
            <div className="font-medium text-gray-900">{group.name}</div>
            <div className="text-gray-500">{group.description}</div>
          </div>
        </div>
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        <Plural
          value={group.num_collections}
          one="# collection"
          other="# collections"
        />
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        <Plural
          value={group.num_price_lists}
          one="# price list"
          other="# price lists"
        />
      </td>
      <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
        <button
          onClick={removeGroup}
          className="text-indigo-600 hover:text-indigo-900"
        >
          <Trans>
            Remove<span className="sr-only">, {group.name}</span>
          </Trans>
        </button>
      </td>
    </tr>
  );
}
