import { Link } from "react-router-dom";
import { BreadcrumbItem } from "../../types/other";

interface Props {
  items: BreadcrumbItem[];
}

export default function Breadcrumbs({ items }: Props) {
  return (
    <nav
      className="flex border-b border-gray-200 px-4 py-4 sm:px-6 lg:px-8 print:hidden"
      aria-label="Breadcrumb"
    >
      <ol className="flex items-center space-x-4">
        {items.map((item, index) => (
          <li key={item.title}>
            <h1 className="flex items-center text-lg font-medium leading-6 text-gray-900 sm:truncate">
              {index > 0 ? (
                <svg
                  className="h-5 w-5 flex-shrink-0 text-gray-300"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                  aria-hidden="true"
                >
                  <path d="M5.555 17.776l8-16 .894.448-8 16-.894-.448z" />
                </svg>
              ) : (
                ""
              )}
              {item.current ? (
                <span className="ml-4 text-gray-800 hover:text-gray-700 text-lg font-medium">
                  {item.title}
                </span>
              ) : (
                <Link
                  to={item.to}
                  className="ml-4 text-gray-500 hover:text-gray-700 text-sm"
                >
                  {item.title}
                </Link>
              )}
            </h1>
          </li>
        ))}
      </ol>
    </nav>
  );
}
