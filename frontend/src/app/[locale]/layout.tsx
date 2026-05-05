import { hasLocale, NextIntlClientProvider } from "next-intl";
import { getMessages } from "next-intl/server";
import { Providers } from "@/components/providers";
import "../globals.css";
import type { Metadata } from "next";
import { Outfit } from "next/font/google";
import { notFound } from "next/navigation";
import { routing } from "@/i18n/routing";

export const metadata: Metadata = {
  title: "Scandium",
  description: "Modern customer-to-customer marketplace",
};

type Props = {
  children: React.ReactNode;
  modal: React.ReactNode;
  params: Promise<{ locale: string }>;
};

const outfit = Outfit({ subsets: ["latin"] });

export default async function LocaleLayout({ children, modal, params }: Props) {
  const { locale } = await params;

  if (!hasLocale(routing.locales, locale)) {
    notFound();
  }

  const messages = await getMessages();

  return (
    <html lang={locale} className={outfit.className} suppressHydrationWarning>
      <body>
        <NextIntlClientProvider messages={messages}>
          <Providers>
            {modal}
            {children}
          </Providers>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
