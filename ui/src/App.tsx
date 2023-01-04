import { Route, Routes } from "react-router-dom";

import React, { useEffect } from "react";
import NotFound from "./pages/app/errors/NotFound";
import NotFoundPublic from "./pages/public/errors/NotFoundPublic";
import LocaleNavigate from "./components/LocaleNavigate";
import SplashPage from "./pages/public/SplashPage";
import { i18n } from "@lingui/core";
import { I18nProvider } from "@lingui/react";
import { defaultLocale, dynamicActivate, useLocaleParam } from "./i18n";
import store from "./state/store";
import SignInPage from "./pages/public/SignInPage";
import SignOutPage from "./pages/app/SignOutPage";
import { debounce } from "debounce";
import { saveState } from "./browser-storage";
import UserMessagesList from "./components/UserMessagesList";
import { useAppSelector } from "./state/hooks";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { MsalProvider } from "@azure/msal-react";
import { msalInstance } from "./msal";
import settings from "./settings";
import AdminLayout from "./pages/app/admin/AdminLayout";
import Loading from "./components/Loading";
import AdminGroupsPage from "./pages/app/admin/AdminGroups";
import AdminUsersPage from "./pages/app/admin/AdminUsers";
import AdminCollectionsPage from "./pages/app/admin/AdminCollections";

const AppLayout = React.lazy(() => import("./AppLayout"));
const HomeScreen = React.lazy(() => import("./pages/app/HomeScreen"));
const Collection = React.lazy(() => import("./pages/app/Collection"));
const Style = React.lazy(() => import("./pages/app/Style"));
const AdminHome = React.lazy(() => import("./pages/app/admin/AdminHome"));
const AdminEditGroup = React.lazy(
  () => import("./pages/app/admin/AdminEditGroup"),
);
const AdminEditUser = React.lazy(
  () => import("./pages/app/admin/AdminEditUser"),
);
const AdminEditCollection = React.lazy(
  () => import("./pages/app/admin/AdminEditCollection"),
);
const AdminCreateCollection = React.lazy(
  () => import("./pages/app/admin/AdminCreateCollection"),
);

export default function App() {
  const localeParam = useLocaleParam();
  const user = useAppSelector((state) => state.user.user);

  useEffect(() => {
    // Activate i18n/l10n for the locale specified for the route
    dynamicActivate(!localeParam ? defaultLocale : localeParam);
  }, [localeParam]);

  // Save Redux state on changes
  const saveStateDebounced = debounce(() => {
    const result = saveState(store.getState());
    result.mapErr((err) => {
      console.error(err);
    });
  }, 800);
  store.subscribe(saveStateDebounced);

  // Ensure that Redux state is persisted on user exit
  useEffect(() => {
    window.addEventListener("beforeunload", () => {
      saveStateDebounced.flush();
    });
  });

  return (
    <MsalProvider instance={msalInstance}>
      <GoogleOAuthProvider clientId={settings.googleClientId}>
        <I18nProvider i18n={i18n} forceRenderOnLocaleChange={false}>
          <UserMessagesList />
          <Routes>
            <Route path="/" element={<SplashPage />} caseSensitive></Route>
            <Route
              path="/app"
              element={<LocaleNavigate to="/app" />}
              caseSensitive
            ></Route>
            <Route path="/:locale/">
              <Route path="" element={<SplashPage />} caseSensitive />
              <Route path="auth/login" element={<SignInPage />}></Route>
              <Route path="auth/logout" element={<SignOutPage />}></Route>
              <Route
                path="app/"
                element={
                  <React.Suspense fallback={<Loading />}>
                    <AppLayout />
                  </React.Suspense>
                }
                caseSensitive
              >
                <Route
                  path=""
                  element={
                    <React.Suspense fallback={<Loading />}>
                      <HomeScreen />
                    </React.Suspense>
                  }
                  caseSensitive
                />
                <Route path="admin/" element={<AdminLayout />} caseSensitive>
                  <Route
                    path=""
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminHome />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="groups"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminGroupsPage />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="groups/:groupSlug"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminEditGroup />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="users"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminUsersPage />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="users/:userId"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminEditUser />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="collections"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminCollectionsPage />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="collections/new"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminCreateCollection />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                  <Route
                    path="collections/:collectionId"
                    element={
                      <React.Suspense fallback={<Loading />}>
                        <AdminEditCollection />
                      </React.Suspense>
                    }
                    caseSensitive
                  />
                </Route>
                <Route
                  path="collections/:collectionSlug"
                  element={
                    <React.Suspense fallback={<Loading />}>
                      <Collection />
                    </React.Suspense>
                  }
                  caseSensitive
                />
                <Route
                  path="collections/:collectionSlug/:styleSlug"
                  element={
                    <React.Suspense fallback={<Loading />}>
                      <Style />
                    </React.Suspense>
                  }
                  caseSensitive
                />
                <Route
                  path="collections/:collectionSlug/:styleSlug/:colorSlug"
                  element={
                    <React.Suspense fallback={<Loading />}>
                      <Style />
                    </React.Suspense>
                  }
                  caseSensitive
                />
                <Route
                  path="*"
                  element={user ? <NotFound /> : <NotFoundPublic />}
                />
              </Route>
              <Route path="*" element={<NotFoundPublic />} />
            </Route>
          </Routes>
        </I18nProvider>
      </GoogleOAuthProvider>
    </MsalProvider>
  );
}
