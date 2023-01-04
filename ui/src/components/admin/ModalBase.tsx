import {
  FormEvent,
  Fragment,
  HTMLInputTypeAttribute,
  ReactElement,
} from "react";
import { Dialog, Transition } from "@headlessui/react";
import {
  ExclamationTriangleIcon,
  XMarkIcon,
} from "@heroicons/react/24/outline";
import { Trans } from "@lingui/macro";
import { classNames } from "../../utils";

interface Props {
  title: string;
  description: ReactElement;
  submitText: string;
  cancelButton?: boolean;
  open: boolean;
  close: () => void;
  controls: FormControlType[];
  onSubmit: () => void;
  danger?: boolean;
}

export type FormControlType = FormInput | FormCheckboxes;

interface FormControlBase {
  id: string;
  name: string;
  description?: string;
  disabled?: boolean;
}

export interface FormInput extends FormControlBase {
  placeholder: string;
  label: string;
  value: string;
  htmlType: HTMLInputTypeAttribute;
  required: boolean;
  onChange: (value: string) => void;
}

interface FormCheckboxValue {
  label: string;
  description?: string;
  checked: boolean;
  value: string;
  disabled?: boolean;
}

export interface FormCheckboxes extends FormControlBase {
  title: string;
  choices: FormCheckboxValue[];
  onChange: (value: string, checked: boolean) => void;
}

function isInput(control: FormControlBase): control is FormInput {
  return (
    (control as FormInput).htmlType !== undefined &&
    (control as FormInput).placeholder !== undefined
  );
}

function isCheckboxes(control: FormControlBase): control is FormCheckboxes {
  return (control as FormCheckboxes).choices !== undefined;
}

interface FormControlProps {
  control: FormControlType;
}

function FormControl({ control }: FormControlProps) {
  if (isInput(control)) {
    return (
      <div className="my-3">
        <label className="text-sm" htmlFor={control.id}>
          {control.label}
        </label>
        {control.description ? (
          <p className="text-gray-500">{control.description}</p>
        ) : (
          ""
        )}
        <input
          type={control.htmlType}
          name={control.name}
          id={control.id}
          disabled={!!control.disabled}
          value={control.value}
          onChange={(evt) => control.onChange(evt.target.value)}
          required={control.required}
          className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
          placeholder={control.placeholder}
        />
      </div>
    );
  } else if (isCheckboxes(control)) {
    return (
      <div className="my-4">
        <div className="my-3">
          <h2 className="text-lg">{control.title}</h2>
          {control.description ? (
            <p className="text-sm text-gray-500">{control.description}</p>
          ) : (
            ""
          )}
        </div>
        {control.choices.map((checkbox) => (
          <div key={`${control.id}-${checkbox.value}`} className="my-2">
            <input
              key={checkbox.value}
              type="checkbox"
              name={control.name}
              defaultChecked={checkbox.checked}
              id={`${control.id}-${checkbox.value}`}
              disabled={!!checkbox.disabled || !!control.disabled}
              value={checkbox.value}
              onChange={(evt) =>
                control.onChange(evt.target.value, evt.target.checked)
              }
              className="inline-block rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
            />
            <label
              className="pl-2 text-gray-700 text-sm"
              htmlFor={`${control.id}-${checkbox.value}`}
            >
              {checkbox.label}
            </label>
            {checkbox.description ? (
              <p className="text-sm text-gray-500">{checkbox.description}</p>
            ) : (
              ""
            )}
          </div>
        ))}
      </div>
    );
  } else {
    throw Error(`No support for control ${control}`);
  }
}

export default function Modal({
  title,
  description,
  submitText,
  controls,
  open,
  close,
  onSubmit,
  cancelButton,
  danger,
}: Props) {
  function onSubmitForm(e: FormEvent<HTMLFormElement>) {
    e.stopPropagation();
    e.preventDefault();
    onSubmit();
  }

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog as="div" className="relative z-10" onClose={close}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6">
                <div className="absolute top-0 right-0 hidden pt-4 pr-4 sm:block">
                  <button
                    type="button"
                    className="rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    onClick={close}
                  >
                    <span className="sr-only">Close</span>
                    <XMarkIcon className="h-6 w-6" aria-hidden="true" />
                  </button>
                </div>
                <div>
                  <div className="px-4 py-5 sm:p-6">
                    <Dialog.Title
                      as="h3"
                      className="text-lg font-medium leading-6 text-gray-900"
                    >
                      {danger ? (
                        <ExclamationTriangleIcon
                          className="h-6 w-6 mr-2 inline text-red-600"
                          aria-hidden="true"
                        />
                      ) : (
                        ""
                      )}
                      {title}
                    </Dialog.Title>
                    <div className="mt-2 max-w-xl text-sm text-gray-500">
                      <p>{description}</p>
                    </div>
                    <form onSubmit={onSubmitForm}>
                      <div className="my-5">
                        {controls.map((control) => (
                          <FormControl key={control.id} control={control} />
                        ))}
                      </div>
                      <div className="mt-6 sm:mt-6 sm:flex sm:flex-row-reverse">
                        <button
                          type="submit"
                          className={classNames(
                            danger
                              ? "bg-red-600 hover:bg-red-700 focus:ring-red-500"
                              : "bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500",
                            "mt-3 inline-flex w-full items-center justify-center rounded-md border border-transparent px-4 py-2 font-medium text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm",
                          )}
                        >
                          {submitText}
                        </button>
                        {cancelButton ? (
                          <button
                            type="button"
                            className="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:w-auto sm:text-sm"
                            onClick={close}
                          >
                            <Trans>Cancel</Trans>
                          </button>
                        ) : (
                          ""
                        )}
                      </div>
                    </form>
                  </div>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}
