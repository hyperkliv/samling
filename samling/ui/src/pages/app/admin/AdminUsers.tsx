import { useLocalize } from "../../../i18n";
import { t } from "@lingui/macro";
import { useGroupSummaries, useUserList } from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import ApiError from "../errors/ApiError";
import Loading from "../../../components/Loading";
import { GroupSummary, Role, User, UserSortOrder } from "../../../types/api";
import UsersTable from "../../../components/admin/UsersTable";
import { Dispatch, SetStateAction, useMemo, useState } from "react";
import { useTitle } from "../../../hooks";

export default function AdminUsersPage() {
  useTitle([t`Admin`, t`Users`]);

  const [userSortBy, setUserSortBy] = useState(UserSortOrder.NameAsc);
  const [userRolesFilter, setUserRolesFilter] = useState([] as Role[]);
  const [userGroupsFilter, setUserGroupsFilter] = useState(
    [] as GroupSummary[],
  );

  const [groupsResult] = useGroupSummaries();
  const userFilters = useMemo(
    () => ({
      roles: userRolesFilter,
      groups: userGroupsFilter.length === 0 ? null : userGroupsFilter.map((g) => ({ id: g.id })),
    }),
    [userRolesFilter, userGroupsFilter],
  );
  const [usersResult, refreshUsers] = useUserList(userSortBy, userFilters);
  const [allUsersResult] = useUserList(userSortBy);

  if (groupsResult.isErr()) {
    return <ApiError error={groupsResult.unwrapErr()} />;
  }

  if (usersResult.isErr()) {
    return <ApiError error={usersResult.unwrapErr()} />;
  }

  const groups = groupsResult.unwrap();
  const users = usersResult.unwrap();
  const allUsers = allUsersResult.unwrap();

  if (groups === null || users === null || allUsers === null) {
    return <Loading />;
  } else {
    return (
      <AdminUsersInner
        groups={groups}
        users={users}
        allUsers={allUsers}
        refreshUsers={refreshUsers}
        userSortBy={userSortBy}
        setUserSortBy={setUserSortBy}
        userRolesFilter={userRolesFilter}
        setUserRolesFilter={setUserRolesFilter}
        userGroupsFilter={userGroupsFilter}
        setUserGroupsFilter={setUserGroupsFilter}
      />
    );
  }
}

interface AdminUsersInnerProps {
  groups: GroupSummary[];
  users: User[];
  allUsers: User[];
  refreshUsers: () => void;
  userSortBy: UserSortOrder;
  setUserSortBy: Dispatch<SetStateAction<UserSortOrder>>;
  userRolesFilter: Role[];
  setUserRolesFilter: Dispatch<SetStateAction<Role[]>>;
  userGroupsFilter: GroupSummary[];
  setUserGroupsFilter: Dispatch<SetStateAction<GroupSummary[]>>;
}

export function AdminUsersInner({
  groups,
  users,
  allUsers,
  refreshUsers,
  userSortBy,
  setUserSortBy,
  userRolesFilter,
  setUserRolesFilter,
  userGroupsFilter,
  setUserGroupsFilter,
}: AdminUsersInnerProps) {
  const { i18nLink } = useLocalize();
  const breadcrumbItems = [
    { title: t`Admin`, to: i18nLink("/app/admin"), current: false },
    { title: t`Users`, to: i18nLink("/app/admin/users"), current: true },
  ];

  return (
    <>
      <Breadcrumbs items={breadcrumbItems} />
      <div className="px-4 sm:px-6 lg:px-8">
        <UsersTable
          allUsers={allUsers}
          users={users}
          refreshUsers={refreshUsers}
          sortBy={userSortBy}
          setSortBy={setUserSortBy}
          roles={userRolesFilter}
          setRoles={setUserRolesFilter}
          groups={userGroupsFilter}
          setGroups={setUserGroupsFilter}
          allGroups={groups}
        />
      </div>
    </>
  );
}
