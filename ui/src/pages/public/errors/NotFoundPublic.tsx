import { Trans } from "@lingui/macro";
import LocaleLink from "../../../components/LocaleLink";

export default function NotFoundPublic() {
  return (
    <div className="min-h-full pt-16 pb-12 flex flex-col bg-white">
      <main className="flex-grow flex flex-col justify-center max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex-shrink-0 flex justify-center">
          <LocaleLink to="/" className="inline-flex">
            <span className="sr-only">Samling</span>
            <img className="h-12 w-auto" src="/logo.webp" alt="" />
          </LocaleLink>
        </div>
        <div className="py-16">
          <div className="text-center">
            <p className="text-sm font-semibold text-indigo-600 uppercase tracking-wide">
              <Trans>404 Error</Trans>
            </p>
            <h1 className="mt-2 text-4xl font-extrabold text-gray-900 tracking-tight sm:text-5xl">
              <Trans>Page not found.</Trans>
            </h1>
            <p className="mt-2 text-base text-gray-500">
              <Trans>
                Sorry, we couldn’t find the page you’re looking for.
              </Trans>
            </p>
            <div className="mt-6">
              <LocaleLink
                to="/"
                className="text-base font-medium text-indigo-600 hover:text-indigo-500"
              >
                <Trans>Go back home</Trans>
                <span aria-hidden="true"> &rarr;</span>
              </LocaleLink>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
