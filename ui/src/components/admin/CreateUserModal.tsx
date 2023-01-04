import { i18n } from "@lingui/core";
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
import Modal, { FormControlType } from "./ModalBase";
import { Role } from "../../types/api";
import { Active, Administrator, Editor, Viewer } from "../../roles";

interface Props {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  onCreate: DispatchWithoutAction;
}

export default function CreateGroupModal({ open, setOpen, onCreate }: Props) {
  const dispatch = useAppDispatch();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [roles, setRoles] = useState([Role.Active]);
  const controls: FormControlType[] = [
    {
      id: "name",
      name: "name",
      label: t`Name`,
      placeholder: t`Jane Doe`,
      htmlType: "text",
      required: true,
      value: name,
      onChange: (value: string) => setName(value),
    },
    {
      id: "email",
      name: "email",
      label: t`E-mail address`,
      placeholder: t`user@example.com`,
      htmlType: "email",
      required: true,
      value: email,
      onChange: (value: string) => setEmail(value),
    },
    {
      id: "password",
      name: "password",
      label: t`Password (optional)`,
      placeholder: "YuXQFHDK2OF8Or",
      htmlType: "password",
      required: false,
      value: password,
      onChange: (value: string) => setPassword(value),
    },
    {
      id: "roles",
      name: "roles",
      title: t`Roles`,
      description: t`These roles will be assign to the user and determine what they can and cannot do.`,
      onChange: (value, checked) =>
        setRoles(
          checked
            ? Array.from(new Set([...roles, value as Role]))
            : roles.filter((role) => role !== value),
        ),
      choices: [
        {
          label: i18n._(Active.title),
          description: i18n._(Active.description),
          checked: true,
          disabled: true,
          value: Role.Active,
        },
        {
          label: i18n._(Viewer.title),
          description: i18n._(Viewer.description),
          checked: false,
          value: Role.Viewer,
        },
        {
          label: i18n._(Editor.title),
          description: i18n._(Editor.description),
          checked: false,
          value: Role.Editor,
        },
        {
          label: i18n._(Administrator.title),
          description: i18n._(Administrator.description),
          checked: false,
          value: Role.Administrator,
        },
      ],
    },
  ];

  function onSubmit() {
    api
      .createUser(
        token as string,
        activeOrganization?.organization.id as number,
        { name, email, password: password || null, roles },
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
      title={`Create user`}
      description={
        <Trans>
          Use this form to create a new user. All users can sign in via a Google
          or Microsoft account matching the entered e-mail address. Filling in a
          password is optional.
        </Trans>
      }
      submitText={t`Create`}
      cancelButton
      open={open}
      close={() => setOpen(false)}
      onSubmit={onSubmit}
      controls={controls}
    />
  );
}
