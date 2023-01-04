import { Fragment } from "react";
import { Menu, Transition } from "@headlessui/react";
import { ChevronUpDownIcon } from "@heroicons/react/24/solid";
import { classNames } from "../../utils";
import { ProfileDropdownProps, ProfileLinkType } from "./propTypes";
import { t } from "@lingui/macro";
import { UserCircleIcon } from "@heroicons/react/24/outline";
import { Link } from "react-router-dom";
import { useAppSelector } from "../../state/hooks";

export default function ProfileDropdownDesktop(props: ProfileDropdownProps) {
  const { user } = useAppSelector((state) => state.user);
  return (
    <Menu as="div">
      <div>
        <Menu.Button className="group w-full bg-gray-100 rounded-md px-3.5 py-2 text-sm text-left font-medium text-gray-700 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-100 focus:ring-indigo-500">
          <span className="flex w-full justify-between items-center">
            <span className="flex min-w-0 items-center justify-between space-x-3">
              {!user?.profile_image ? (
                <UserCircleIcon className="w-10 h-10 bg-gray-300 rounded-full flex-shrink-0 text-gray-600" />
              ) : (
                <img
                  className="w-10 h-10 bg-gray-300 rounded-full flex-shrink-0"
                  src={user.profile_image}
                  alt={t`User profile`}
                />
              )}
              <span className="flex-1 flex flex-col min-w-0">
                <span className="text-gray-900 text-sm font-medium truncate">
                  {user?.name ? user?.name : t`User profile`}
                </span>
                <span className="text-gray-500 text-sm truncate">
                  {user?.email ? user.email : "–"}
                </span>
              </span>
            </span>
            <ChevronUpDownIcon
              className="flex-shrink-0 h-5 w-5 text-gray-400 group-hover:text-gray-500"
              aria-hidden="true"
            />
          </span>
        </Menu.Button>
      </div>
      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <Menu.Items className="z-10 mx-3 origin-top absolute right-0 left-0 mt-1 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 divide-y divide-gray-200 focus:outline-none">
          {props.linkGroups.map((links, index) => (
            <div className="py-1" key={index}>
              {links.map((link) => (
                <Menu.Item key={link.to}>
                  {({ active }) =>
                    link.type === ProfileLinkType.email ? (
                      <a
                        className={classNames(
                          active
                            ? "bg-gray-100 text-gray-900"
                            : "text-gray-700",
                          "block px-4 py-2 text-sm",
                        )}
                        href={`mailto:${link.to}`}
                      >
                        {link.text}
                      </a>
                    ) : (
                      <Link
                        to={link.to}
                        className={classNames(
                          active
                            ? "bg-gray-100 text-gray-900"
                            : "text-gray-700",
                          "block px-4 py-2 text-sm",
                        )}
                      >
                        {link.text}
                      </Link>
                    )
                  }
                </Menu.Item>
              ))}
            </div>
          ))}
        </Menu.Items>
      </Transition>
    </Menu>
  );
}
