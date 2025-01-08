import { useLocation } from "react-router-dom";
import { locales } from "../../i18n";
import LocaleLink from "../LocaleLink";

export default function LocaleDropdown() {
  let loc = useLocation();
  return (
    <>
      {Object.entries(locales).map(([code, name]) => (
        <LocaleLink
          override={code}
          key={code}
          to={loc.pathname.substring(3) + loc.search}
          className="group flex items-center px-3 py-2 text-sm font-medium text-gray-700 rounded-md hover:text-gray-900 hover:bg-gray-5 outline-none focus:ring-2 focus:ring-indigo-500"
        >
          <span className="truncate">{name}</span>
        </LocaleLink>
      ))}
    </>
  );
}
