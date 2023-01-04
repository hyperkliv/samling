import { t } from "@lingui/macro";
import { useEffect } from "react";
import { useNavigate } from "react-router";
import { useTitle } from "../../hooks";
import { useLocalize } from "../../i18n";
import { useAppDispatch, useAppSelector } from "../../state/hooks";
import { logoutUser } from "../../state/slices/user";

interface Props {
  next?: string;
}

export default function SignOutPage({ next }: Props) {
  useTitle([t`Sign out`]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { i18nLink } = useLocalize();
  const { user } = useAppSelector((state) => state.user);
  useEffect(() => {
    if (user) {
      dispatch(logoutUser());
    } else {
      navigate(next || i18nLink("/"));
    }
  }, [user, dispatch, navigate, i18nLink, next]);
  return <h1>Signing out...</h1>;
}
