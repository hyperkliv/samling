import { t, Trans } from "@lingui/macro";
import LocaleLink from "../../components/LocaleLink";
import { useTitle } from "../../hooks";
import { User } from "../../types/api";

interface Params {
  user: User;
}

export default function AlreadySignedInPage({ user }: Params) {
  useTitle([t`Already signed in`]);
  return (
    <div className="min-h-full pt-16 pb-12 flex flex-col bg-white">
      <main className="flex-grow flex flex-col justify-center max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex-shrink-0 flex justify-center">
          <LocaleLink to="/app" className="inline-flex">
            <span className="sr-only">Samling</span>
            <img className="h-12 w-auto" src="/logo.webp" alt="" />
          </LocaleLink>
        </div>
        <div className="py-16">
          <div className="text-center">
            <p className="text-sm font-semibold text-indigo-600 uppercase tracking-wide">
              <Trans>Error</Trans>
            </p>
            <h1 className="mt-2 text-4xl font-extrabold text-gray-900 tracking-tight sm:text-5xl">
              <Trans>Already signed in.</Trans>
            </h1>
            <p className="mt-2 text-base text-gray-500">
              <Trans>
                You are already signed in as {user.name} ({user.email})
              </Trans>
            </p>
            <div className="mt-6">
              <LocaleLink
                to="/auth/logout"
                className="text-base font-medium text-indigo-600 hover:text-indigo-500"
              >
                <Trans>Sign out</Trans>
                <span aria-hidden="true"> &rarr;</span>
              </LocaleLink>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
