import { NavigationItem } from "./propTypes";
import ProfileDropdown from "./ProfileDropdown";
import { classNames } from "../../utils";
import { t, Trans } from "@lingui/macro";
import LocaleDropdown from "./LocaleDropdown";
import { Link } from "react-router-dom";
import EnvironmentWarning from "../EnvironmentWarning";
import { useAppSelector } from "../../state/hooks";

interface Props {
  navigationItems: NavigationItem[];
}

export default function SidebarDesktop(props: Props) {
  const { activeOrganization } = useAppSelector((state) => state.user);
  const logoUrl = activeOrganization?.organization.logo_url;
  return (
    <div className="hidden lg:flex lg:flex-col lg:w-64 lg:fixed lg:inset-y-0 lg:border-r lg:border-gray-200 lg:pt-6 lg:pb-6 lg:bg-gray-100">
      <div className="flex items-center justify-center flex-shrink-0 px-6">
        {logoUrl && (
          <img
            className="h-12 w-auto"
            src={logoUrl}
            alt={t`${activeOrganization.organization.name} logotype`}
          />
        )}
      </div>
      {/* Sidebar component, swap this element with another sidebar if you like */}
      <div className="mt-6 h-0 flex-1 flex flex-col overflow-y-auto">
        <div className="mt-1 mx-2 inline-block text-left">
          <ProfileDropdown />
        </div>
        {/* TODO: Implement! <SidebarSearch /> */}
        {/* Navigation */}
        <nav className="px-3 mt-6">
          <div className="space-y-1">
            {props.navigationItems.map((item) => (
              <Link
                key={item.text}
                to={item.to}
                className={classNames(
                  item.current
                    ? "bg-gray-200 text-gray-900"
                    : "text-gray-700 hover:text-gray-900 hover:bg-gray-50",
                  "group flex items-center px-2 py-2 text-sm font-medium rounded-md outline-none focus:ring-2 focus:ring-indigo-500",
                )}
                aria-current={item.current ? "page" : undefined}
              >
                <item.icon
                  className={classNames(
                    item.current
                      ? "text-gray-500"
                      : "text-gray-400 group-hover:text-gray-500",
                    "mr-3 flex-shrink-0 h-6 w-6",
                  )}
                  aria-hidden="true"
                />
                {item.text}
              </Link>
            ))}
          </div>
          <div className="mt-8">
            <h3
              className="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider"
              id="language-dropdown-headline"
            >
              <Trans>Language</Trans>
            </h3>
            <div
              className="mt-1 space-y-1"
              role="group"
              aria-labelledby="recent-headline"
            >
              <LocaleDropdown />
            </div>
          </div>
        </nav>
      </div>
      <EnvironmentWarning />
      <div className="flex items-center justify-center flex-shrink-0 px-6 pt-6">
        <img
          className="h-6 w-auto"
          src="/logo.webp"
          alt={t`Samling.io logotype`}
        />
      </div>
    </div>
  );
}
