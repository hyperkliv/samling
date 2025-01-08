import { ExclamationTriangleIcon } from "@heroicons/react/20/solid";
import { Trans } from "@lingui/macro";

import { useAppSelector } from "../state/hooks";
import { Environment } from "../types/api";
import { camelCaseToCapitalized } from "../utils";

export default function EnvironmentWarning() {
  const { environment } = useAppSelector((state) => state.user);
  if (environment === null || environment === Environment.Production) {
    return <></>;
  } else {
    return (
      <div className="rounded-md bg-yellow-50 py-2 px-4 lg:py-4 lg:mt-4 lg:mx-4">
        <div className="flex">
          <div className="flex-shrink-0">
            <ExclamationTriangleIcon
              className="h-5 w-5 text-yellow-400"
              aria-hidden="true"
            />
          </div>
          <div className="ml-3">
            <h3 className="text-sm font-medium text-yellow-800">
              {camelCaseToCapitalized(environment)} env
            </h3>
            <div className="mt-2 text-sm text-yellow-700 hidden lg:block">
              <p>
                <Trans>
                  Please go to{" "}
                  <a className="text-indigo-700" href="https://www.samling.io/">
                    Samling.io
                  </a>{" "}
                  for the production environment.
                </Trans>
              </p>
            </div>
          </div>
        </div>
      </div>
    );
  }
}
