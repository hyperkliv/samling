import { Dispatch, SetStateAction } from "react";
import { I18NString } from "../../types/api";
import { classNames } from "../../utils";

interface Locale {
  id: keyof I18NString;
  name: string;
}

const locales: Locale[] = [
  { id: "en", name: "English" },
  { id: "sv", name: "Svenska" },
];

interface I18nStringInputProps {
  id: string;
  placeholder: I18NString;
  title: string;
  description?: string | null;
  value: I18NString;
  setValue: Dispatch<SetStateAction<I18NString>>;
  required: boolean;
}

export default function I18nStringInput({
  id,
  placeholder,
  title,
  description,
  value,
  setValue,
  required,
}: I18nStringInputProps) {
  return (
    <>
      <h2 className="block text-sm font-medium text-gray-700">{title}</h2>
      <div className="isolate -space-y-px rounded-md shadow-sm mt-2">
        {locales.map((locale, idx) => (
          <div
            key={locale.id}
            className={classNames(
              "relative rounded-md border border-gray-300 px-3 py-2 focus-within:z-10 focus-within:border-indigo-600 focus-within:ring-1 focus-within:ring-indigo-600",
              idx === 0 ? "rounded-b-none" : "",
              idx === locales.length - 1 ? "rounded-t-none" : "",
            )}
          >
            <label
              htmlFor={id + "-" + locale.id}
              className="block text-xs mb-1 font-normal text-gray-500"
            >
              {locale.name}
            </label>
            <input
              type="text"
              name={id + "-" + locale.id}
              id={id + "-" + locale.id}
              value={value[locale.id] || ""}
              required={required && locale.id === "en"}
              onChange={(e) =>
                setValue({ ...value, [locale.id]: e.target.value })
              }
              className="block w-full border-0 p-0 text-gray-900 placeholder-gray-500 focus:ring-0 sm:text-sm"
              placeholder={placeholder[locale.id]}
              aria-describedby={`${id}-description`}
            />
          </div>
        ))}
      </div>
      {description ? (
        <p className="mt-2 text-sm text-gray-500" id={`${id}-description`}>
          {description}
        </p>
      ) : (
        ""
      )}
    </>
  );
}
