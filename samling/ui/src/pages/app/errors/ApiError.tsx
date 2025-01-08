import { Trans } from "@lingui/macro";
import LocaleLink from "../../../components/LocaleLink";
import { ApiErrorResponse } from "../../../types/api";
import { camelCaseToConstantCase } from "../../../utils";

interface Props {
  error: ApiErrorResponse;
}

export default function ApiError({ error }: Props) {
  return (
    <div className="flex min-h-full flex-col bg-white pt-16 pb-12">
      <main className="mx-auto flex w-full max-w-7xl flex-grow flex-col justify-center px-4 sm:px-6 lg:px-8">
        <div className="flex flex-shrink-0 justify-center">
          <a href="/" className="inline-flex">
            <span className="sr-only">Samling.io</span>
            <img className="h-12 w-auto" src="/logo.webp" alt="" />
          </a>
        </div>
        <div className="py-16">
          <div className="text-center">
            <p className="text-base font-semibold text-indigo-600">
              {camelCaseToConstantCase(error.error_code)}
            </p>
            <h1 className="mt-2 text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
              <Trans>An error occurred.</Trans>
            </h1>
            <p className="mt-4 text-base text-gray-500">
              {error.error_message}
            </p>
            <div className="mt-6">
              <LocaleLink
                to="/app"
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
