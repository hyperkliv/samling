import React from "react";
import { Link, LinkProps } from "react-router-dom";
import { useLocalize } from "../i18n";
import { dummyLink } from "../utils";

interface DummyLocaleLinkProps extends Omit<LinkProps, "to"> {
  override?: string;
}

const DummyLocaleLink = React.forwardRef(
  ({ override, ...props }: DummyLocaleLinkProps, ref) => {
    const { i18nLink } = useLocalize();
    const to = i18nLink(dummyLink(), override);
    return <Link to={to} ref={ref as any} {...props} />;
  },
);

export default DummyLocaleLink;
