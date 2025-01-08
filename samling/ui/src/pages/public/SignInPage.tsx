import { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import api from "../../api";
import { useLocalize } from "../../i18n";
import { useAppDispatch, useAppSelector } from "../../state/hooks";
import * as google from "@react-oauth/google";
import {
  loginUser,
  userErrorMessage,
  wrongLoginCredentials,
  apiErrorMessage,
} from "../../state/slices/user";
import AlreadySignedInPage from "../app/AlreadySignedInPage";
import { t, Trans } from "@lingui/macro";
import { useMsal } from "@azure/msal-react";
import { ApiErrorCode, MicrosoftCredentials } from "../../types/api";
import * as msalBrowser from "@azure/msal-browser";
import { match } from "oxide.ts";
import SignInModal from "../../components/SignInModal";
import { useTitle } from "../../hooks";

interface SignInPageParams {
  next?: string;
}

export default function SignInPage({ next }: SignInPageParams) {
  const navigate = useNavigate();
  const { user } = useAppSelector((state) => state.user);
  const dispatch = useAppDispatch();
  const { i18nLink } = useLocalize();

  useTitle([t`Sign in`]);

  const [openSignInModal, setOpenSignInModal] = useState(false);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  function onTraditionalSignIn() {
    api
      .login(email, password)
      .then((result) => {
        match(result, {
          Ok: (user) => {
            dispatch(loginUser(user));
          },
          Err: (error) => {
            if (error.error_code === ApiErrorCode.InvalidUserCredentials) {
              dispatch(wrongLoginCredentials());
            } else {
              dispatch(apiErrorMessage(error));
            }
          },
        });
      })
      .catch(() => {
        dispatch(userErrorMessage(t`Technical error prevents login`));
      });
  }

  function onGoogleSuccess(resp: google.CredentialResponse) {
    if (!resp.credential) {
      console.error(
        `Google unexpectedly did not return credential value in response ${resp}?`,
      );
    } else {
      api
        .googleLogin({ idToken: resp.credential })
        .then((result) => {
          match(result, {
            Ok: (user) => {
              dispatch(loginUser(user));
            },
            Err: (error) => {
              if (error.error_code === ApiErrorCode.InvalidUserCredentials) {
                dispatch(wrongLoginCredentials());
              } else {
                dispatch(apiErrorMessage(error));
              }
            },
          });
        })
        .catch(() => {
          dispatch(userErrorMessage(t`Technical error prevents login`));
        });
    }
  }

  useEffect(() => {
    if (!!user) {
      navigate(next || i18nLink("/app"));
    }
  }, [user, i18nLink, navigate, next]);

  return (
    <>
      {!user ? (
        <div className="min-h-full flex flex-col justify-center py-12 sm:px-6 lg:px-8">
          <div className="sm:mx-auto sm:w-full sm:max-w-md">
            <img
              className="mx-auto h-12 w-auto"
              src="/logo.webp"
              alt="Workflow"
            />
            <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
              <Trans>Sign in to your account</Trans>
            </h2>
            {/* <p className="mt-2 text-center text-sm text-gray-600">
              Or{" "}
              <DummyLocaleLink className="font-medium text-indigo-600 hover:text-indigo-500">
                start your 14-day free trial
              </DummyLocaleLink>
            </p> */}
          </div>

          <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
            <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
              <div>
                <div className="mt-6 flex flex-col items-center space-y-8">
                  <div>
                    <MicrosoftLogin />
                  </div>
                  <div>
                    <google.GoogleLogin onSuccess={onGoogleSuccess} />
                  </div>
                  <div>
                    <button
                      onClick={() => setOpenSignInModal(true)}
                      className="inline-flex items-center rounded-md border border-gray-300 bg-white px-3 py-3 text-sm font-normal leading-4 text-gray-500 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                    >
                      E-mail and password sign in
                    </button>
                    <SignInModal
                      open={openSignInModal}
                      setOpen={setOpenSignInModal}
                      onSignIn={onTraditionalSignIn}
                      email={email}
                      setEmail={setEmail}
                      password={password}
                      setPassword={setPassword}
                    />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      ) : (
        <AlreadySignedInPage user={user} />
      )}
    </>
  );
}

function MicrosoftLogin() {
  const dispatch = useAppDispatch();
  const { instance } = useMsal();

  function onMicrosoftSuccess(resp: msalBrowser.AuthenticationResult) {
    api
      .microsoftLogin(resp as MicrosoftCredentials)
      .then((result) => {
        match(result, {
          Ok: (user) => {
            dispatch(loginUser(user));
          },
          Err: (error) => {
            if (error.error_code === ApiErrorCode.InvalidUserCredentials) {
              dispatch(wrongLoginCredentials());
            } else {
              dispatch(apiErrorMessage(error));
            }
          },
        });
      })
      .catch(() => {
        dispatch(userErrorMessage(t`Technical error prevents login`));
      });
  }

  function handlePopupLogin() {
    instance
      .loginPopup({ scopes: ["openid", "profile", "email", "User.Read"] })
      .then(onMicrosoftSuccess)
      .catch((e) => {
        console.error(e);
      });
  }
  return (
    <button onClick={() => handlePopupLogin()}>
      <img
        src="/ms-symbollockup_signin_light.svg"
        alt={t`Sign in with Microsoft`}
      />
    </button>
  );
}
