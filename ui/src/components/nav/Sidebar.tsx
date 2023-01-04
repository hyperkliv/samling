import {
  AdjustmentsHorizontalIcon,
  TruckIcon,
  RectangleStackIcon,
} from "@heroicons/react/24/outline";
import { t } from "@lingui/macro";
import SidebarMobile from "./SidebarMobile";
import { Role } from "../../types/api";
import { SidebarOpenProps, NavigationItem } from "./propTypes";
import SidebarDesktop from "./SidebarDesktop";
import { useLocalize } from "../../i18n";
import { useLocation } from "react-router-dom";
import { useAppSelector } from "../../state/hooks";

export default function Sidebar(props: SidebarOpenProps) {
  let { pathname } = useLocation();
  let { activeOrganization } = useAppSelector((state) => state.user);
  const { i18nLink } = useLocalize();

  let navigationItems: NavigationItem[] = [
    {
      text: t`Collections`,
      to: i18nLink("/app"),
      icon: RectangleStackIcon,
      current: false,
    },
  ];

  if (
    !!activeOrganization &&
    activeOrganization.roles.includes(Role.Administrator)
  ) {
    navigationItems.push({
      text: t`Admin`,
      to: i18nLink("/app/admin"),
      icon: AdjustmentsHorizontalIcon,
      current: false,
    });
  }
  navigationItems.push({
    text: t`To public page`,
    to: i18nLink("/"),
    icon: TruckIcon,
    current: false,
  });

  navigationItems.forEach((item) => {
    item.current = item.to === pathname;
  });

  return (
    <div className="print:hidden">
      <SidebarMobile {...props} navigationItems={navigationItems} />
      <SidebarDesktop navigationItems={navigationItems} />
    </div>
  );
}
