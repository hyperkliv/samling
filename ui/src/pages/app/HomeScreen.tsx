import CollectionsGridList from "../../components/CollectionsGridList";
// import PinnedCollections from "../../components/PinnedCollections";
import { useCollectionList } from "../../api";
import ApiError from "./errors/ApiError";
import { match } from "oxide.ts";
import Loading from "../../components/Loading";
import { useTitle } from "../../hooks";
import { t } from "@lingui/macro";

export default function HomeScreen() {
  const [result] = useCollectionList();
  useTitle([t`Collections`]);

  return match(result, {
    Ok: (collectionList) =>
      collectionList === null ? (
        <Loading />
      ) : (
        <>
          {/* <PinnedCollections
          pinnedCollections={collectionList.filter((_, idx) => idx < 4)}
        /> */}
          <CollectionsGridList collectionList={collectionList} />
        </>
      ),
    Err: (error) => <ApiError error={error} />,
  });
}
