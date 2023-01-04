import React from "react";
import { Link, LinkProps } from "react-router-dom";
import { useLocalize } from "../i18n";

interface LocaleLinkProps extends LinkProps {
  override?: string;
}

const LocaleLink = React.forwardRef(
  ({ to, override, ...props }: LocaleLinkProps, ref) => {
    const { i18nLink } = useLocalize();
    if ((to as string).startsWith("/")) {
      const newTo = i18nLink(to as string, override);
      return <Link to={newTo} ref={ref as any} {...props} />;
    } else {
      // Relative - assume locale already in link
      return <Link to={to} ref={ref as any} {...props} />;
    }
  },
);

export default LocaleLink;
