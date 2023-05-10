import { Dialog, Transition } from "@headlessui/react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { t } from "@lingui/macro";
import {
  Dispatch,
  FormEvent,
  Fragment,
  SetStateAction,
  useEffect,
  useState,
} from "react";
import { useNestedStyles } from "../../api";
import { useLocalize } from "../../i18n";
import { cloudflareImageUrl } from "../../images";
import { EditableStyle, makeEditableStyle } from "../../types/admin";
import {
  EntityFilterChoice,
  ItemFilterChoices,
  NestedStyle,
  NestedStyleSummary,
  StyleFilters,
} from "../../types/api";
import MultipleCombobox from "./MultipleCombobox";

type ItemStatus = string;

export default function AddCollectionItemsModal({
  open,
  setOpen,
  collectionStylesMap,
  allNestedStylesMap,
  setEditableStyles,
  itemFilterChoices,
}: {
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
  collectionStylesMap: Map<number, NestedStyle>;
  allNestedStylesMap: Map<number, NestedStyleSummary>;
  setEditableStyles: Dispatch<SetStateAction<EditableStyle[]>>;
  itemFilterChoices: ItemFilterChoices;
}) {
  const { i18nDbText } = useLocalize();

  const [filters, setFilters] = useState(() => ({} as StyleFilters));
  const [nestedStylesResult] = useNestedStyles(filters);
  const nestedStyles = nestedStylesResult.unwrapOr([]) || [];

  // Status filter
  const [filteredStatuses, setFilteredStatuses] = useState(
    () => [] as ItemStatus[],
  );

  // Style filter
  const [filteredStyles, setFilteredStyles] = useState(
    () => [] as EntityFilterChoice[],
  );

  // Category filter
  const [filteredCategories, setFilteredCategories] = useState(
    () => [] as EntityFilterChoice[],
  );

  // Attribute filter
  const [filteredAttributes, setFilteredAttributes] = useState(
    () => [] as EntityFilterChoice[],
  );

  // Apply filters
  useEffect(() => {
    setFilters({
      status: filteredStatuses.length > 0 ? filteredStatuses : null,
      refs:
        filteredStyles.length > 0
          ? filteredStyles.map((style) => ({ id: style.id }))
          : null,
      categories:
        filteredCategories.length > 0
          ? filteredCategories.map((cat) => ({ id: cat.id }))
          : null,
      attributes:
        filteredAttributes.length > 0
          ? filteredAttributes.map((cat) => ({ id: cat.id }))
          : null,
    });
  }, [filteredStatuses, filteredCategories, filteredAttributes, filteredStyles, setFilters]);

  function onSubmitForm(evt: FormEvent<HTMLFormElement>) {
    evt.preventDefault();
    evt.stopPropagation();
    setOpen(false);
    setEditableStyles((prevEditableStyles) => {
      const prevMapIndexes = new Map(
        prevEditableStyles.map((style, idx) => [style.id, idx]),
      );
      const newEditableStyles = nestedStyles.flatMap((nestedStyle) => {
        const style = collectionStylesMap.get(nestedStyle.id);
        const fullStyle = allNestedStylesMap.get(nestedStyle.id);
        if (fullStyle) {
          const newStyle = makeEditableStyle(style || fullStyle, fullStyle);
          const prevIdx = prevMapIndexes.get(nestedStyle.id);
          if (prevIdx === undefined) {
            return [newStyle];
          } else {
            prevEditableStyles[prevIdx] = newStyle;
            return [];
          }
        } else {
          throw Error(
            `Didn't find style and/or fullStyle for ${JSON.stringify(
              nestedStyle,
            )}`,
          );
        }
      });
      return [...newEditableStyles, ...prevEditableStyles];
    });
  }

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog as="div" className="relative z-10" onClose={() => setOpen(false)}>
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
              <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-2xl sm:p-6">
                <div className="absolute top-0 right-0 hidden pt-4 pr-4 sm:block">
                  <button
                    type="button"
                    className="rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    onClick={() => setOpen(false)}
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
                      {t`Add collection items`}
                    </Dialog.Title>
                    <div className="mt-2 max-w-xl text-sm text-gray-500">
                      <p>{t`Make your selection of items to add here. Manual adjustments can be made once added.`}</p>
                    </div>
                    <form onSubmit={onSubmitForm}>
                      <div className="my-5">
                        <MultipleCombobox
                          title={t`Status`}
                          allItems={itemFilterChoices.status}
                          selectedItems={filteredStatuses}
                          setSelectedItems={setFilteredStatuses}
                          toFilterItem={(status) => ({
                            title: status,
                          })}
                        />
                      </div>
                      <div className="my-5">
                        <MultipleCombobox
                          title={t`Categories`}
                          allItems={itemFilterChoices.category}
                          selectedItems={filteredCategories}
                          setSelectedItems={setFilteredCategories}
                          toFilterItem={(choice) => ({
                            id: choice.id,
                            title: i18nDbText(choice.title),
                            subtitle: choice.subtitle ? i18nDbText(choice.subtitle) : '',
                            imageUrl: choice.image ? cloudflareImageUrl(choice.image.url, "thumbnail") : null,
                          })}
                        />
                      </div>
                      <div className="my-5">
                        <MultipleCombobox
                          title={t`Attributes`}
                          description={`Different types of attributes must all get matched. Within the same type any may match.`}
                          allItems={itemFilterChoices.attribute}
                          selectedItems={filteredAttributes}
                          setSelectedItems={setFilteredAttributes}
                          toFilterItem={(choice) => ({
                            id: choice.id,
                            title: i18nDbText(choice.title),
                            bubblePrefix: choice.subtitle ? i18nDbText(choice.subtitle) : '',
                            subtitle: choice.subtitle ? i18nDbText(choice.subtitle) : '',
                          })}
                          numSelectedVisible={2}
                        />
                      </div>
                      <div className="my-5">
                        <MultipleCombobox
                          title={t`Style`}
                          allItems={itemFilterChoices.style}
                          selectedItems={filteredStyles}
                          setSelectedItems={setFilteredStyles}
                          toFilterItem={(choice) => ({
                            id: choice.id,
                            title: i18nDbText(choice.title),
                            subtitle: choice.subtitle ? i18nDbText(choice.subtitle) : '',
                            imageUrl: choice.image ? cloudflareImageUrl(choice.image.url, "thumbnail") : null,
                          })}
                        />
                      </div>
                      <div>
                        <ItemStats nestedStyles={nestedStyles} />
                      </div>
                      <div className="mt-6 sm:mt-6 sm:flex sm:flex-row-reverse">
                        <button
                          type="submit"
                          className="bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500 mt-3 inline-flex w-full items-center justify-center rounded-md border border-transparent px-4 py-2 font-medium text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
                        >
                          {t`Add`}
                        </button>
                        <button
                          type="button"
                          className="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:w-auto sm:text-sm"
                          onClick={() => setOpen(false)}
                        >
                          {t`Cancel`}
                        </button>
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

function ItemStats({ nestedStyles }: { nestedStyles: NestedStyleSummary[] }) {
  const numStyles = nestedStyles.length;
  const numColors = nestedStyles
    .map((style) => style.colors.length)
    .reduce((sum, count) => sum + count, 0);
  const numSizes = nestedStyles
    .flatMap((style) => style.colors.map((color) => color.sizes.length))
    .reduce((sum, count) => sum + count, 0);
  const stats = [
    { title: t`Number of styles`, stat: numStyles },
    { title: t`Number of colors`, stat: numColors },
    { title: t`Number of sizes`, stat: numSizes },
  ];
  return (
    <div>
      <h3 className="text-lg font-medium leading-6 text-gray-900">
        {t`Results from current filters`}
      </h3>
      <dl className="mt-5 grid grid-cols-1 gap-5 sm:grid-cols-3">
        {stats.map((item) => (
          <div
            key={item.title}
            className="overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:p-6"
          >
            <dt className="truncate text-sm font-medium text-gray-500">
              {item.title}
            </dt>
            <dd className="mt-1 text-3xl font-semibold tracking-tight text-gray-900">
              {item.stat}
            </dd>
          </div>
        ))}
      </dl>
    </div>
  );
}
