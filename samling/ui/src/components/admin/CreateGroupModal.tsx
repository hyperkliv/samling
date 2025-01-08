import { t, Trans } from "@lingui/macro";
import {
  Dispatch,
  DispatchWithoutAction,
  SetStateAction,
  useState,
} from "react";
import api from "../../api";
import { useAppDispatch, useAppSelector } from "../../state/hooks";
import { match } from "oxide.ts";
import { apiErrorMessage } from "../../state/slices/user";
import Modal, { FormInput } from "./ModalBase";

interface Props {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  onCreate: DispatchWithoutAction;
}

export default function CreateGroupModal({ open, setOpen, onCreate }: Props) {
  const dispatch = useAppDispatch();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [name, setName] = useState("");
  const input: FormInput = {
    id: "name",
    name: "name",
    label: t`Name`,
    placeholder: t`North America sales reps`,
    htmlType: "text",
    required: true,
    value: name,
    onChange: (value) => setName(value),
  };

  function onSubmit() {
    api
      .createGroup(
        token as string,
        activeOrganization?.organization.id as number,
        { name },
      )
      .then((res) => {
        match(res, {
          Ok: () => {
            onCreate();
          },
          Err: (error) => {
            dispatch(apiErrorMessage(error));
          },
        });
        setOpen(false);
      });
  }

  return (
    <Modal
      title={`Create group`}
      description={
        <Trans>
          Groups are used to limit user access to collections and price lists.
        </Trans>
      }
      submitText={t`Create`}
      cancelButton
      open={open}
      close={() => setOpen(false)}
      onSubmit={onSubmit}
      controls={[input]}
    />
  );
}
