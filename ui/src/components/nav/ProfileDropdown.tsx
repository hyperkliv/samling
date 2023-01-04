import ProfileDropdownMobile from "./ProfileDropdownMobile";
import ProfileDropdownDesktop from "./ProfileDropdownDesktop";
import { t } from "@lingui/macro";
import { useLocalize } from "../../i18n";
import { useAppSelector } from "../../state/hooks";
import { ProfileDropdownLink, ProfileLinkType } from "./propTypes";

const SUPPORT_EMAIL = "support@hyperkliv.se";

export default function ProfileDropdown() {
  const { user } = useAppSelector((state) => state.user);
  const { i18nLink } = useLocalize();

  let linkGroups: ProfileDropdownLink[][] = [];
  if (user === null) {
    linkGroups = [
      [{ text: t`Login`, to: i18nLink("/auth/login") }],
      [{ text: t`Support`, to: SUPPORT_EMAIL, type: ProfileLinkType.email }],
    ];
  } else {
    linkGroups = [
      [
        {
          text: t`View profile`,
          to: i18nLink(`/users/${user.id}/profile`),
        },
        { text: t`Settings`, to: i18nLink(`/users/${user.id}/settings`) },
        {
          text: t`Notifications`,
          to: i18nLink(`/users/${user.id}/notifications`),
        },
      ],
      [{ text: t`Support`, to: SUPPORT_EMAIL, type: ProfileLinkType.email }],
      [{ text: t`Sign out`, to: i18nLink(`/auth/logout`) }],
    ];
  }
  return (
    <>
      <div className="lg:hidden">
        <ProfileDropdownMobile linkGroups={linkGroups} />
      </div>
      <div className="hidden lg:block">
        <ProfileDropdownDesktop linkGroups={linkGroups} />
      </div>
    </>
  );
}
