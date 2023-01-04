import { useLocalize } from "../../../i18n";
import { t, Trans } from "@lingui/macro";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import { useTitle } from "../../../hooks";
import {
  CircleStackIcon,
  RectangleGroupIcon,
  UserGroupIcon,
} from "@heroicons/react/24/outline";
import LocaleLink from "../../../components/LocaleLink";

interface AdminLink {
  name: string;
  to: string;
  linkText: string;
  description: string;
  icon: Function;
}

export default function AdminPage() {
  useTitle([t`Admin`]);
  const { i18nLink } = useLocalize();
  const adminLinks: AdminLink[] = [
    {
      name: t`Users`,
      to: "/app/admin/users",
      linkText: t`Administrate users`,
      description: t`Manage users, associate roles and groups individually.`,
      icon: UserGroupIcon,
    },
    {
      name: t`Groups`,
      to: "/app/admin/groups",
      linkText: t`Administrate groups`,
      description: t`Use groups to specify which collections and price lists users have access to.`,
      icon: RectangleGroupIcon,
    },
    {
      name: t`Collections`,
      to: "/app/admin/collections",
      linkText: t`Administrate collections`,
      description: t`This is where you create collections and associate styles, colors and sizes to them.`,
      icon: CircleStackIcon,
    },
  ];
  return (
    <>
      <Breadcrumbs
        items={[{ title: t`Admin`, to: i18nLink("/app/admin"), current: true }]}
      />
      <section
        className="mx-auto max-w-7xl pt-20 pb-32 px-4 sm:px-6 lg:px-8"
        aria-labelledby="contact-heading"
      >
        <h2 className="sr-only" id="contact-heading">
          <Trans>Admin</Trans>
        </h2>
        <div className="grid grid-cols-1 gap-y-20 lg:grid-cols-3 lg:gap-y-0 lg:gap-x-8">
          {adminLinks.map((link) => (
            <div
              key={link.name}
              className="flex flex-col rounded-2xl bg-white shadow-xl"
            >
              <div className="relative flex-1 px-6 pt-16 pb-8 md:px-8">
                <div className="absolute top-0 inline-block -translate-y-1/2 transform rounded-xl bg-indigo-600 p-5 shadow-lg">
                  <link.icon
                    className="h-6 w-6 text-white"
                    aria-hidden="true"
                  />
                </div>
                <h3 className="text-xl font-medium text-gray-900">
                  {link.name}
                </h3>
                <p className="mt-4 text-base text-gray-500">
                  {link.description}
                </p>
              </div>
              <div className="rounded-bl-2xl rounded-br-2xl bg-gray-50 p-6 md:px-8">
                <LocaleLink
                  to={link.to}
                  className="text-base font-medium text-indigo-700 hover:text-indigo-600"
                >
                  {link.linkText}
                  <span aria-hidden="true"> &rarr;</span>
                </LocaleLink>
              </div>
            </div>
          ))}
        </div>
      </section>
    </>
  );
}
