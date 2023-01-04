import { useLocalize } from "../../../i18n";
import { t } from "@lingui/macro";
import { useCollectionList } from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import ApiError from "../errors/ApiError";
import Loading from "../../../components/Loading";
import { CollectionSummary } from "../../../types/api";
import { useTitle } from "../../../hooks";
import CollectionsTable from "../../../components/admin/CollectionsTable";

export default function AdminCollectionsPage() {
  useTitle([t`Admin`, t`Collections`]);

  const [collectionsResult, refreshCollections] = useCollectionList();

  if (collectionsResult.isErr()) {
    return <ApiError error={collectionsResult.unwrapErr()} />;
  }

  const collections = collectionsResult.unwrap();

  if (collections === null) {
    return <Loading />;
  } else {
    return (
      <AdminCollectionsInner
        collections={collections}
        refreshCollections={refreshCollections}
      />
    );
  }
}

interface AdminCollectionsInnerProps {
  collections: CollectionSummary[];
  refreshCollections: () => void;
}

export function AdminCollectionsInner({
  collections,
  refreshCollections,
}: AdminCollectionsInnerProps) {
  const { i18nLink } = useLocalize();
  const breadcrumbItems = [
    { title: t`Admin`, to: i18nLink("/app/admin"), current: false },
    {
      title: t`Collections`,
      to: i18nLink("/app/admin/collections"),
      current: true,
    },
  ];

  return (
    <>
      <Breadcrumbs items={breadcrumbItems} />
      <div className="px-4 sm:px-6 lg:px-8">
        <CollectionsTable
          collections={collections}
          refreshCollections={refreshCollections}
        />
      </div>
    </>
  );
}
