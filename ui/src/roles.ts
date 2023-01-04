import { MessageDescriptor } from "@lingui/core";
import { defineMessage } from "@lingui/macro";
import { Role as ApiRole } from "./types/api";

export interface RoleDetail {
  id: ApiRole;
  title: MessageDescriptor;
  titlePlural: MessageDescriptor;
  description: MessageDescriptor;
}

export const Active: RoleDetail = {
  id: ApiRole.Active,
  title: defineMessage({ message: "Active" }),
  titlePlural: defineMessage({ message: "Active" }),
  description: defineMessage({ message: "Can sign in." }),
};

export const Viewer: RoleDetail = {
  id: ApiRole.Viewer,
  title: defineMessage({ message: "Viewer" }),
  titlePlural: defineMessage({ message: "Viewers" }),
  description: defineMessage({
    message:
      "Can view and export collections where explicit access has been configured in Groups.",
  }),
};

export const Editor: RoleDetail = {
  id: ApiRole.Editor,
  title: defineMessage({ message: "Editor" }),
  titlePlural: defineMessage({ message: "Editors" }),
  description: defineMessage({
    message:
      "Can create and edit groups and collections, in addition to viewer level access.",
  }),
};

export const Administrator: RoleDetail = {
  id: ApiRole.Administrator,
  title: defineMessage({ message: "Administrator" }),
  titlePlural: defineMessage({ message: "Administrators" }),
  description: defineMessage({
    message:
      "Can manage users, in addition to editor level access. Also gets full API access, which is useful when creating integrations.",
  }),
};

export const allRoles: RoleDetail[] = [Active, Viewer, Editor, Administrator];

export function getRoleDetail(role: ApiRole): RoleDetail {
  return allRoles.find((roleDetail) => roleDetail.id === role) as RoleDetail;
}
