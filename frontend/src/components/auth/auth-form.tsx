"use client";

import { Eye, EyeOff } from "lucide-react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { useActionState, useEffect, useState } from "react";
import { FaGithub } from "react-icons/fa";
import { FcGoogle } from "react-icons/fc";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useUser } from "@/contexts/user-context";
import { Link } from "@/i18n/navigation";

type AuthMode = "login" | "register";
type ActionState = { error?: string; success?: boolean } | null;
type AuthFormProps = {
  modal?: boolean;
};

async function loginAction(_prevState: ActionState, formData: FormData): Promise<ActionState> {
  const requestBody = {
    login_handle: formData.get("emailOrUsername") as string,
    password: formData.get("password") as string,
  };

  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    const data = await response.json();
    return { error: data.error ?? "Something went wrong" };
  }

  return { success: true };
}

async function registerAction(_prevState: ActionState, formData: FormData): Promise<ActionState> {
  const requestBody = {
    email: formData.get("email") as string,
    username: formData.get("username") as string,
    display_name: formData.get("fullName") as string,
    password: formData.get("password") as string,
  };

  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include",
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    const data = await response.json();
    return { error: data.error ?? "Something went wrong" };
  }

  return { success: true };
}

export function AuthForm({ modal }: AuthFormProps) {
  const [mode, setMode] = useState<AuthMode>("login");
  const [showPassword, setShowPassword] = useState<boolean>(false);
  const t = useTranslations("auth");
  const router = useRouter();
  const { refreshUser } = useUser();

  const [loginState, loginFormAction, isLoginPending] = useActionState(loginAction, null);
  const [registerState, registerFormAction, isRegisterPending] = useActionState(
    registerAction,
    null
  );

  // Redirect/close modal on successful login/register
  useEffect(() => {
    if (loginState?.success || registerState?.success) {
      refreshUser();
      if (modal) {
        router.back();
      } else {
        router.push("/");
      }
    }
  }, [loginState, registerState, router, modal, refreshUser]);

  return (
    <div className="flex flex-col gap-6 w-full">
      {/* Mode tabs */}
      <div className="flex border border-border rounded-lg p-1 bg-muted">
        <Button
          type="button"
          onClick={() => setMode("login")}
          className={`flex-1 text-sm py-1.5 rounded-md transition-colors cursor-pointer ${
            mode === "login"
              ? "bg-card text-foreground font-medium shadow"
              : "bg-muted text-foreground hover:text-muted-foreground"
          }`}
        >
          {t("login")}
        </Button>
        <Button
          type="button"
          onClick={() => setMode("register")}
          className={`flex-1 text-sm py-1.5 rounded-md transition-colors cursor-pointer ${
            mode === "register"
              ? "bg-card text-foreground font-medium shadow"
              : "bg-muted text-foreground hover:text-muted-foreground"
          }`}
        >
          {t("register")}
        </Button>
      </div>

      {/* Login form */}
      {mode === "login" && (
        <form action={loginFormAction} className="flex flex-col gap-6">
          <div className="flex flex-col gap-4">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="emailOrUsername">{t("emailOrUsername")}</Label>
              <Input id="emailOrUsername" name="emailOrUsername" type="text" required />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="password">{t("password")}</Label>
              <div className="relative">
                <Input
                  id="password"
                  name="password"
                  type={showPassword ? "text" : "password"}
                  required
                />
                <Button
                  variant="ghost"
                  type="button"
                  size="icon"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-0.5"
                >
                  {showPassword ? <EyeOff size={14} /> : <Eye size={14} />}
                </Button>
              </div>
            </div>
          </div>

          {loginState?.error && <p className="text-destructive text-sm">{loginState.error}</p>}

          <Button type="submit" className="w-full" disabled={isLoginPending}>
            {isLoginPending ? "..." : t("login")}
          </Button>

          <hr className="border-border" />

          <div className="flex flex-col gap-2">
            <Button type="button" variant="outline" className="w-full gap-2">
              <FcGoogle size={16} />
              {t("continueWithGoogle")}
            </Button>
            <Button type="button" variant="outline" className="w-full gap-2">
              <FaGithub size={16} />
              {t("continueWithGithub")}
            </Button>
          </div>
          <div className="text-center">
            <Link
              href="/auth/forgot-password"
              className="text-xs text-muted-foreground hover:text-foreground transition-colors"
            >
              {t("forgotPassword")}
            </Link>
          </div>
        </form>
      )}

      {/* Register form */}
      {mode === "register" && (
        <form action={registerFormAction} className="flex flex-col gap-6">
          <div className="flex flex-col gap-4">
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="username">{t("username")}</Label>
              <Input
                id="username"
                name="username"
                type="text"
                placeholder={t("usernamePlaceholder")}
                required
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="fullName">{t("fullName")}</Label>
              <Input
                id="fullName"
                name="fullName"
                type="text"
                placeholder={t("fullNamePlaceholder")}
                required
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="email">{t("email")}</Label>
              <Input
                id="email"
                name="email"
                type="email"
                placeholder={t("emailPlaceholder")}
                required
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <Label htmlFor="password">{t("password")}</Label>
              <div className="relative">
                <Input
                  id="password"
                  name="password"
                  type={showPassword ? "text" : "password"}
                  placeholder={t("passwordPlaceholder")}
                  required
                />
                <Button
                  variant="ghost"
                  type="button"
                  size="icon"
                  onClick={() => setShowPassword(!showPassword)}
                  className="absolute right-0.5"
                >
                  {showPassword ? <EyeOff size={14} /> : <Eye size={14} />}
                </Button>
              </div>
            </div>
          </div>

          {registerState?.error && (
            <p className="text-destructive text-sm">{registerState.error}</p>
          )}

          <Button type="submit" className="w-full" disabled={isRegisterPending}>
            {isRegisterPending ? "..." : t("register")}
          </Button>

          <hr className="border-border" />

          <div className="flex flex-col gap-2">
            <Button type="button" variant="outline" className="w-full gap-2">
              <FcGoogle size={16} />
              {t("continueWithGoogle")}
            </Button>
            <Button type="button" variant="outline" className="w-full gap-2">
              <FaGithub size={16} />
              {t("continueWithGithub")}
            </Button>
          </div>
        </form>
      )}
    </div>
  );
}
