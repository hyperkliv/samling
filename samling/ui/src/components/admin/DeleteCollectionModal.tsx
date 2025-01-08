import { t, Trans } from "@lingui/macro";
import { Dispatch, DispatchWithoutAction, SetStateAction } from "react";
import api from "../../api";
import { useAppDispatch, useAppSelector } from "../../state/hooks";
import { match } from "oxide.ts";
import { apiErrorMessage } from "../../state/slices/user";
import Modal from "./ModalBase";
import { CollectionSummary } from "../../types/api";
import { useLocalize } from "../../i18n";

interface Props {
  collection: CollectionSummary | null;
  setCollection: Dispatch<SetStateAction<CollectionSummary | null>>;
  onDelete: DispatchWithoutAction;
}

export default function DeleteCollectionModal({
  collection,
  setCollection,
  onDelete,
}: Props) {
  const { i18nDbText } = useLocalize();
  const dispatch = useAppDispatch();
  const { token, activeOrganization } = useAppSelector((state) => state.user);

  function onSubmit() {
    api
      .deleteCollection(
        token as string,
        activeOrganization?.organization.id as number,
        collection?.id as number,
      )
      .then((res) => {
        match(res, {
          Ok: () => {
            onDelete();
          },
          Err: (error) => {
            dispatch(apiErrorMessage(error));
          },
        });
        setCollection(null);
      });
  }

  return collection ? (
    <Modal
      title={`Delete collection`}
      description={
        <Trans>
          Are you sure you want to delete the collection{" "}
          <span className="font-medium text-gray-900">
            {i18nDbText(collection.name)}
          </span>
          ? This action cannot be undone.
        </Trans>
      }
      submitText={t`Delete`}
      cancelButton
      open={!!collection}
      close={() => setCollection(null)}
      onSubmit={onSubmit}
      controls={[]}
      danger={true}
    />
  ) : (
    <></>
  );
}
