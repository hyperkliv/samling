import { t, Trans } from "@lingui/macro";
import { Dispatch, DispatchWithoutAction, SetStateAction } from "react";
import api from "../../api";
import { useAppDispatch, useAppSelector } from "../../state/hooks";
import { match } from "oxide.ts";
import { apiErrorMessage } from "../../state/slices/user";
import Modal from "./ModalBase";
import { User } from "../../types/api";

interface Props {
  user: User | null;
  setUser: Dispatch<SetStateAction<User | null>>;
  onDelete: DispatchWithoutAction;
}

export default function DeleteUserModal({ user, setUser, onDelete }: Props) {
  const dispatch = useAppDispatch();
  const { token, activeOrganization } = useAppSelector((state) => state.user);

  function onSubmit() {
    api
      .deleteUser(
        token as string,
        activeOrganization?.organization.id as number,
        user?.id as number,
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
        setUser(null);
      });
  }

  return user ? (
    <Modal
      title={`Delete user`}
      description={
        <Trans>
          Are you sure you want to delete the user{" "}
          <span className="font-medium text-gray-900">{user.name}</span>? This
          action cannot be undone.
        </Trans>
      }
      submitText={t`Delete`}
      cancelButton
      open={!!user}
      close={() => setUser(null)}
      onSubmit={onSubmit}
      controls={[]}
      danger={true}
    />
  ) : (
    <></>
  );
}
