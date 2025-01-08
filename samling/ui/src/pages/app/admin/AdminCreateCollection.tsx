import { useLocalize } from "../../../i18n";
import base64 from "base64-js";
import { t, Trans } from "@lingui/macro";
import api from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import { match } from "oxide.ts";
import { useAppDispatch, useAppSelector } from "../../../state/hooks";
import { CreateCollection } from "../../../types/api";
import { useEffect, useState } from "react";
import LocaleLink from "../../../components/LocaleLink";
import { apiErrorMessage, userMessage } from "../../../state/slices/user";
import { UserMessageLevel } from "../../../types/messages";
import Input from "../../../components/forms/Input";
import { useTitle } from "../../../hooks";
import I18nStringInput from "../../../components/forms/I18nInput";
import ImageInput from "../../../components/forms/ImageInput";
import LoadingIndicator from "../../../components/LoadingIndicator";
import { useNavigate } from "react-router-dom";

// TODO: Merge this into AdminEditCollection. Too much code duplication!
export default function AdminCreateCollection() {
  const { i18nLink } = useLocalize();

  useTitle([t`Admin`, t`New collection`]);

  const { token, activeOrganization } = useAppSelector((state) => state.user);

  return (
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
            title: t`New`,
            to: i18nLink(`/app/admin/collections/new`),
            current: true,
          },
        ]}
      />
      <CollectionCreateForm
        apiToken={token as string}
        organizationId={activeOrganization?.organization.id as number}
      />
    </>
  );
}

interface CollectionCreateFormProps {
  apiToken: string;
  organizationId: number;
}

function CollectionCreateForm({
  apiToken,
  organizationId,
}: CollectionCreateFormProps) {
  const { i18nLink, i18nDbText } = useLocalize();
  const dispatch = useAppDispatch();

  const [submitting, setSubmitting] = useState(false);
  const [name, setName] = useState({});
  const [createdId, setCreatedId] = useState(null as number | null);
  const [slug, setSlug] = useState("");
  const [newImage, setNewImage] = useState(null as File | null);
  const [externalId, setExternalId] = useState(null as string | null);
  const navigate = useNavigate();

  useEffect(() => {
    if (!!createdId) {
      navigate(i18nLink(`/app/admin/collections/${createdId}`));
      setCreatedId(null);
    }
  }, [createdId, i18nLink, navigate]);

  function submitFormImpl(createCollection: CreateCollection) {
    api
      .createCollection(apiToken, organizationId, createCollection)
      .then((res) => {
        match(res, {
          Ok: (collection) => {
            dispatch(
              userMessage({
                level: UserMessageLevel.Info,
                title: t`Success!`,
                body: t`The collection "${i18nDbText(
                  collection.name,
                )}" was successfully created.`,
                opts: { dismissAfter: 3000 },
              }),
            );
            setNewImage(null); // To ensure that new image is not uploaded again
            setCreatedId(collection.id);
          },
          Err: (error) => {
            dispatch(apiErrorMessage(error));
          },
        });
      })
      .finally(() => setSubmitting(false));
  }

  function submitForm() {
    setSubmitting(true);
    const bufferPromise = newImage?.arrayBuffer();
    let createCollection: CreateCollection = {
      name,
      acronym: {}, // TODO: Add support for acronym
      slug: slug || null,
      external_id: externalId,
      new_styles: [], // TODO: Add support for new_styles
      new_colors: [], // TODO: Add support for new_colors
      pricing: [], // TODO: Add support for pricing
    };
    if (!!bufferPromise) {
      bufferPromise.then((buffer) => {
        const uints = new Uint8Array(buffer);
        submitFormImpl({
          ...createCollection,
          image: { base64: base64.fromByteArray(uints) },
        });
      });
    } else {
      submitFormImpl(createCollection);
    }
  }

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        submitForm();
      }}
      className="space-y-8 mx-12 my-8"
    >
      <div className="space-y-8 divide-y divide-gray-200">
        <div>
          <div>
            <h2 className="text-3xl font-medium leading-6 text-gray-900">
              {i18nDbText(name) || t`New collection`}
            </h2>
            <p className="mt-1 text-sm text-gray-500">
              <Trans>Make your changes, then hit save when done.</Trans>
            </p>
          </div>

          <div className="mt-10">
            <div>
              <h3 className="text-lg font-medium leading-6 text-gray-900">
                <Trans>Basic settings</Trans>
              </h3>
            </div>
          </div>
          <div className="mt-2 grid grid-cols-1 gap-y-2 gap-x-4 sm:grid-cols-6">
            <div className="sm:col-span-4">
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
            <div className="sm:col-span-4">
              <ImageInput
                existing={null}
                label={t`Cover photo`}
                maxSizeMegaBytes={10}
                description={t`It is recommended to give the image a 10:7 ratio and a minimum width and height of 1070 by 750 pixels.`}
                newImage={newImage}
                setNewImage={setNewImage}
                accept={["image/*"]}
              />
            </div>
          </div>
          <div className="mt-10">
            <div>
              <h3 className="text-lg font-medium leading-6 text-gray-900">
                <Trans>Advanced settings</Trans>
              </h3>
            </div>
          </div>
          <div className="mt-2 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div className="sm:col-span-4">
              <Input
                id="slug"
                placeholder={t`Some slug value...`}
                title={t`Slug`}
                value={slug}
                setValue={setSlug}
                required={false}
              />
            </div>
            <div className="sm:col-span-4">
              <Input
                id="externalId"
                placeholder={t`Some external ID value...`}
                title={t`External ID`}
                value={externalId || ""}
                setValue={(value) =>
                  setExternalId((value || null) as string | null)
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
