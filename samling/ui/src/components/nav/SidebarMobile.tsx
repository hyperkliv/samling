import { Fragment } from "react";
import { Dialog, Transition } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { classNames } from "../../utils";
import { t, Trans } from "@lingui/macro";
import { NavigationItem, SidebarOpenProps } from "./propTypes";
import { useAppSelector } from "../../state/hooks";
import { Link } from "react-router-dom";
import LocaleDropdown from "./LocaleDropdown";

export interface SidebarProps extends SidebarOpenProps {
  navigationItems: NavigationItem[];
}

export default function SidebarMobile(props: SidebarProps) {
  const { activeOrganization } = useAppSelector((state) => state.user);
  const logoUrl = activeOrganization?.organization.logo_url;
  return (
    <Transition.Root show={props.sidebarOpen} as={Fragment}>
      <Dialog
        as="div"
        className="relative z-40 lg:hidden"
        onClose={props.setSidebarOpen}
      >
        <Transition.Child
          as={Fragment}
          enter="transition-opacity ease-linear duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="transition-opacity ease-linear duration-300"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-600 bg-opacity-75" />
        </Transition.Child>

        <div className="fixed inset-0 flex z-40">
          <Transition.Child
            as={Fragment}
            enter="transition ease-in-out duration-300 transform"
            enterFrom="-translate-x-full"
            enterTo="translate-x-0"
            leave="transition ease-in-out duration-300 transform"
            leaveFrom="translate-x-0"
            leaveTo="-translate-x-full"
          >
            <Dialog.Panel className="relative flex-1 flex flex-col max-w-xs w-full pt-5 pb-4 bg-white">
              <Transition.Child
                as={Fragment}
                enter="ease-in-out duration-300"
                enterFrom="opacity-0"
                enterTo="opacity-100"
                leave="ease-in-out duration-300"
                leaveFrom="opacity-100"
                leaveTo="opacity-0"
              >
                <div className="absolute top-0 right-0 -mr-12 pt-2">
                  <button
                    type="button"
                    className="ml-1 flex items-center justify-center h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                    onClick={() => props.setSidebarOpen(false)}
                  >
                    <span className="sr-only">
                      <Trans>Close sidebar</Trans>
                    </span>
                    <XMarkIcon
                      className="h-6 w-6 text-white"
                      aria-hidden="true"
                    />
                  </button>
                </div>
              </Transition.Child>
              <div className="flex items-center justify-center flex-shrink-0">
                {logoUrl && (
                  <img
                    className="h-12 w-auto"
                    src={logoUrl}
                    alt={t`${activeOrganization.organization.name} logotype`}
                  />
                )}
              </div>
              {/* Sidebar component, swap this element with another sidebar if you like */}
              <div className="mt-2 h-0 flex-1 flex flex-col overflow-y-auto">
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
              <div className="flex items-center justify-center flex-shrink-0 px-6 pt-6">
                <img
                  className="h-6 w-auto"
                  src="/logo.webp"
                  alt={t`Samling.io logotype`}
                />
              </div>
            </Dialog.Panel>
          </Transition.Child>
          <div className="flex-shrink-0 w-14" aria-hidden="true">
            {/* Dummy element to force sidebar to shrink to fit close icon */}
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}
