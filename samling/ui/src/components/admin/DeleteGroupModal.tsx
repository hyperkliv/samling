import { t, Trans } from "@lingui/macro";
import { Dispatch, DispatchWithoutAction, SetStateAction } from "react";
import api from "../../api";
import { useAppDispatch, useAppSelector } from "../../state/hooks";
import { match } from "oxide.ts";
import { apiErrorMessage } from "../../state/slices/user";
import Modal from "./ModalBase";
import { GroupSummary } from "../../types/api";

interface Props {
  group: GroupSummary | null;
  setGroup: Dispatch<SetStateAction<GroupSummary | null>>;
  onDelete: DispatchWithoutAction;
}

export default function DeleteGroupModal({ group, setGroup, onDelete }: Props) {
  const dispatch = useAppDispatch();
  const { token, activeOrganization } = useAppSelector((state) => state.user);

  function onSubmit() {
    api
      .deleteGroup(
        token as string,
        activeOrganization?.organization.id as number,
        group?.id as number,
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
        setGroup(null);
      });
  }

  return group ? (
    <Modal
      title={`Delete group`}
      description={
        <Trans>
          Are you sure you want to delete the group{" "}
          <span className="font-medium text-gray-900">{group.name}</span>? This
          action cannot be undone.
        </Trans>
      }
      submitText={t`Delete`}
      cancelButton
      open={!!group}
      close={() => setGroup(null)}
      onSubmit={onSubmit}
      controls={[]}
      danger={true}
    />
  ) : (
    <></>
  );
}
