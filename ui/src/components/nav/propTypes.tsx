import { Dispatch, SetStateAction } from "react";

export interface NavigationItem {
  text: string;
  to: string;
  icon: Function;
  current: boolean;
}

export interface SidebarOpenProps {
  sidebarOpen: boolean;
  setSidebarOpen: Dispatch<SetStateAction<boolean>>;
}

export enum ProfileLinkType {
  email,
  url,
}

export interface ProfileDropdownLink {
  type?: ProfileLinkType;
  text: string;
  to: string;
}

export interface ProfileDropdownProps {
  linkGroups: ProfileDropdownLink[][];
}
