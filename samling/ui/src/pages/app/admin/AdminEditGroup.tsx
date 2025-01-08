import { useLocalize } from "../../../i18n";
import { t, Trans } from "@lingui/macro";
import api, {
  useCollectionList,
  useGroup,
  usePricelists,
  useUserList,
} from "../../../api";
import Breadcrumbs from "../../../components/nav/Breadcrumbs";
import { match } from "oxide.ts";
import ApiError from "../errors/ApiError";
import Loading from "../../../components/Loading";
import { useParams } from "react-router-dom";
import { useAppDispatch, useAppSelector } from "../../../state/hooks";
import {
  CollectionSummary,
  Group,
  PriceListSummary,
  UpdateGroup,
  User,
  UserSummary,
} from "../../../types/api";
import {
  Dispatch,
  DispatchWithoutAction,
  SetStateAction,
  useState,
} from "react";
import LocaleLink from "../../../components/LocaleLink";
import { apiErrorMessage, userMessage } from "../../../state/slices/user";
import { UserMessageLevel } from "../../../types/messages";
import { cloudflareImageUrl } from "../../../images";
import { formatRelative, parseISO } from "date-fns";
import SearchableFilter, {
  SearchableFilterItem,
} from "../../../components/filters/SearchableFilter";
import Input from "../../../components/forms/Input";
import { useTitle } from "../../../hooks";
import AssociatedPriceListsEditor from "../../../components/forms/AssociatedPriceListsEditor";
import { Active } from "../../../roles";

export default function AdminEditGroup() {
  const { groupSlug } = useParams();
  const { i18nLink } = useLocalize();
  const [groupResult, refreshGroup] = useGroup(groupSlug as string);
  const allPriceLists = usePricelists().unwrapOr(null);
  const group = groupResult.unwrapOr(null);

  useTitle([t`Admin`, group?.name]);

  const { token, activeOrganization } = useAppSelector((state) => state.user);

  return match(groupResult, {
    Err: (error) => <ApiError error={error} />,
    Ok: () =>
      group === null || allPriceLists === null ? (
        <Loading />
      ) : (
        <>
          <Breadcrumbs
            items={[
              { title: t`Admin`, to: i18nLink("/app/admin"), current: false },
              {
                title: t`Groups`,
                to: i18nLink("/app/admin/groups"),
                current: false,
              },
              {
                title: t`Edit group`,
                to: i18nLink(`/app/admin/groups/${groupSlug}`),
                current: true,
              },
            ]}
          />
          <GroupEditForm
            group={group}
            allPriceLists={allPriceLists}
            onSuccess={refreshGroup}
            apiToken={token as string}
            organizationId={activeOrganization?.organization.id as number}
          />
        </>
      ),
  });
}

interface GroupEditFormProps {
  group: Group;
  apiToken: string;
  allPriceLists: PriceListSummary[];
  organizationId: number;
  onSuccess: () => void;
}

function GroupEditForm({
  apiToken,
  organizationId,
  group,
  allPriceLists,
  onSuccess,
}: GroupEditFormProps) {
  const dispatch = useAppDispatch();
  const isSuperuserGroup = group.external_id === "superusers";

  const [name, setName] = useState(group.name);
  const [description, setDescription] = useState(group.description);
  const [slug, setSlug] = useState(group.slug);
  const [externalId, setExternalId] = useState(group.external_id);

  const [users, setUsers] = useState(group.users);
  const [priceLists, setPriceLists] = useState(group.price_lists);
  const [collections, setCollections] = useState(group.collections);

  const [allUsers] = useUserList();
  const [allCollections] = useCollectionList();

  function submitForm() {
    let updateGroup = {
      name,
      description,
      slug,
      external_id: externalId,
      users: users.map((user) => ({
        id: user.id,
      })),
      price_lists: priceLists.map((priceList) => ({
        id: priceList.id,
      })),
      collections: collections.map((collection) => ({
        id: collection.id,
      })),
    } as UpdateGroup;
    api
      .updateGroup(apiToken, organizationId, group.id, updateGroup)
      .then((res) => {
        match(res, {
          Ok: (group) => {
            dispatch(
              userMessage({
                level: UserMessageLevel.Info,
                title: t`Success!`,
                body: t`The group "${group.name}" was successfully updated.`,
                opts: { dismissAfter: 3000 },
              }),
            );
            onSuccess();
          },
          Err: (error) => {
            dispatch(apiErrorMessage(error));
          },
        });
      });
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
              {group.name}
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
          <div className="mt-2 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
            <div className="sm:col-span-4">
              <Input
                id="name"
                placeholder={t`Some group name...`}
                title={t`Name`}
                value={name || ""}
                setValue={setName}
                required={true}
                disabled={isSuperuserGroup}
              />
            </div>
            <div className="sm:col-span-4">
              <Input
                id="description"
                placeholder={t`Some group description...`}
                title={t`Description`}
                value={description || ""}
                setValue={setDescription}
                required={false}
                disabled={isSuperuserGroup}
              />
            </div>
            <div className="sm:col-span-4">
              <Input
                id="slug"
                placeholder={t`Some slug value...`}
                title={t`Slug`}
                value={slug || ""}
                setValue={setSlug}
                required={true}
                disabled={isSuperuserGroup}
              />
            </div>
            <div className="sm:col-span-4">
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
                disabled={isSuperuserGroup}
              />
            </div>
          </div>

          <div className="max-w-xl">
            <UsersEditTable
              users={users}
              setUsers={setUsers}
              allUsers={allUsers.unwrapOr([]) || []}
            />
          </div>

          <div className="mt-10">
            <div>
              <h3 className="text-lg font-medium leading-6 text-gray-900">
                <Trans>Price lists</Trans>
              </h3>
              <p className="mt-1 text-sm text-gray-500">
                <Trans>
                  Which price lists users of this group will have access to.
                </Trans>
              </p>
            </div>
            <div className="max-w-md">
              <AssociatedPriceListsEditor
                priceLists={priceLists}
                setPriceLists={setPriceLists}
                availablePriceLists={allPriceLists}
              />
            </div>
          </div>

          <div className="max-w-xl">
            <CollectionsEditTable
              collections={collections}
              setCollections={setCollections}
              allCollections={allCollections.unwrapOr([]) || []}
              canAdd={!isSuperuserGroup}
              canRemove={!isSuperuserGroup}
            />
          </div>
        </div>
      </div>
      <div className="py-5 sticky bottom-0 bg-white">
        <div className="flex justify-start">
          <LocaleLink
            to="/app/admin"
            type="button"
            className="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <Trans>Cancel</Trans>
          </LocaleLink>
          <button
            type="submit"
            className="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <Trans>Save</Trans>
          </button>
        </div>
      </div>
    </form>
  );
}

interface UsersEditTableProps {
  users: UserSummary[];
  setUsers: Dispatch<SetStateAction<UserSummary[]>>;
  allUsers: User[];
}

function UsersEditTable({ users, setUsers, allUsers }: UsersEditTableProps) {
  const userIds = users.map((user) => user.id);
  const fullUsers = allUsers.filter((user) => userIds.includes(user.id));
  const searchableItems = allUsers
    .filter((user) => !userIds.includes(user.id))
    .map(
      (user) =>
        ({
          id: user.id,
          title: user.name,
          subtitle: user.email,
          imageUrl: user.profile_image
            ? cloudflareImageUrl(user.profile_image, "thumbnail")
            : null,
        } as SearchableFilterItem),
    );

  function onNewItem(newItem: SearchableFilterItem) {
    const newUser = allUsers.find(
      (user) => user.id === newItem.id,
    ) as UserSummary;
    setUsers([...users, newUser]);
  }

  function remove(toRemove: UserSummary) {
    setUsers(users.filter((user) => user.id !== toRemove.id));
  }

  return (
    <div className="mt-10">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900">
          <Trans>Users</Trans>
        </h3>
        <p className="mt-1 text-sm text-gray-500">
          <Trans>The users that belong to this group.</Trans>
        </p>
      </div>
      {users.length === 0 ? (
        <div className="text-sm text-gray-500 my-2">
          <Trans>List is empty.</Trans>
        </div>
      ) : (
        <table className="min-w-full divide-y divide-gray-300">
          <thead className="bg-gray-50">
            <tr>
              <th
                scope="col"
                className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6"
              >
                <Trans>Name</Trans>
              </th>
              <th
                scope="col"
                className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
              >
                <Trans>Status</Trans>
              </th>
              <th
                scope="col"
                className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
              >
                <Trans>Last sign in</Trans>
              </th>
              <th scope="col" className="relative py-3.5 pl-3 pr-4 sm:pr-6">
                <span className="sr-only">Edit</span>
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white">
            {fullUsers.map((user) => (
              <UserTableRow
                key={user.id}
                user={user}
                removeUser={() => remove(user)}
              />
            ))}
          </tbody>
        </table>
      )}
      <SearchableFilter
        title={t`Add user`}
        items={searchableItems}
        onNewItem={onNewItem}
      />
    </div>
  );
}

interface UserTableRowProps {
  user: User;
  removeUser: DispatchWithoutAction;
}

function UserTableRow({ user, removeUser }: UserTableRowProps) {
  const { activeOrganization } = useAppSelector((state) => state.user);
  const imageUrl = user.profile_image
    ? cloudflareImageUrl(user.profile_image, "thumbnail")
    : "";
  const userRoles =
    user.organizations.find(
      (org) => org.organization.id === activeOrganization?.organization.id,
    )?.roles || [];
  return (
    <tr>
      <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
        <div className="flex items-center">
          <div className="h-10 w-10 flex-shrink-0">
            {imageUrl ? (
              <img
                className="h-10 max-w-10 rounded-full"
                src={imageUrl}
                loading="lazy"
                alt=""
              />
            ) : (
              ""
            )}
          </div>
          <div className="ml-4">
            <div className="font-medium text-gray-900">{user.name}</div>
            <div className="text-gray-500">{user.email}</div>
          </div>
        </div>
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        <span className="inline-flex rounded-full bg-indigo-100 px-2 text-xs font-semibold leading-5 text-indigo-800">
          {userRoles.includes(Active.id) && <Trans>Active</Trans>}
        </span>
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        {user.last_sign_in
          ? formatRelative(parseISO(user.last_sign_in), new Date())
          : ""}
      </td>
      <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
        <button
          onClick={removeUser}
          className="text-indigo-600 hover:text-indigo-900"
        >
          <Trans>
            Remove<span className="sr-only">, {user.name}</span>
          </Trans>
        </button>
      </td>
    </tr>
  );
}

interface CollectionsEditTableProps {
  collections: CollectionSummary[];
  allCollections: CollectionSummary[];
  setCollections: Dispatch<SetStateAction<CollectionSummary[]>>;
  canAdd: boolean;
  canRemove: boolean;
}

function CollectionsEditTable({
  collections,
  allCollections,
  setCollections,
  canAdd,
  canRemove,
}: CollectionsEditTableProps) {
  const { i18nDbText } = useLocalize();
  const collectionIds = collections.map((collection) => collection.id);
  const searchableItems = allCollections
    .filter((collection) => !collectionIds.includes(collection.id))
    .map(
      (collection) =>
        ({
          id: collection.id,
          title: i18nDbText(collection.name),
          subtitle: i18nDbText(collection.acronym),
          imageUrl: collection.image_url
            ? cloudflareImageUrl(collection.image_url, "thumbnail")
            : null,
        } as SearchableFilterItem),
    );

  function onNewItem(newItem: SearchableFilterItem) {
    const newCollection = allCollections.find(
      (collection) => collection.id === newItem.id,
    ) as CollectionSummary;
    setCollections([...collections, newCollection]);
  }

  function remove(toRemove: CollectionSummary) {
    setCollections(
      collections.filter((collection) => collection.id !== toRemove.id),
    );
  }

  return (
    <div className="mt-10">
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900">
          <Trans>Collections</Trans>
        </h3>
        <p className="mt-1 text-sm text-gray-500">
          <Trans>
            Which collections users of this group will have access to.
          </Trans>
        </p>
      </div>

      {collections.length === 0 ? (
        <div className="text-sm text-gray-500 my-2">
          <Trans>List is empty.</Trans>
        </div>
      ) : (
        <table className="min-w-full divide-y divide-gray-300">
          <thead className="bg-gray-50">
            <tr>
              <th
                scope="col"
                className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6"
              >
                <Trans>Name</Trans>
              </th>
              <th
                scope="col"
                className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
              >
                <Trans># styles</Trans>
              </th>
              <th
                scope="col"
                className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
              >
                <Trans># colors</Trans>
              </th>
              <th
                scope="col"
                className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
              >
                <Trans># sizes</Trans>
              </th>
              <th scope="col" className="relative py-3.5 pl-3 pr-4 sm:pr-6">
                <span className="sr-only">Edit</span>
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white">
            {collections.map((collection) => (
              <CollectionTableRow
                key={collection.id}
                collection={collection}
                removeCollection={
                  canRemove ? () => remove(collection) : undefined
                }
              />
            ))}
          </tbody>
        </table>
      )}
      {canAdd && (
        <SearchableFilter
          title={t`Add collection`}
          items={searchableItems}
          onNewItem={onNewItem}
        />
      )}
    </div>
  );
}

interface CollectionTableRowProps {
  collection: CollectionSummary;
  removeCollection?: DispatchWithoutAction;
}

function CollectionTableRow({
  collection,
  removeCollection,
}: CollectionTableRowProps) {
  const { i18nDbText } = useLocalize();
  const imageUrl = collection.image_url
    ? cloudflareImageUrl(collection.image_url, "thumbnail")
    : "";
  return (
    <tr>
      <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm sm:pl-6">
        <div className="flex items-center">
          <div className="h-10 w-10 flex-shrink-0">
            {imageUrl ? (
              <img
                className="h-10 max-w-10 rounded"
                src={imageUrl}
                loading="lazy"
                alt=""
              />
            ) : (
              ""
            )}
          </div>
          <div className="ml-4">
            <div className="font-medium text-gray-900">
              {i18nDbText(collection.name)}
            </div>
            <div className="text-gray-500">
              {i18nDbText(collection.acronym)}
            </div>
          </div>
        </div>
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        {collection.num_styles}
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        {collection.num_colors}
      </td>
      <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
        {collection.num_sizes}
      </td>
      <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
        {!!removeCollection && (
          <button
            onClick={removeCollection}
            className="text-indigo-600 hover:text-indigo-900"
          >
            <Trans>
              Remove
              <span className="sr-only">, {i18nDbText(collection.name)}</span>
            </Trans>
          </button>
        )}
      </td>
    </tr>
  );
}
