import { getRequestConfig } from "next-intl/server";

export const locales = ["fi", "en"] as const;
export type Locale = (typeof locales)[number];
export const defaultLocale: Locale = "fi";

export default getRequestConfig(async ({ requestLocale }) => {
  const locale = (await requestLocale) ?? defaultLocale;

  return {
    locale,
    messages: (await import(`../../translations/${locale}.json`)).default,
  };
});
