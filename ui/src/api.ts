import settings from "./settings";
import { Result, Ok, Err } from "oxide.ts";
import {
  AuthenticatedUser,
  CollectionSummary,
  CollectionWithItems,
  NestedStyleSortOrder,
  PriceListSummary,
  GoogleCredentials,
  MicrosoftCredentials,
  ApiErrorResponse,
  ExportFormat,
  ApiErrorCode,
  GroupSummary,
  CreateGroup,
  Group,
  UpdateGroup,
  User,
  CollectionFilters,
  CollectionItem,
  GroupBy,
  ExportField,
  CreateUser,
  UserSortOrder,
  UserFilters,
  UpdateUser,
  Collection,
  UpdateCollection,
  CreateCollection,
  Color,
  StyleFilters,
  NestedStyleSummary,
  Category,
  ItemFilterChoices,
} from "./types/api";
import { useAppDispatch, useAppSelector } from "./state/hooks";
import { useEffect, useState } from "react";
import { logoutExpiredUser } from "./state/slices/user";
import { useNavigate } from "react-router-dom";
import { useLocalize } from "./i18n";
import { entityRefToPath, Ref, refIsValid } from "./entityRef";

interface ApiSurface {
  login: (
    email: string,
    password: string,
  ) => Promise<Result<AuthenticatedUser, ApiErrorResponse>>;
  googleLogin: (
    credentials: GoogleCredentials,
  ) => Promise<Result<AuthenticatedUser, ApiErrorResponse>>;
  microsoftLogin: (
    credentials: MicrosoftCredentials,
  ) => Promise<Result<AuthenticatedUser, ApiErrorResponse>>;
  getUserDetails: (
    token: string,
  ) => Promise<Result<AuthenticatedUser, ApiErrorResponse>>;
  fetchUserList: (
    token: string,
    organizationId: number,
    sortOrder?: UserSortOrder,
    filters?: UserFilters,
  ) => Promise<Result<User[], ApiErrorResponse>>;
  fetchUser: (
    token: string,
    organizationId: number,
    userId: number,
  ) => Promise<Result<User, ApiErrorResponse>>;
  fetchItemFilterChoices: (
    token: string,
    organizationId: number,
  ) => Promise<Result<ItemFilterChoices, ApiErrorResponse>>;
  fetchColors: (
    token: string,
    organizationId: number,
  ) => Promise<Result<Color[], ApiErrorResponse>>;
  fetchNestedStyleSummaries: (
    token: string,
    organizationId: number,
    sortOrder?: NestedStyleSortOrder,
    filters?: StyleFilters,
  ) => Promise<Result<NestedStyleSummary[], ApiErrorResponse>>;
  fetchCollectionList: (
    token: string,
    organizationId: number,
  ) => Promise<Result<CollectionSummary[], ApiErrorResponse>>;
  fetchCollectionWithItems: (
    token: string,
    organizationId: number,
    ref: Ref,
    sortOrder?: NestedStyleSortOrder,
    filters?: CollectionFilters,
  ) => Promise<Result<CollectionWithItems, ApiErrorResponse>>;
  fetchCollectionItem: (
    token: string,
    organizationId: number,
    collectionSlug: string,
    styleSlug: string,
  ) => Promise<Result<CollectionItem, ApiErrorResponse>>;
  fetchCollectionSummary: (
    token: string,
    organizationId: number,
    ref: Ref,
  ) => Promise<Result<CollectionSummary, ApiErrorResponse>>;
  deleteCollection(
    token: string,
    organizationId: number,
    collectionId: number,
  ): Promise<Result<boolean, ApiErrorResponse>>;
  createCollection(
    token: string,
    organizationId: number,
    collection: CreateCollection,
  ): Promise<Result<Collection, ApiErrorResponse>>;
  updateCollection(
    token: string,
    organizationId: number,
    collectionId: number,
    collection: UpdateCollection,
  ): Promise<Result<Collection, ApiErrorResponse>>;
  fetchGroup: (
    token: string,
    organizationId: number,
    groupSlug: string,
  ) => Promise<Result<Group, ApiErrorResponse>>;
  fetchGroupSummaries: (
    token: string,
    organizationId: number,
  ) => Promise<Result<GroupSummary[], ApiErrorResponse>>;
  fetchPriceLists: (
    token: string,
    organizationId: number,
  ) => Promise<Result<PriceListSummary[], ApiErrorResponse>>;
  fetchCategories: (
    token: string,
    organizationId: number,
  ) => Promise<Result<Category[], ApiErrorResponse>>;
  createGroup(
    token: string,
    organizationId: number,
    group: CreateGroup,
  ): Promise<Result<Group, ApiErrorResponse>>;
  updateGroup(
    token: string,
    organizationId: number,
    groupId: number,
    group: UpdateGroup,
  ): Promise<Result<Group, ApiErrorResponse>>;
  deleteGroup(
    token: string,
    organizationId: number,
    groupId: number,
  ): Promise<Result<boolean, ApiErrorResponse>>;
  createUser(
    token: string,
    organizationId: number,
    user: CreateUser,
  ): Promise<Result<User, ApiErrorResponse>>;
  updateUser(
    token: string,
    organizationId: number,
    userId: number,
    user: UpdateUser,
  ): Promise<Result<User, ApiErrorResponse>>;
  deleteUser(
    token: string,
    organizationId: number,
    userId: number,
  ): Promise<Result<boolean, ApiErrorResponse>>;
  downloadExportFile(
    token: string,
    organizationId: number,
    collectionSlug: string,
    exportFormat: ExportFormat,
    groupBy: GroupBy[],
    fields: ExportField[],
    filters?: CollectionFilters,
  ): Promise<Response>;
}

interface ApiGetRequestOptions {
  token: string;
  sortOrder?: string;
  filters?: object;
  groupBy?: GroupBy[]; // TODO: This is only really meant for downloadExportFile...
  fields?: ExportField[]; // TODO: This is only really meant for downloadExportFile...
}

interface ApiPostRequestOptions {
  token: string;
}

const api: ApiSurface = makeApi(settings.apiBaseUrl);

export default api;

async function toApiResult<T>(
  fut: Promise<Response>,
): Promise<Result<T, ApiErrorResponse>> {
  try {
    const res = await fut;
    if (res.ok) {
      if (res.status === 204) {
        return Ok(true as any);
      } else {
        const value = await res.json();
        return Ok(value);
      }
    } else {
      const error = await res.json();
      return Err(error);
    }
  } catch (error) {
    console.error(error);
    return Err({
      error_code: ApiErrorCode.IoError,
      error_message:
        "A network error occurred while fetching data from the backend.",
    });
  }
}

function makeApi(baseUrl: string): ApiSurface {
  async function fetchRaw(
    subpath: string,
    opts: ApiGetRequestOptions,
  ): Promise<Response> {
    const url = new URL(`${baseUrl}${subpath}`);
    const searchParams = new URLSearchParams();
    if (!!opts.filters) {
      searchParams.append("filters", JSON.stringify(opts.filters));
    }
    if (!!opts.groupBy) {
      searchParams.append("group_by", JSON.stringify(opts.groupBy));
    }
    if (!!opts.fields) {
      searchParams.append("fields", JSON.stringify(opts.fields));
    }
    url.search = searchParams.toString();
    const req = new Request(url, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${opts.token}`,
      },
    });
    return fetch(req);
  }
  async function getJson<T>(
    subpath: string,
    opts: ApiGetRequestOptions,
  ): Promise<Result<T, ApiErrorResponse>> {
    const url = new URL(`${baseUrl}${subpath}`);
    const searchParams = new URLSearchParams();
    if (!!opts.filters) {
      searchParams.append("filters", JSON.stringify(opts.filters));
    }
    if (!!opts.sortOrder) {
      searchParams.append("sort_by", opts.sortOrder);
    }
    url.search = searchParams.toString();
    const req = new Request(url, {
      method: "GET",
      headers: {
        Accept: "application/json",
        Authorization: `Bearer ${opts.token}`,
      },
    });
    const res = fetch(req);
    return await toApiResult(res);
  }
  async function postJson<I, O>(
    subpath: string,
    data: I,
    opts: ApiPostRequestOptions,
  ): Promise<Result<O, ApiErrorResponse>> {
    const url = new URL(`${baseUrl}${subpath}`);
    const searchParams = new URLSearchParams();
    url.search = searchParams.toString();
    const req = new Request(url, {
      method: "POST",
      body: serializeApiBody(data),
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        Authorization: `Bearer ${opts.token}`,
      },
    });
    const res = fetch(req);
    return await toApiResult(res);
  }
  async function patchJson<I, O>(
    subpath: string,
    data: I,
    opts: ApiPostRequestOptions,
  ): Promise<Result<O, ApiErrorResponse>> {
    const url = new URL(`${baseUrl}${subpath}`);
    const searchParams = new URLSearchParams();
    url.search = searchParams.toString();
    const req = new Request(url, {
      method: "PATCH",
      body: serializeApiBody(data),
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        Authorization: `Bearer ${opts.token}`,
      },
    });
    const res = fetch(req);
    return await toApiResult(res);
  }
  async function deleteRequest(
    subpath: string,
    opts: ApiPostRequestOptions,
  ): Promise<Result<boolean, ApiErrorResponse>> {
    const url = new URL(`${baseUrl}${subpath}`);
    const searchParams = new URLSearchParams();
    url.search = searchParams.toString();
    const req = new Request(url, {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${opts.token}`,
      },
    });
    const res = fetch(req);
    return await toApiResult(res);
  }
  return {
    async login(
      email: string,
      password: string,
    ): Promise<Result<AuthenticatedUser, ApiErrorResponse>> {
      const req = new Request(`${baseUrl}/auth/login`, {
        method: "POST",
        body: serializeApiBody({
          email: email,
          password: password,
        }),
        headers: { "Content-Type": "application/json" },
      });
      const res = fetch(req);
      return await toApiResult(res);
    },
    async googleLogin(
      credentials: GoogleCredentials,
    ): Promise<Result<AuthenticatedUser, ApiErrorResponse>> {
      const req = new Request(`${baseUrl}/auth/login/google`, {
        method: "POST",
        body: serializeApiBody(credentials),
        headers: { "Content-Type": "application/json" },
      });
      const res = fetch(req);
      return await toApiResult(res);
    },
    async microsoftLogin(
      credentials: MicrosoftCredentials,
    ): Promise<Result<AuthenticatedUser, ApiErrorResponse>> {
      const req = new Request(`${baseUrl}/auth/login/microsoft`, {
        method: "POST",
        body: serializeApiBody(credentials),
        headers: { "Content-Type": "application/json" },
      });
      const res = fetch(req);
      return await toApiResult(res);
    },
    async getUserDetails(
      token: string,
    ): Promise<Result<AuthenticatedUser, ApiErrorResponse>> {
      return await getJson<AuthenticatedUser>(`/users/me`, { token });
    },
    async fetchUserList(
      token: string,
      organizationId: number,
      sortOrder?: UserSortOrder,
      filters?: UserFilters,
    ): Promise<Result<User[], ApiErrorResponse>> {
      return await getJson<User[]>(`/${organizationId}/users`, {
        token,
        sortOrder,
        filters,
      });
    },
    async fetchUser(
      token: string,
      organizationId: number,
      userId: number,
    ): Promise<Result<User, ApiErrorResponse>> {
      return await getJson<User>(`/${organizationId}/users/${userId}`, {
        token,
      });
    },
    async fetchItemFilterChoices (
      token: string,
      organizationId: number,
    ): Promise<Result<ItemFilterChoices, ApiErrorResponse>> {
      return await getJson<ItemFilterChoices>(`/${organizationId}/admin/filters/items`, {
        token,
      });
    },
    async fetchColors(
      token: string,
      organizationId: number,
    ): Promise<Result<Color[], ApiErrorResponse>> {
      return await getJson<Color[]>(`/${organizationId}/colors`, {
        token,
      });
    },
    async fetchCollectionList(
      token: string,
      organizationId: number,
    ): Promise<Result<CollectionSummary[], ApiErrorResponse>> {
      const res = await getJson<CollectionSummary[]>(
        `/${organizationId}/collections/summaries`,
        { token },
      );
      return res.map((values) => values.map((value) => value));
    },
    async fetchPriceLists(
      token: string,
      organizationId: number,
    ): Promise<Result<PriceListSummary[], ApiErrorResponse>> {
      return await getJson<PriceListSummary[]>(
        `/${organizationId}/pricelists/summary`,
        { token },
      );
    },
    async fetchCategories(
      token: string,
      organizationId: number,
    ): Promise<Result<Category[], ApiErrorResponse>> {
      return await getJson<Category[]>(`/${organizationId}/categories`, {
        token,
      });
    },
    async fetchGroup(
      token: string,
      organizationId: number,
      groupSlug: string,
    ): Promise<Result<Group, ApiErrorResponse>> {
      return await getJson<Group>(
        `/${organizationId}/groups/slug:${groupSlug}`,
        { token },
      );
    },
    async fetchGroupSummaries(
      token: string,
      organizationId: number,
    ): Promise<Result<GroupSummary[], ApiErrorResponse>> {
      return await getJson<GroupSummary[]>(
        `/${organizationId}/groups/summary`,
        { token },
      );
    },
    async fetchCollectionWithItems(
      token: string,
      organizationId: number,
      ref: Ref,
      sortOrder?: NestedStyleSortOrder,
      filters?: CollectionFilters,
    ): Promise<Result<CollectionWithItems, ApiErrorResponse>> {
      return await getJson<CollectionWithItems>(
        `/${organizationId}/collections/${entityRefToPath(ref)}/with-items`,
        { token, sortOrder, filters },
      );
    },
    async fetchNestedStyleSummaries(
      token: string,
      organizationId: number,
      sortOrder?: NestedStyleSortOrder,
      filters?: StyleFilters,
    ): Promise<Result<NestedStyleSummary[], ApiErrorResponse>> {
      return await getJson<NestedStyleSummary[]>(
        `/${organizationId}/styles/nested/summary`,
        { token, sortOrder, filters },
      );
    },
    async fetchCollectionItem(
      token: string,
      organizationId: number,
      collectionSlug: string,
      styleSlug: string,
    ): Promise<Result<CollectionItem, ApiErrorResponse>> {
      return await getJson<CollectionItem>(
        `/${organizationId}/collections/slug:${collectionSlug}/items/slug:${styleSlug}`,
        { token },
      );
    },
    async fetchCollectionSummary(
      token: string,
      organizationId: number,
      ref: Ref,
    ): Promise<Result<CollectionSummary, ApiErrorResponse>> {
      return await getJson<CollectionSummary>(
        `/${organizationId}/collections/${entityRefToPath(ref)}/summary`,
        { token },
      );
    },
    async deleteCollection(
      token: string,
      organizationId: number,
      collectionId: number,
    ): Promise<Result<boolean, ApiErrorResponse>> {
      return await deleteRequest(
        `/${organizationId}/collections/${collectionId}`,
        {
          token,
        },
      );
    },
    async createCollection(
      token: string,
      organizationId: number,
      collection: CreateCollection,
    ): Promise<Result<Collection, ApiErrorResponse>> {
      return await postJson<CreateCollection, Collection>(
        `/${organizationId}/collections`,
        collection,
        { token },
      );
    },
    async updateCollection(
      token: string,
      organizationId: number,
      collectionId: number,
      collection: UpdateCollection,
    ): Promise<Result<Collection, ApiErrorResponse>> {
      return await patchJson<UpdateCollection, Collection>(
        `/${organizationId}/collections/${collectionId}`,
        collection,
        { token },
      );
    },
    async createGroup(
      token: string,
      organizationId: number,
      group: CreateGroup,
    ): Promise<Result<Group, ApiErrorResponse>> {
      return await postJson<CreateGroup, Group>(
        `/${organizationId}/groups`,
        group,
        { token },
      );
    },
    async updateGroup(
      token: string,
      organizationId: number,
      groupId: number,
      group: UpdateGroup,
    ): Promise<Result<Group, ApiErrorResponse>> {
      return await patchJson<UpdateGroup, Group>(
        `/${organizationId}/groups/${groupId}`,
        group,
        { token },
      );
    },
    async deleteGroup(
      token: string,
      organizationId: number,
      groupId: number,
    ): Promise<Result<boolean, ApiErrorResponse>> {
      return await deleteRequest(`/${organizationId}/groups/${groupId}`, {
        token,
      });
    },
    async createUser(
      token: string,
      organizationId: number,
      user: CreateUser,
    ): Promise<Result<User, ApiErrorResponse>> {
      return await postJson<CreateUser, User>(
        `/${organizationId}/users`,
        user,
        { token },
      );
    },
    async updateUser(
      token: string,
      organizationId: number,
      userId: number,
      user: UpdateUser,
    ): Promise<Result<User, ApiErrorResponse>> {
      return await patchJson<UpdateUser, User>(
        `/${organizationId}/users/${userId}`,
        user,
        { token },
      );
    },
    async deleteUser(
      token: string,
      organizationId: number,
      userId: number,
    ): Promise<Result<boolean, ApiErrorResponse>> {
      return await deleteRequest(`/${organizationId}/users/${userId}`, {
        token,
      });
    },
    async downloadExportFile(
      token: string,
      organizationId: number,
      collectionSlug: string,
      exportFormat: ExportFormat,
      groupBy: GroupBy[],
      fields: ExportField[],
      filters?: CollectionFilters,
    ): Promise<Response> {
      const subpath = `/${organizationId}/exports/slug:${collectionSlug}/${exportFormat}`;
      return fetchRaw(subpath, { token, filters, groupBy, fields }).catch(
        (err) => {
          console.error(`Failed to download file, with error: ${err}`);
          return err;
        },
      );
    },
  };
}

type UserListFetchResult = Result<User[] | null, ApiErrorResponse>;

export function useUserList(
  sortBy?: UserSortOrder,
  filters?: UserFilters,
): [UserListFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as UserListFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchUserList(
          token as string,
          activeOrganization.organization.id,
          sortBy,
          filters,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, sortBy, filters, refreshCount]);

  return [fetchResult, refresh];
}

type UserFetchResult = Result<User | null, ApiErrorResponse>;

export function useUser(
  userId: number | undefined,
): [UserFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(Ok(null) as UserFetchResult);
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!userId && !!token && !!activeOrganization) {
      api
        .fetchUser(token as string, activeOrganization.organization.id, userId)
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [userId, token, activeOrganization, refreshCount]);

  return [fetchResult, refresh];
}

type CollectionWithItemsFetchResult = Result<
  CollectionWithItems | null,
  ApiErrorResponse
>;

export function useCollectionWithItems(
  ref: Ref,
  sortOrder?: NestedStyleSortOrder,
  filters?: CollectionFilters,
): [CollectionWithItemsFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as CollectionWithItemsFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!ref && !!token && !!activeOrganization) {
      api
        .fetchCollectionWithItems(
          token as string,
          activeOrganization.organization.id,
          ref,
          sortOrder,
          filters,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, ref, sortOrder, filters, refreshCount]);

  return [fetchResult, refresh];
}

type NestedStylesFetchResult = Result<
  NestedStyleSummary[] | null,
  ApiErrorResponse
>;

export function useNestedStyles(
  filters?: StyleFilters,
): [NestedStylesFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as NestedStylesFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchNestedStyleSummaries(
          token as string,
          activeOrganization.organization.id,
          undefined,
          filters,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, filters, refreshCount]);

  return [fetchResult, refresh];
}

type CollectionItemFetchResult = Result<
  CollectionItem | null,
  ApiErrorResponse
>;

export function useCollectionItem(
  collectionSlug: string | null,
  styleSlug: string | null,
): CollectionItemFetchResult {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as CollectionItemFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!collectionSlug && !!styleSlug && !!token && !!activeOrganization) {
      api
        .fetchCollectionItem(
          token as string,
          activeOrganization.organization.id,
          collectionSlug,
          styleSlug,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, collectionSlug, styleSlug]);

  return fetchResult;
}

type CollectionSummaryFetchResult = Result<
  CollectionSummary | null,
  ApiErrorResponse
>;

export function useCollectionSummary(
  ref: Ref,
): [CollectionSummaryFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    () => Ok(null) as CollectionSummaryFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (refIsValid(ref) && !!token && !!activeOrganization) {
      api
        .fetchCollectionSummary(
          token as string,
          activeOrganization.organization.id,
          ref,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, ref, refreshCount]);
  return [fetchResult, refresh];
}

type GroupFetchResult = Result<Group | null, ApiErrorResponse>;

export function useGroup(slug: string): [GroupFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(Ok(null) as GroupFetchResult);
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!slug && !!token && !!activeOrganization) {
      api
        .fetchGroup(token as string, activeOrganization.organization.id, slug)
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, slug, refreshCount]);

  return [fetchResult, refresh];
}

type GroupSummariesFetchResult = Result<
  GroupSummary[] | null,
  ApiErrorResponse
>;

export function useGroupSummaries(): [GroupSummariesFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as GroupSummariesFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchGroupSummaries(
          token as string,
          activeOrganization.organization.id,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, refreshCount]);

  return [fetchResult, refresh];
}

type PricelistsFetchResult = Result<
  PriceListSummary[] | null,
  ApiErrorResponse
>;

export function usePricelists(): PricelistsFetchResult {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as PricelistsFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchPriceLists(
          token as string,
          activeOrganization.organization.id,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization]);

  return fetchResult;
}

type CategoriesFetchResult = Result<Category[] | null, ApiErrorResponse>;

export function useCategories(): [CategoriesFetchResult, () => void] {
  const [refreshCount, refresh] = useRefresh();
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as CategoriesFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchCategories(token as string, activeOrganization.organization.id)
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, refreshCount]);

  return [fetchResult, refresh];
}

type CollectionListFetchResult = Result<
  CollectionSummary[] | null,
  ApiErrorResponse
>;

export function useCollectionList(): [CollectionListFetchResult, () => void] {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [refreshCount, refresh] = useRefresh();
  const [fetchResult, setFetchResult] = useState(
    Ok(null) as CollectionListFetchResult,
  );
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchCollectionList(
          token as string,
          activeOrganization.organization.id,
        )
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, refreshCount]);

  return [fetchResult, refresh];
}

type ItemFilterChoicesFetchResult = Result<ItemFilterChoices | null, ApiErrorResponse>;

export function useItemFilterChoices(): ItemFilterChoicesFetchResult {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [fetchResult, setFetchResult] = useState(Ok(null) as ItemFilterChoicesFetchResult);
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchItemFilterChoices(token as string, activeOrganization.organization.id)
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization]);

  return fetchResult;
}

type ColorsFetchResult = Result<Color[] | null, ApiErrorResponse>;

export function useColors(): [ColorsFetchResult, () => void] {
  const { token, activeOrganization } = useAppSelector((state) => state.user);
  const [refreshCount, refresh] = useRefresh();
  const [fetchResult, setFetchResult] = useState(Ok(null) as ColorsFetchResult);
  useLogoutOnExpiredToken(fetchResult);
  useEffect(() => {
    if (!!token && !!activeOrganization) {
      api
        .fetchColors(token as string, activeOrganization.organization.id)
        .then((result) => {
          setFetchResult(result);
        });
    }
  }, [token, activeOrganization, refreshCount]);

  return [fetchResult, refresh];
}

function useRefresh(): [number, () => void] {
  const [count, setCount] = useState(0);
  function refresh() {
    setCount(count + 1);
  }
  return [count, refresh];
}

function useLogoutOnExpiredToken(result: Result<any, ApiErrorResponse>) {
  const navigate = useNavigate();
  const { i18nLink } = useLocalize();
  const dispatch = useAppDispatch();
  result.orElse((err) => {
    if (err.error_code === ApiErrorCode.ExpiredToken) {
      dispatch(logoutExpiredUser());
      navigate(i18nLink("/auth/login"));
    }
    return result;
  });
}

function serializeApiBody(body: any): string {
  return JSON.stringify(body);
}
