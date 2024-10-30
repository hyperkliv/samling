import { useState } from "react";
import { Disclosure, RadioGroup, Tab } from "@headlessui/react";
import { MinusIcon, PlusIcon } from "@heroicons/react/24/outline";
import { classNames } from "../../utils";
import Breadcrumbs from "../../components/nav/Breadcrumbs";
import { useLocalize } from "../../i18n";
import * as api from "../../types/api";
import { t, Trans } from "@lingui/macro";
import { useCollectionItem, useCollectionSummary } from "../../api";
import { useParams } from "react-router-dom";
import Loading from "../../components/Loading";
import ApiError from "./errors/ApiError";
import { cloudflareImageUrl } from "../../images";
import { useTitle } from "../../hooks";

export default function Style() {
  const { collectionSlug, styleSlug, colorSlug } = useParams();
  const { i18nDbText } = useLocalize();
  const [collectionResult] = useCollectionSummary({ slug: collectionSlug });
  const itemResult = useCollectionItem(
    collectionSlug || null,
    styleSlug || null,
  );

  const collection = collectionResult.unwrapOr(null);
  const item = itemResult.unwrapOr(null);

  const activeColor = item?.style.colors.find(
    (color) => color.slug === colorSlug,
  );

  useTitle([i18nDbText(item?.style.name)]);

  if (collectionResult.isErr()) {
    return <ApiError error={collectionResult.unwrapErr()} />;
  }

  if (itemResult.isErr()) {
    return <ApiError error={itemResult.unwrapErr()} />;
  }

  if (collection === null || item === null) {
    return <Loading />;
  } else {
    return (
      <StyleInner
        collection={collection}
        item={item}
        activeColor={activeColor}
      />
    );
  }
}

interface StyleInnerProps {
  collection: api.CollectionSummary;
  item: api.CollectionItem;
  activeColor?: api.NestedColor;
}

type AttributeGroups = Map<number, api.AttributeSummary[]>;

function StyleInner({ collection, item, activeColor }: StyleInnerProps) {
  const style = item.style;
  const [selectedColor, setSelectedColor] = useState(
    activeColor || style.colors[0],
  );
  const { i18nLink, i18nDbText } = useLocalize();
  const groupedAttributes = style.attributes.reduce((groups, attr) => {
    if (!groups.has(attr.type.id)) {
      groups.set(attr.type.id, []);
    }
    const attributes = groups.get(attr.type.id);
    attributes?.push(attr);
    return groups;
  }, new Map() as AttributeGroups);

  return (
    <>
      <Breadcrumbs
        items={[
          { title: t`Collections`, to: i18nLink("/app"), current: false },
          {
            title: i18nDbText(collection.name),
            to: i18nLink(`/app/collections/${collection.slug}`),
            current: false,
          },
          {
            title: i18nDbText(style.name),
            to: i18nLink(`/app/collections/${collection.slug}/${style.slug}`),
            current: true,
          },
        ]}
      />
      <div className="mx-auto max-w-2xl py-16 px-4 sm:py-24 sm:px-6 lg:max-w-7xl lg:px-8">
        <div className="lg:grid lg:grid-cols-2 lg:items-start lg:gap-x-8">
          {/* Image gallery */}
          <Tab.Group as="div" className="flex flex-col-reverse">
            {/* Image selector */}
            <div className="mx-auto mt-6 hidden w-full max-w-2xl sm:block lg:max-w-none">
              <Tab.List className="grid grid-cols-4 gap-6">
                {selectedColor.images.map((image) => (
                  <Tab
                    key={image.id}
                    className="relative flex h-24 cursor-pointer items-center justify-center rounded-md bg-white text-sm font-medium uppercase text-gray-900 hover:bg-gray-50 focus:outline-none focus:ring focus:ring-opacity-50 focus:ring-offset-4"
                  >
                    {({ selected }) => (
                      <>
                        <span className="sr-only">
                          {i18nDbText(style.name)}{" "}
                          {i18nDbText(selectedColor.name)}
                        </span>
                        <span className="absolute inset-0 overflow-hidden rounded-md">
                          <img
                            src={cloudflareImageUrl(image.url, "thumbnail")}
                            alt=""
                            className="h-full w-full object-cover object-center"
                          />
                        </span>
                        <span
                          className={classNames(
                            selected ? "ring-indigo-500" : "ring-transparent",
                            "pointer-events-none absolute inset-0 rounded-md ring-2 ring-offset-2",
                          )}
                          aria-hidden="true"
                        />
                      </>
                    )}
                  </Tab>
                ))}
              </Tab.List>
            </div>

            <Tab.Panels className="aspect-w-1 aspect-h-1 w-full">
              {selectedColor.images.map((image) => (
                <Tab.Panel key={image.id}>
                  <img
                    src={cloudflareImageUrl(image.url, "medium")}
                    alt=""
                    className="h-full w-full object-cover object-center sm:rounded-lg"
                  />
                </Tab.Panel>
              ))}
            </Tab.Panels>
          </Tab.Group>

          {/* Product info */}
          <div className="mt-10 px-4 sm:mt-16 sm:px-0 lg:mt-0">
            <h1 className="text-3xl font-bold tracking-tight text-gray-900">
              {i18nDbText(style.name)}
            </h1>
            <p className="text-sm text-gray-700">
              {selectedColor?.number} / {i18nDbText(selectedColor?.name)}
            </p>

            <div className="mt-3">
              <h2 className="sr-only">
                <Trans>Product information</Trans>
              </h2>
              {/*  TODO: Make all prices visible
              <p className="text-3xl tracking-tight text-gray-900">
                {[style.prices[0].amount, style.prices[0].currency].join(" ")}
              </p>
              */}
            </div>

            {/* Reviews
            <div className="mt-3">
              <h3 className="sr-only">Reviews</h3>
              <div className="flex items-center">
                <div className="flex items-center">
                  {[0, 1, 2, 3, 4].map((rating) => (
                    <StarIcon
                      key={rating}
                      className={classNames(
                        product.rating > rating ? 'text-indigo-500' : 'text-gray-300',
                        'h-5 w-5 flex-shrink-0'
                      )}
                      aria-hidden="true"
                    />
                  ))}
                </div>
                <p className="sr-only">{product.rating} out of 5 stars</p>
              </div>
            </div>
            */}

            <div className="mt-6">
              <h3 className="sr-only">
                <Trans>Description</Trans>
              </h3>

              <div
                className="space-y-6 text-base text-gray-700"
                dangerouslySetInnerHTML={{
                  __html: i18nDbText(style.description),
                }}
              />
            </div>

            {/* <form className="mt-6"> */}
            {/* Colors */}
            <div>
              <h3 className="text-sm text-gray-600 mt-4">
                <Trans>Color</Trans>
              </h3>

              <RadioGroup
                value={selectedColor}
                onChange={setSelectedColor}
                className="mt-2"
              >
                <RadioGroup.Label className="sr-only">
                  {" "}
                  <Trans>Choose a color</Trans>{" "}
                </RadioGroup.Label>
                <span className="flex flex-wrap">
                  {style.colors.map((color) => (
                    <RadioGroup.Option
                      key={color.id}
                      value={color}
                      className={({ active, checked }) =>
                        classNames(
                          "ring-indigo-500",
                          active && checked ? "ring" : "",
                          "rounded-md cursor-pointer m-2",
                        )
                      }
                    >
                      <RadioGroup.Label as="span" className="sr-only">
                        {i18nDbText(color.name)}
                      </RadioGroup.Label>
                      {color.images.length > 0 ? (
                        <img
                          aria-hidden="true"
                          className="h-12 max-w-none"
                          src={cloudflareImageUrl(
                            color.images[0].url,
                            "thumbnail",
                          )}
                          alt=""
                        />
                      ) : (
                        <span className="inline-block h-12 w-12 bg-gray-100 rounded-md"></span>
                      )}
                    </RadioGroup.Option>
                  ))}
                </span>
              </RadioGroup>
            </div>

            <section aria-labelledby="details-heading" className="mt-12">
              <h2 id="details-heading" className="sr-only">
                <Trans>Additional details</Trans>
              </h2>

              <div
                className={classNames(
                  groupedAttributes.size > 0
                    ? "divide-y divide-gray-200 border-t"
                    : "",
                )}
              >
                {Array.from(groupedAttributes.entries()).map(
                  ([_attributeType, attributes]) => (
                    <Disclosure as="div" key={attributes[0].type.id}>
                      {({ open }) => (
                        <>
                          <h3>
                            <Disclosure.Button className="group relative flex w-full items-center justify-between py-6 text-left">
                              <span
                                className={classNames(
                                  open ? "text-gray-900" : "text-gray-900",
                                  "text-sm font-medium",
                                )}
                              >
                                {i18nDbText(attributes[0].type.name)}
                                <span className="inline-flex items-center rounded-full bg-gray-100 ml-2 px-2.5 py-1 text-xs font-medium text-gray-800">
                                  {attributes.length}
                                </span>
                              </span>
                              <span className="ml-6 flex items-center">
                                {open ? (
                                  <MinusIcon
                                    className="block h-6 w-6 text-indigo-400 group-hover:text-indigo-500"
                                    aria-hidden="true"
                                  />
                                ) : (
                                  <PlusIcon
                                    className="block h-6 w-6 text-gray-400 group-hover:text-gray-500"
                                    aria-hidden="true"
                                  />
                                )}
                              </span>
                            </Disclosure.Button>
                          </h3>
                          <Disclosure.Panel
                            as="div"
                            className="prose prose-sm pb-6"
                          >
                            <ul>
                              {attributes.map((attr) => (
                                <li key={attr.id}>
                                  <>
                                    <b>{i18nDbText(attr.title)}</b>{" "}
                                    {i18nDbText(attr.description)}
                                  </>
                                </li>
                              ))}
                            </ul>
                          </Disclosure.Panel>
                        </>
                      )}
                    </Disclosure>
                  ),
                )}
              </div>
            </section>
          </div>
        </div>
      </div>
    </>
  );
}
