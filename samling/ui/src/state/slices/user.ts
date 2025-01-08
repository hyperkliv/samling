import { t } from "@lingui/macro";
import * as uuid from "uuid";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import {
  SimpleUserMessage,
  UserMessage,
  UserMessageLevel,
  UserMessageReplaceGroup,
  UserMessageOpts,
} from "../../types/messages";
import {
  ApiErrorResponse,
  Environment,
  User,
  AuthenticatedUser,
  UserOrganization,
} from "../../types/api";
import { camelCaseToConstantCase } from "../../utils";

const { Info, Warn, Error } = UserMessageLevel;

interface UserState {
  token: null | string;
  user: null | User;
  environment: null | Environment;
  activeOrganization: null | UserOrganization;
  messages: UserMessage[];
}

const initialState: UserState = {
  token: null,
  user: null,
  environment: null,
  activeOrganization: null,
  messages: [],
};

export const slice = createSlice({
  name: "user",
  initialState,
  reducers: {
    userErrorMessage: (state, { payload: body }: PayloadAction<string>) => {
      newMessage(state.messages, Error, t`An error occurred`, body);
    },
    userMessage: (
      state,
      { payload: msg }: PayloadAction<SimpleUserMessage>,
    ) => {
      newMessage(state.messages, msg.level, msg.title, msg.body, msg.opts);
    },
    apiErrorMessage: (
      state,
      { payload: error }: PayloadAction<ApiErrorResponse>,
    ) => {
      newMessage(
        state.messages,
        Error,
        t`Backend error`,
        `${error.error_message} [${camelCaseToConstantCase(error.error_code)}]`,
      );
    },
    dismissUserMessage: (state, { payload: id }: PayloadAction<string>) => {
      state.messages = state.messages.filter((m) => m.id !== id);
    },
    loginUser: (
      state,
      {
        payload: { user, token, environment },
      }: PayloadAction<AuthenticatedUser>,
    ) => {
      if (user.organizations.length === 0) {
        slice.caseReducers.noOrganizationAccess(state);
      } else {
        state.token = token;
        state.user = user;
        state.environment = environment;
        state.activeOrganization = user.organizations[0]; // TODO: Support multiple orgs
        newMessage(
          state.messages,
          Info,
          t`Successfully signed in`,
          t`You are now signed in with account ${user.email}.`,
          {
            dismiss: true,
            replaceGroup: UserMessageReplaceGroup.Authentication,
          },
        );
      }
    },
    logoutUser: (state) => {
      const userEmail = state.user?.email;
      state.token = null;
      state.user = null;
      state.activeOrganization = null;
      if (userEmail) {
        newMessage(
          state.messages,
          Info,
          t`Successfully signed out`,
          t`You've signed out from account ${userEmail}.`,
          {
            dismissAfter: 4000,
            replaceGroup: UserMessageReplaceGroup.Authentication,
          },
        );
      }
    },
    logoutExpiredUser: (state) => {
      const userEmail = state.user?.email;
      state.token = null;
      state.user = null;
      state.activeOrganization = null;
      if (userEmail) {
        newMessage(
          state.messages,
          Warn,
          t`Automatic sign out`,
          t`You were automatically signed out from account ${userEmail} because of an expired token. Please sign in again.`,
          {
            dismissAfter: 4000,
            replaceGroup: UserMessageReplaceGroup.Authentication,
          },
        );
      }
    },
    wrongLoginCredentials: (state) => {
      newMessage(
        state.messages,
        Error,
        t`Wrong credentials`,
        t`The given e-mail and password combination does not exist.`,
        {
          dismissAfter: 4000,
          replaceGroup: UserMessageReplaceGroup.Authentication,
        },
      );
    },
    noOrganizationAccess: (state) => {
      newMessage(
        state.messages,
        Error,
        t`No access`,
        t`You don't have access to any organizations. Please contact an administrator.`,
        {
          dismissAfter: 4000,
          replaceGroup: UserMessageReplaceGroup.Authentication,
        },
      );
    },
  },
});

export const {
  userErrorMessage,
  userMessage,
  apiErrorMessage,
  dismissUserMessage,
  loginUser,
  logoutUser,
  logoutExpiredUser,
  wrongLoginCredentials,
} = slice.actions;

export default slice;

function newMessage(
  messages: UserMessage[],
  level: UserMessageLevel,
  title: string,
  body: string,
  opts?: UserMessageOpts,
) {
  if (!!opts?.replaceGroup) {
    messages.forEach((msg) => {
      if (msg.replaceGroup === opts.replaceGroup) {
        msg.dismissed = true;
      }
    });
  }
  const createdAt = new Date().toISOString();
  const message = {
    id: uuid.v4(),
    title,
    body,
    createdAt,
    level,
    dismissed: opts?.dismiss || false,
    dismissAfter: opts?.dismissAfter,
    replaceGroup: opts?.replaceGroup,
  };
  messages.push(message);
}
