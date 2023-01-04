import { useLocalize } from "../../../i18n";
import { t } from "@lingui/macro";
import { useGroupSummaries } from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import ApiError from "../errors/ApiError";
import Loading from "../../../components/Loading";
import { GroupSummary } from "../../../types/api";
import { useTitle } from "../../../hooks";
import GroupsTable from "../../../components/admin/GroupsTable";

export default function AdminGroupsPage() {
  useTitle([t`Admin`, t`Groups`]);

  const [groupsResult, refreshGroups] = useGroupSummaries();

  if (groupsResult.isErr()) {
    return <ApiError error={groupsResult.unwrapErr()} />;
  }

  const groups = groupsResult.unwrap();

  if (groups === null) {
    return <Loading />;
  } else {
    return <AdminGroupsInner groups={groups} refreshGroups={refreshGroups} />;
  }
}

interface AdminGroupsInnerProps {
  groups: GroupSummary[];
  refreshGroups: () => void;
}

export function AdminGroupsInner({
  groups,
  refreshGroups,
}: AdminGroupsInnerProps) {
  const { i18nLink } = useLocalize();
  const breadcrumbItems = [
    { title: t`Admin`, to: i18nLink("/app/admin"), current: false },
    { title: t`Groups`, to: i18nLink("/app/admin/groups"), current: true },
  ];

  return (
    <>
      <Breadcrumbs items={breadcrumbItems} />
      <div className="px-4 sm:px-6 lg:px-8">
        <GroupsTable groups={groups} refreshGroups={refreshGroups} />
      </div>
    </>
  );
}
