import { Dispatch, SetStateAction } from "react";
import { classNames } from "../../utils";

interface InputProps {
  id: string;
  placeholder?: string;
  title: string;
  description?: string | null;
  value: string;
  setValue: Dispatch<SetStateAction<string>>;
  required: boolean;
  type?: string;
  disabled?: boolean;
}

export default function Input({
  id,
  placeholder,
  title,
  description,
  value,
  setValue,
  required,
  type,
  disabled,
}: InputProps) {
  return (
    <>
      <label htmlFor={id} className="block text-sm font-medium text-gray-700">
        {title}
      </label>
      <div className="mt-2">
        <input
          type={type || "text"}
          name={id}
          id={id}
          value={value}
          required={disabled ? false : required}
          onChange={(e) => setValue(e.target.value)}
          disabled={disabled}
          className={classNames(
            disabled ? "bg-gray-100" : "",
            "block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
          )}
          placeholder={placeholder}
          aria-describedby={`${id}-description`}
        />
      </div>
      <p className="mt-2 text-sm text-gray-500" id={`${id}-description`}>
        {description}
      </p>
    </>
  );
}
