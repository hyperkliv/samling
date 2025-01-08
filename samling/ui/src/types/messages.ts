export enum UserMessageLevel {
  Error = "error",
  Warn = "warn",
  Info = "info",
}

export enum UserMessageReplaceGroup {
  Authentication = "authentication",
}

export interface UserMessageOpts {
  dismiss?: boolean;
  dismissAfter?: number;
  replaceGroup?: UserMessageReplaceGroup;
}

export interface UserMessage {
  id: string;
  title: string;
  body: string;
  createdAt: string;
  level: UserMessageLevel;
  dismissed: boolean;
  dismissAfter?: number;
  replaceGroup?: UserMessageReplaceGroup;
}

export interface SimpleUserMessage {
  title: string;
  body: string;
  level: UserMessageLevel;
  opts?: UserMessageOpts;
}
