import { Fragment } from "react";
import { Menu, Transition } from "@headlessui/react";
import { classNames } from "../../utils";
import { Trans } from "@lingui/macro";
import { ProfileDropdownProps, ProfileLinkType } from "./propTypes";
import { UserCircleIcon } from "@heroicons/react/24/outline";
import { Link } from "react-router-dom";
import { useAppSelector } from "../../state/hooks";

export default function ProfileDropdownMobile(props: ProfileDropdownProps) {
  const { user } = useAppSelector((state) => state.user);
  return (
    <Menu as="div" className="ml-3 relative">
      <div>
        <Menu.Button className="max-w-xs bg-white flex items-center text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
          <span className="sr-only">
            <Trans>Open user menu</Trans>
          </span>
          {!user?.profile_image ? (
            <UserCircleIcon className="h-8 w-8 text-gray-500" />
          ) : (
            <img
              className="h-8 w-8 rounded-full"
              src={user?.profile_image}
              alt=""
            />
          )}
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
        <Menu.Items className="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 divide-y divide-gray-200 focus:outline-none">
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
