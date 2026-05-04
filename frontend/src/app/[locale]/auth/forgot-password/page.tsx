import { ArrowLeft } from "lucide-react";
import { getTranslations } from "next-intl/server";
import { ForgotPasswordForm } from "@/components/auth/forgot-password-form";
import { Button } from "@/components/ui/button";
import { Link } from "@/i18n/navigation";

export default async function ForgotPasswordPage() {
  const t = await getTranslations("auth");

  return (
    <main className="min-h-screen bg-background flex items-center justify-center px-4">
      <div className="w-full max-w-sm">
        <div className="bg-card border border-border rounded-xl p-8">
          <div className="text-center mb-6">
            <h1 className="text-lg font-bold tracking-widest text-foreground">
              SCAN<span className="text-primary">DIUM</span>
            </h1>
            <p className="text-sm text-muted-foreground mt-2">{t("forgotPasswordDescription")}</p>
          </div>
          <div className="mb-8">
            <ForgotPasswordForm />
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
      </div>
    </main>
  );
}
