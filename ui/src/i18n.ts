import { i18n, I18n } from "@lingui/core";
import { useLingui } from "@lingui/react";
import { en, sv } from "make-plural/plurals";
import { useLocation } from "react-router-dom";
import { I18NString } from "./types/api";

export const locales = {
  en: "English",
  sv: "Svenska",
};
export { i18n };
export const localeCodes = Object.keys(locales);

export const defaultLocale = "en";

interface I18nHookData {
  locale: string;
  i18nLink: (to: string, override?: string) => string;
  i18n: I18n;
  i18nDbText: (s: I18NString | undefined) => string;
}

export function localizeLink(
  to: string,
  locale: string,
  override?: string,
): string {
  return `/${override || locale}${to}`;
}

export function useLocalize(): I18nHookData {
  const { i18n } = useLingui();
  return {
    locale: i18n.locale,
    i18nLink: function i18nLink(to: string, override?: string) {
      return localizeLink(to, i18n.locale, override);
    },
    i18n: i18n,
    i18nDbText: function i18nDbText(s: I18NString | undefined): string {
      if (!s) {
        return "";
      }
      type I18NStringLocale = keyof I18NString;
      return s[i18n.locale as I18NStringLocale] || s[defaultLocale] || "";
    },
  };
}

export function useLocaleParam(): string | null {
  const { pathname } = useLocation();
  const maybeLocale = pathname.split("/")[1];
  return localeCodes.indexOf(maybeLocale) === -1 ? null : maybeLocale;
}

i18n.loadLocaleData({
  en: { plurals: en },
  sv: { plurals: sv },
});

/**
 * We do a dynamic import of just the catalog that we need
 * @param locale any locale string
 */
export async function dynamicActivate(locale: string) {
  const { messages } = await import(`./locales/${locale}/messages`);
  i18n.load(locale, messages);
  i18n.activate(locale);
}
