import { ArrowLeft } from "lucide-react";
import { redirect } from "next/navigation";
import { getTranslations } from "next-intl/server";
import { ResetPasswordForm } from "@/components/auth/reset-password-form";
import { Button } from "@/components/ui/button";
import { Link } from "@/i18n/navigation";

async function verifyToken(token: string): Promise<boolean> {
  const response = await fetch(`${process.env.API_URL}/api/v1/auth/verify-reset-token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ token }),
    cache: "no-store",
  });

  return response.ok;
}

export default async function ResetPasswordPage({
  searchParams,
}: {
  searchParams: Promise<{ token?: string }>;
}) {
  const { token } = await searchParams;
  const t = await getTranslations("auth");

  if (!token) redirect("/auth/forgot-password");

  const isValid = await verifyToken(token);

  if (!isValid) {
    return (
      <main className="min-h-screen bg-background flex items-center justify-center px-4">
        <div className="bg-card border border-border rounded-xl p-8 w-full max-w-sm text-center">
          <div className="mb-6">
            <h1 className="text-lg font-bold tracking-widest text-foreground">
              SCAN<span className="text-primary">DIUM</span>
            </h1>
          </div>
          <p className="text-foreground font-medium">{t("invalidResetLink")}</p>
          <div className="text-center">
            <Button variant="ghost" size="lg" asChild>
              <Link href="/auth/forgot-password">{t("requestNewLink")}</Link>
            </Button>
          </div>
        </div>
      </main>
    );
  }

  return (
    <main className="min-h-screen bg-background flex items-center justify-center px-4">
      <div className="bg-card border border-border rounded-xl p-8 w-full max-w-sm">
        <div className="text-center mb-6">
          <h1 className="text-lg font-bold tracking-widest text-foreground">
            SCAN<span className="text-primary">DIUM</span>
          </h1>
        </div>
        <div className="mb-8">
          <ResetPasswordForm token={token} />
        </div>
        <div className="text-center">
          <Button variant="ghost" size="lg" asChild>
            <Link href="/">
              <ArrowLeft size={16} />
              {t("back")}
            </Link>
          </Button>
        </div>
      </div>
    </main>
  );
}
