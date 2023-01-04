import { Navigate } from "react-router-dom";
import { defaultLocale } from "../i18n";

interface LocaleNavigateProps {
  to: string;
}

export default function LocaleNavigate(props: LocaleNavigateProps) {
  const to = `/${defaultLocale}${props.to}`;
  return <Navigate replace to={to} />;
}
