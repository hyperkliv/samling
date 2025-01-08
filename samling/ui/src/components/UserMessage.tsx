/* This example requires Tailwind CSS v2.0+ */
import { Fragment, useEffect, useState } from "react";
import { Transition } from "@headlessui/react";
import { CheckCircleIcon, XCircleIcon } from "@heroicons/react/24/outline";
import { XMarkIcon } from "@heroicons/react/24/solid";
import * as messageTypes from "../types/messages";
import { Trans } from "@lingui/macro";
import { dismissUserMessage } from "../state/slices/user";
import { useAppDispatch } from "../state/hooks";

interface Props {
  message: messageTypes.UserMessage;
}

export default function UserMessage({ message }: Props) {
  const [show, setShow] = useState(false);
  const dispatch = useAppDispatch();

  useEffect(() => {
    setTimeout(() => {
      setShow(true);
    }, 100);

    if (message.dismissAfter) {
      setTimeout(() => {
        setShow(false);
        dispatch(dismissUserMessage(message.id));
      }, message.dismissAfter);
    }
  });

  return (
    <>
      {/* Global notification live region, render this permanently at the end of the document */}
      <div
        aria-live="assertive"
        className="flex flex-col items-end sm:items-start mx-4 my-6 sm:m-6"
      >
        <div className="w-full flex flex-col items-center space-y-4 sm:items-end">
          {/* Notification panel, dynamically insert this into the live region when it needs to be displayed */}
          <Transition
            as={Fragment}
            show={show}
            enter="transform ease-out duration-300 transition"
            enterFrom="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-2"
            enterTo="translate-y-0 opacity-100 sm:translate-x-0"
            leave="transition ease-in duration-100"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="max-w-sm w-full bg-white shadow-lg rounded-lg pointer-events-auto ring-1 ring-black ring-opacity-5 overflow-hidden">
              <div className="p-4">
                <div className="flex items-start">
                  <div className="flex-shrink-0">
                    {message.level === messageTypes.UserMessageLevel.Error ? (
                      <XCircleIcon
                        className="h-6 w-6 text-red-400"
                        aria-hidden="true"
                      />
                    ) : message.level === messageTypes.UserMessageLevel.Warn ? (
                      <CheckCircleIcon
                        className="h-6 w-6 text-yellow-400"
                        aria-hidden="true"
                      />
                    ) : (
                      <CheckCircleIcon
                        className="h-6 w-6 text-indigo-400"
                        aria-hidden="true"
                      />
                    )}
                  </div>
                  <div className="ml-3 w-0 flex-1 pt-0.5">
                    <p className="text-sm font-medium text-gray-900">
                      {message.title}
                    </p>
                    <p className="mt-1 text-sm text-gray-500">{message.body}</p>
                  </div>
                  <div className="ml-4 flex-shrink-0 flex">
                    <button
                      type="button"
                      className="bg-white rounded-md inline-flex text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                      onClick={() => {
                        dispatch(dismissUserMessage(message.id));
                      }}
                    >
                      <span className="sr-only">
                        <Trans>Close</Trans>
                      </span>
                      <XMarkIcon className="h-5 w-5" aria-hidden="true" />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </Transition>
        </div>
      </div>
    </>
  );
}
