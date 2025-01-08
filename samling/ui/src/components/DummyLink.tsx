import React from "react";
import { Link, LinkProps } from "react-router-dom";
import { dummyLink } from "../utils";

interface DummyLinkProps extends Omit<LinkProps, "to"> {}

const DummyLink = React.forwardRef((props: DummyLinkProps, ref) => {
  return <Link to={dummyLink()} ref={ref as any} {...props} />;
});

export default DummyLink;
