import { ArrowLeft } from "lucide-react";
import { useTranslations } from "next-intl";
import { AuthForm } from "@/components/auth/auth-form";
import { Button } from "@/components/ui/button";
import { Link } from "@/i18n/navigation";

export default function AuthPage() {
  const t = useTranslations("auth");

  return (
    <main className="min-h-screen bg-background flex items-center justify-center px-4">
      <div className="w-full max-w-sm bg-card border border-border rounded-xl flex flex-col p-8">
        <div className="text-center mb-8">
          <h1 className="text-2xl font-bold tracking-widest text-foreground">
            SCAN<span className="text-primary">DIUM</span>
          </h1>
        </div>
        <div className="mb-8">
          <AuthForm />
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
