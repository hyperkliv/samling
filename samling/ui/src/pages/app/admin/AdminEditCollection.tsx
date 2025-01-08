import { useLocalize } from "../../../i18n";
import base64 from "base64-js";
import { Plural, t, Trans } from "@lingui/macro";
import api, {
  useCollectionWithItems,
  useItemFilterChoices,
  useNestedStyles,
  usePricelists,
} from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import { match } from "oxide.ts";
import ApiError from "../errors/ApiError";
import Loading from "../../../components/Loading";
import { useParams } from "react-router-dom";
import { useAppDispatch, useAppSelector } from "../../../state/hooks";
import {
  CollectionPricing,
  CollectionWithItems,
  NestedStyleSortOrder,
  NestedStyleSummary,
  PriceListSummary,
  UpdateCollection,
} from "../../../types/api";
import {
  ChangeEventHandler,
  Dispatch,
  Fragment,
  SetStateAction,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import LocaleLink from "../../../components/LocaleLink";
import { apiErrorMessage, userMessage } from "../../../state/slices/user";
import { UserMessageLevel } from "../../../types/messages";
import Input from "../../../components/forms/Input";
import { useTitle } from "../../../hooks";
import I18nStringInput from "../../../components/forms/I18nInput";
import ImageInput from "../../../components/forms/ImageInput";
import { cloudflareImageUrl } from "../../../images";
import LoadingIndicator from "../../../components/LoadingIndicator";
import AssociatedPriceListsEditor from "../../../components/forms/AssociatedPriceListsEditor";
import NewCollectionPricingGroupModal from "../../../components/admin/NewCollectionPricingGroupModal";
import { classNames } from "../../../utils";
import AddCollectionItemsModal from "../../../components/admin/AddCollectionItemsModal";
import {
  EditableColor,
  EditableSize,
  EditableStyle,
  makeEditableStyle,
} from "../../../types/admin";
import { sortBy } from "lodash";

export default function AdminEditCollection() {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const { collectionId } = useParams();
  const { i18nLink, i18nDbText } = useLocalize();
  const collectionRef = useMemo(() => ({ id: collectionId }), [collectionId]);
  const [sortOrder] = useState(NestedStyleSortOrder.NumberAsc);
  const [collectionResult, refreshCollection] = useCollectionWithItems(
    collectionRef,
    sortOrder,
  );
  const allPriceLists = usePricelists().unwrapOr(null);
  const [nestedStyleChoicesResult] = useNestedStyles();
  const collection = collectionResult.unwrapOr(null);
  const fullNestedStyles = nestedStyleChoicesResult.unwrapOr(null);

  useTitle([t`Admin`, t`Collections`, i18nDbText(collection?.name)]);

  return match(collectionResult, {
    Err: (error) => <ApiError error={error} />,
    Ok: () =>
      collection === null ||
      allPriceLists === null ||
      fullNestedStyles == null ? (
        <Loading />
      ) : (
        <>
          <Breadcrumbs
            items={[
              { title: t`Admin`, to: i18nLink("/app/admin"), current: false },
              {
                title: t`Collections`,
                to: i18nLink("/app/admin/collections"),
                current: false,
              },
              {
                title: i18nDbText(collection.name),
                to: i18nLink(`/app/admin/collections/${collectionId}`),
                current: true,
              },
            ]}
          />
          <CollectionEditForm
            collection={collection}
            allPriceLists={allPriceLists}
            fullNestedStyles={fullNestedStyles}
            onSuccess={refreshCollection}
            apiToken={token as string}
            organizationId={activeOrganization?.organization.id as number}
          />
        </>
      ),
  });
}

interface CollectionEditFormProps {
  collection: CollectionWithItems;
  allPriceLists: PriceListSummary[];
  fullNestedStyles: NestedStyleSummary[];
  apiToken: string;
  organizationId: number;
  onSuccess: () => void;
}

function CollectionEditForm({
  apiToken,
  organizationId,
  collection,
  allPriceLists,
  fullNestedStyles,
  onSuccess,
}: CollectionEditFormProps) {
  const { i18nDbText } = useLocalize();
  const dispatch = useAppDispatch();

  const [submitting, setSubmitting] = useState(false);
  const [name, setName] = useState(collection.name);
  const [pricingItems, setPricingItems] = useState(() => {
    // NOTE: This is very important. We make sure that the pricingItem contains the
    //       exact same PriceListSummary object.
    return collection.pricing.map(
      (pricingItem) =>
        ({
          date: pricingItem.date,
          list: allPriceLists.find((pl) => pl.id === pricingItem.list.id),
        } as CollectionPricing),
    );
  });
  const [slug, setSlug] = useState(collection.slug);
  const [newImage, setNewImage] = useState(null as File | null);
  const [externalId, setExternalId] = useState(collection.external_id);
  const collectionStyles = useMemo(
    () => collection.items.map((i) => i.style),
    [collection],
  );
  const nestedStylesMap = useMemo(() => {
    const map: Map<number, NestedStyleSummary> = new Map();
    fullNestedStyles.forEach((style) => {
      map.set(style.id, style);
    });
    return map;
  }, [fullNestedStyles]);
  const [editableStyles, setEditableStyles] = useState(
    collectionStyles.map((style) =>
      makeEditableStyle(
        style,
        nestedStylesMap.get(style.id) as NestedStyleSummary,
      ),
    ),
  );

  function submitFormImpl(updateCollection: UpdateCollection) {
    api
      .updateCollection(
        apiToken,
        organizationId,
        collection.id,
        updateCollection,
      )
      .then((res) => {
        match(res, {
          Ok: (collection) => {
            dispatch(
              userMessage({
                level: UserMessageLevel.Info,
                title: t`Success!`,
                body: t`The collection "${i18nDbText(
                  collection.name,
                )}" was successfully updated.`,
                opts: { dismissAfter: 3000 },
              }),
            );
            setNewImage(null); // To ensure that new image is not uploaded again
            onSuccess();
          },
          Err: (error) => {
            dispatch(apiErrorMessage(error));
          },
        });
      })
      .finally(() => {
        setSubmitting(false);
      });
  }

  function submitForm() {
    setSubmitting(true);
    const bufferPromise = newImage?.arrayBuffer();
    const sizes = editableStyles.flatMap((style) =>
      style.colors.flatMap((color) =>
        color.sizes
          .filter((size) => size.enabled)
          .map((size) => ({
            id: size.id,
          })),
      ),
    );

    const newStyles = editableStyles.flatMap((style) =>
      style.isNew ? [{ id: style.id }] : [],
    );
    const newColors = editableStyles.flatMap((style) =>
      style.colors.flatMap((color) => (color.isNew ? [{ id: color.id }] : [])),
    );
    const updateCollection: UpdateCollection = {
      name,
      slug: slug || null,
      external_id: externalId,
      pricing: pricingItems,
      new_styles: newStyles,
      new_colors: newColors,
      sizes,
    };
    if (!!bufferPromise) {
      bufferPromise.then((buffer) => {
        const uints = new Uint8Array(buffer);
        submitFormImpl({
          ...updateCollection,
          image: { base64: base64.fromByteArray(uints) },
        });
      });
    } else {
      submitFormImpl(updateCollection);
    }
  }

  return (
    <form
      onSubmit={(e) => {
        submitForm();
        e.preventDefault();
      }}
      className="space-y-8 mx-12 my-8"
    >
      <div className="space-y-8 divide-y divide-gray-200">
        <div>
          <div>
            <h2 className="text-3xl font-medium leading-6 text-gray-900">
              {i18nDbText(collection.name)}
            </h2>
            <p className="mt-2 text-sm text-gray-500">
              <Trans>Make your changes, then hit save when done.</Trans>
            </p>
          </div>

          <div className="mt-10 mb-5">
            <div>
              <h3 className="text-xl font-medium leading-6 text-gray-900">
                <Trans>Basic settings</Trans>
              </h3>
            </div>
          </div>
          <div className="mt-2 space-y-6">
            <div className="max-w-xl">
              <I18nStringInput
                id="name"
                placeholder={{
                  en: "Some collection name...",
                  sv: "En samling...",
                }}
                title={t`Name`}
                value={name}
                setValue={setName}
                required={true}
              />
            </div>
            <div className="max-w-xl">
              <ImageInput
                existing={
                  collection.image_url
                    ? {
                        src: cloudflareImageUrl(collection.image_url, "medium"),
                        alt: i18nDbText(collection.name),
                      }
                    : null
                }
                label={t`Cover photo`}
                maxSizeMegaBytes={10}
                description={t`It is recommended to give the image a 10:7 ratio and a minimum width and height of 1070 by 750 pixels.`}
                newImage={newImage}
                setNewImage={setNewImage}
                accept={["image/*"]}
              />
            </div>
            <div>
              <CollectionPricingGroupsEditor
                pricingItems={pricingItems}
                setPricingItems={setPricingItems}
                allPriceLists={allPriceLists}
              />
            </div>
            <div>
              <CollectionItemsEditor
                allNestedStylesMap={nestedStylesMap}
                editableStyles={editableStyles}
                setEditableStyles={setEditableStyles}
              />
            </div>
          </div>
          <div className="mt-10">
            <div>
              <h3 className="text-xl font-medium leading-6 text-gray-900">
                <Trans>Advanced settings</Trans>
              </h3>
            </div>
          </div>
          <div className="mt-2 max-w-xl space-y-4">
            <div>
              <Input
                id="slug"
                placeholder={t`Some slug value...`}
                title={t`Slug`}
                value={slug}
                setValue={setSlug}
                required={false}
              />
            </div>
            <div>
              <Input
                id="externalId"
                placeholder={t`Some external ID value...`}
                title={t`External ID`}
                value={externalId || ""}
                setValue={(value) =>
                  setExternalId(
                    (value || undefined) as string | null | undefined,
                  )
                }
                required={false}
              />
            </div>
          </div>
        </div>
      </div>
      <div className="py-5 sticky bottom-0 bg-white">
        <div className="flex justify-start items-center">
          <LocaleLink
            to="/app/admin"
            type="button"
            className="h-10 rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <Trans>Cancel</Trans>
          </LocaleLink>
          <button
            type="submit"
            className="h-10 ml-3 inline-flex justify-center items-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            disabled={submitting}
          >
            {submitting ? (
              <span className="mr-2">
                <LoadingIndicator textColor="gray" fillColor="gray" size={6} />
              </span>
            ) : (
              " "
            )}
            <span>
              {submitting ? <Trans>Saving...</Trans> : <Trans>Save</Trans>}
            </span>
          </button>
        </div>
      </div>
    </form>
  );
}

type DateGroupedCollectionPricing = Map<string, CollectionPricing[]>;

function CollectionPricingGroupsEditor({
  pricingItems,
  setPricingItems,
  allPriceLists,
}: {
  pricingItems: CollectionPricing[];
  setPricingItems: Dispatch<SetStateAction<CollectionPricing[]>>;
  allPriceLists: PriceListSummary[];
}) {
  const availablePriceLists = useMemo(() => {
    const addedPriceLists = pricingItems.map((p) => p.list);
    const addedPriceListIds = addedPriceLists.map((pl) => pl.id);
    return allPriceLists.filter((pl) => !addedPriceListIds.includes(pl.id));
  }, [pricingItems, allPriceLists]);

  function addPricingItems(newPricingItems: CollectionPricing[]) {
    const newListIds = newPricingItems.map((p) => p.list.id);
    const oldPricingItems = pricingItems.filter(
      (p) => !newListIds.includes(p.list.id),
    );
    setPricingItems([...oldPricingItems, ...newPricingItems]);
  }

  const dates = useMemo(
    () =>
      sortBy(pricingItems, "date").reduce((mapped, pricingItem) => {
        if (!mapped.has(pricingItem.date)) {
          mapped.set(pricingItem.date, []);
        }
        const datePricing = mapped.get(pricingItem.date);
        datePricing?.push(pricingItem);
        return mapped;
      }, new Map() as DateGroupedCollectionPricing),
    [pricingItems],
  );

  const [
    openNewCollectionPricingGroupModal,
    setNewCollectionPricingGroupModalOpen,
  ] = useState(false);
  return (
    <div>
      <h2 className="text-md font-medium leading-6 text-gray-900">
        <Trans>Pricing dates</Trans>
      </h2>
      <p className="mt-1 text-sm text-gray-500">
        <Trans>
          These dates decide which item prices should be picked up for each
          price list. Each item price is associated with a start date and end
          date, so the date you choose has to fall within those.
        </Trans>
      </p>
      {Array.from(dates.entries()).map(([date, groupPricingItems]) => (
        <div key={date} className="my-4">
          <CollectionPricingGroup
            date={date}
            groupPricingItems={groupPricingItems}
            setAllPricingItems={setPricingItems}
            availablePriceLists={availablePriceLists}
          />
        </div>
      ))}

      <NewCollectionPricingGroupModal
        open={openNewCollectionPricingGroupModal}
        setOpen={setNewCollectionPricingGroupModalOpen}
        addPricingItems={addPricingItems}
        availablePriceLists={availablePriceLists}
      />

      <button
        onClick={() => {
          setNewCollectionPricingGroupModalOpen(true);
        }}
        type="button"
        className="my-4 inline-flex items-center justify-center rounded-md border border-transparent bg-gray-400 px-3 py-2 text-xs font-medium text-white shadow-sm hover:bg-gray-500 focus:outline-none focus:ring-2 focus:ring-gray-300 focus:ring-offset-2 sm:w-auto"
      >
        <Trans>New date</Trans>
      </button>
    </div>
  );
}

function CollectionPricingGroup({
  date,
  groupPricingItems,
  setAllPricingItems,
  availablePriceLists,
}: {
  date: string;
  groupPricingItems: CollectionPricing[];
  setAllPricingItems: Dispatch<SetStateAction<CollectionPricing[]>>;
  availablePriceLists: PriceListSummary[];
}) {
  const { i18n } = useLocalize();
  const [priceLists, setPriceLists] = useState(() =>
    groupPricingItems.map((cp) => cp.list),
  );
  useEffect(() => {
    setAllPricingItems((prevCollectionPricing) => [
      ...prevCollectionPricing.filter((cp) => cp.date !== date),
      ...priceLists.map((list) => ({ date, list })),
    ]);
  }, [date, priceLists, setAllPricingItems]);
  return (
    <>
      <h3 className="text-sm font-medium leading-6 text-gray-900">
        {i18n.date(date, { dateStyle: "long" })}
      </h3>
      <AssociatedPriceListsEditor
        priceLists={priceLists}
        setPriceLists={setPriceLists}
        availablePriceLists={availablePriceLists}
      />
    </>
  );
}
function CollectionItemsEditor({
  allNestedStylesMap,
  editableStyles,
  setEditableStyles,
}: {
  allNestedStylesMap: Map<number, NestedStyleSummary>;
  editableStyles: EditableStyle[];
  setEditableStyles: Dispatch<SetStateAction<EditableStyle[]>>;
}) {
  const checkbox = useRef<HTMLInputElement>(null);
  const [checked, setChecked] = useState(false);
  const [indeterminate, setIndeterminate] = useState(false);
  const [selectedItems, setSelectedItems] = useState([] as EditableStyle[]);
  const [addItemsModalOpen, setAddItemsModalOpen] = useState(false);
  const itemFilterChoices = useItemFilterChoices().unwrapOr(null);

  function setStylesEnabled(styles: EditableStyle[], enabled: boolean) {
    setEditableStyles([
      ...editableStyles.map((otherStyle) =>
        styles.includes(otherStyle)
          ? {
              ...otherStyle,
              colors: otherStyle.colors.map((color) => ({
                ...color,
                sizes: color.sizes.map((size) => ({ ...size, enabled })),
              })),
            }
          : otherStyle,
      ),
    ]);
  }

  function setStyleIsNew(style: EditableStyle, isNew: boolean) {
    setEditableStyles(
      editableStyles.map((otherStyle) =>
        otherStyle.id === style.id
          ? {
              ...style,
              isNew,
            }
          : otherStyle,
      ),
    );
  }

  function setColorEnabled(color: EditableColor, enabled: boolean) {
    setEditableStyles([
      ...editableStyles.map((style) => ({
        ...style,
        colors: style.colors.map((otherColor) =>
          otherColor.id === color.id
            ? {
                ...color,
                sizes: color.sizes.map((size) => ({ ...size, enabled })),
              }
            : otherColor,
        ),
      })),
    ]);
  }

  function setColorIsNew(color: EditableColor, isNew: boolean) {
    setEditableStyles([
      ...editableStyles.map((style) => ({
        ...style,
        colors: style.colors.map((otherColor) =>
          otherColor.id === color.id ? { ...color, isNew } : otherColor,
        ),
      })),
    ]);
  }

  function setSizeEnabled(size: EditableSize, enabled: boolean) {
    setEditableStyles([
      ...editableStyles.map((style) => ({
        ...style,
        colors: style.colors.map((color) => ({
          ...color,
          sizes: color.sizes.map((otherSize) =>
            otherSize.id === size.id ? { ...size, enabled } : otherSize,
          ),
        })),
      })),
    ]);
  }

  useLayoutEffect(() => {
    const isIndeterminate =
      selectedItems.length > 0 && selectedItems.length < editableStyles.length;
    setChecked(selectedItems.length === editableStyles.length);
    setIndeterminate(isIndeterminate);
    if (!!checkbox.current) {
      checkbox.current.indeterminate = isIndeterminate;
    }
  }, [selectedItems, editableStyles.length]);

  function toggleAll() {
    setSelectedItems(checked || indeterminate ? [] : editableStyles);
    setChecked(!checked && !indeterminate);
    setIndeterminate(false);
  }

  return (
    <div>
      {itemFilterChoices && (
        <AddCollectionItemsModal
          open={addItemsModalOpen}
          setOpen={setAddItemsModalOpen}
          allNestedStylesMap={allNestedStylesMap}
          setEditableStyles={setEditableStyles}
          itemFilterChoices={itemFilterChoices}
        />
      )}
      <div className="sm:flex sm:items-center">
        <div className="sm:flex-auto">
          <h1 className="text-xl font-semibold text-gray-900">
            <Trans>Items</Trans>
          </h1>
          <p className="mt-2 text-sm text-gray-700">
            <Trans>
              Styles, colors and sizes to associate with this collection.
            </Trans>
          </p>
        </div>
        <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
          <button
            type="button"
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
            onClick={() => setAddItemsModalOpen(true)}
          >
            <Trans>Add items</Trans>
          </button>
        </div>
      </div>
      <div className="mt-8 flex flex-col">
        <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
          <div className="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
            <div className="relative overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
              {selectedItems.length > 0 && (
                <div className="absolute top-0 left-12 flex h-12 items-center space-x-3 bg-gray-50 sm:left-16">
                  <button
                    type="button"
                    className="inline-flex items-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-30"
                    onClick={() => {
                      setStylesEnabled(selectedItems, false);
                      setSelectedItems([]);
                    }}
                  >
                    <Plural
                      value={selectedItems.length}
                      one="Disable # style"
                      other="Disable # styles"
                    />
                  </button>
                  <button
                    type="button"
                    className="inline-flex items-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-30"
                    onClick={() => {
                      setStylesEnabled(selectedItems, true);
                      setSelectedItems([]);
                    }}
                  >
                    <Plural
                      value={selectedItems.length}
                      one="Enable # style"
                      other="Enable # styles"
                    />
                  </button>
                  <button
                    type="button"
                    className="inline-flex items-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-30"
                    onClick={() => {
                      setSelectedItems([]);
                    }}
                  >
                    <Trans>Clear selection</Trans>
                  </button>
                </div>
              )}
              <table className="min-w-full table-fixed divide-y divide-gray-300">
                <thead className="bg-gray-50">
                  <tr>
                    <th
                      scope="col"
                      className="relative w-12 px-6 sm:w-16 sm:px-8"
                    >
                      <input
                        type="checkbox"
                        className="absolute left-4 top-1/2 -mt-2 h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500 sm:left-6"
                        ref={checkbox}
                        checked={checked}
                        onChange={toggleAll}
                      />
                    </th>
                    <th
                      scope="col"
                      className="min-w-[12rem] py-3.5 pr-3 text-left text-sm font-semibold text-gray-900"
                    >
                      <Trans>Style</Trans>
                    </th>
                    <th
                      scope="col"
                      className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                    >
                      <Trans>Colors / Sizes</Trans>
                    </th>
                    <th
                      scope="col"
                      className="relative py-3.5 pl-3 pr-4 sm:pr-6"
                    >
                      <span className="sr-only">Edit</span>
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200 bg-white">
                  {editableStyles.map((editableStyle) => (
                    <EditableCollectionItemRow
                      key={editableStyle.id}
                      style={editableStyle}
                      selected={selectedItems.includes(editableStyle)}
                      onSelectChange={(checked) =>
                        setSelectedItems(
                          checked
                            ? [...selectedItems, editableStyle]
                            : selectedItems.filter((p) => p !== editableStyle),
                        )
                      }
                      setSizeEnabled={setSizeEnabled}
                      setColorEnabled={setColorEnabled}
                      setColorIsNew={setColorIsNew}
                      setStyleIsNew={setStyleIsNew}
                    />
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function EditableCollectionItemRow({
  style,
  selected,
  onSelectChange,
  setSizeEnabled,
  setColorEnabled,
  setColorIsNew,
  setStyleIsNew,
}: {
  style: EditableStyle;
  selected: boolean;
  onSelectChange: (checked: boolean) => void;
  setSizeEnabled: (size: EditableSize, enabled: boolean) => void;
  setColorEnabled: (color: EditableColor, enabled: boolean) => void;
  setColorIsNew: (color: EditableColor, isNew: boolean) => void;
  setStyleIsNew: (style: EditableStyle, isNew: boolean) => void;
}) {
  const { i18nDbText } = useLocalize();
  return (
    <tr key={style.id} className={selected ? "bg-gray-50" : undefined}>
      <td className="relative w-12 px-6 sm:w-16 sm:px-8">
        {selected && (
          <div className="absolute inset-y-0 left-0 w-0.5 bg-indigo-600" />
        )}
        <input
          type="checkbox"
          className="absolute left-4 top-1/2 -mt-2 h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500 sm:left-6"
          value={style.id}
          checked={selected}
          onChange={(e) => onSelectChange(e.target.checked)}
        />
      </td>
      <td
        className={classNames(
          "whitespace-nowrap py-4 pr-3 text-sm font-medium",
          selected ? "text-indigo-600" : "text-gray-900",
        )}
      >
        <div className="mx-4 flex items-center">
          <div>
            <div className="font-medium text-gray-900">
              {i18nDbText(style.name)}
            </div>
            <div className="text-gray-500">{style.number}</div>
          </div>
          <div className="mx-4">
            <NewIconSelectBox
              checked={style.isNew}
              onChange={(evt) => setStyleIsNew(style, evt.target.checked)}
            />
            <span className="mx-1" />
            {t`New`}
          </div>
        </div>
      </td>
      <td className="whitespace-nowrap py-2 text-sm text-gray-500">
        {style.colors.map((color) => (
          <ColorCell
            key={color.id}
            color={color}
            setSizeEnabled={setSizeEnabled}
            setColorEnabled={setColorEnabled}
            setColorIsNew={setColorIsNew}
          />
        ))}
      </td>
    </tr>
  );
}

function ColorCell({
  color,
  setColorEnabled,
  setSizeEnabled,
  setColorIsNew,
}: {
  color: EditableColor;
  setSizeEnabled: (size: EditableSize, enabled: boolean) => void;
  setColorEnabled: (color: EditableColor, enabled: boolean) => void;
  setColorIsNew: (color: EditableColor, isNew: boolean) => void;
}) {
  const { i18nDbText } = useLocalize();
  const anyEnabled = color.sizes.find((size) => size.enabled) !== undefined;
  return (
    <div key={color.id}>
      <div className="mx-4 flex items-center">
        <div className="h-12 w-12 inline-block">
          {color.primaryImage ? (
            <img
              className="h-12 max-w-12"
              loading="lazy"
              src={cloudflareImageUrl(color.primaryImage.url, "thumbnail")}
              alt={""}
            />
          ) : (
            ""
          )}
        </div>
        <div className="inline-block font-medium text-gray-900 mx-2">
          {i18nDbText(color.name)}
        </div>
        <div className="inline-block text-gray-500 mx-2">{color.number}</div>
        <div className="inline-block text-gray-500 mx-2">
          {color.sizes.map((size) => (
            <span className="mx-2" key={size.id}>
              <input
                type="checkbox"
                className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                checked={size.enabled}
                onChange={(e) => setSizeEnabled(size, e.target.checked)}
              />{" "}
              {size.number}
            </span>
          ))}
        </div>
        <div className="inline-block text-gray-500 mx-2">
          <button
            type="button"
            className="text-indigo-600 hover:text-indigo-900 text-xs"
            onClick={(evt) => {
              evt.stopPropagation();
              evt.preventDefault();
              setColorEnabled(color, !anyEnabled);
            }}
          >
            {anyEnabled ? t`Disable all` : t`Enable all`}
            <span className="sr-only">, {i18nDbText(color.name)}</span>
          </button>
        </div>
        <div className="inline-block text-gray-500 mx-2 text-sm">
          {anyEnabled && (
            <>
              <NewIconSelectBox
                checked={color.isNew}
                onChange={(evt) => setColorIsNew(color, evt.target.checked)}
              />
              <span className="mx-1" />
              {t`New`}
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function NewIconSelectBox({
  checked,
  onChange,
}: {
  checked: boolean;
  onChange: ChangeEventHandler<HTMLInputElement>;
}) {
  return (
    <input
      type="checkbox"
      className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
      checked={checked}
      onChange={onChange}
    />
  );
}
