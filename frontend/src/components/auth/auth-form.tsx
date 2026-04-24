"use client";

import { useTranslations } from "next-intl";
import { useState } from "react";
import { FaGithub } from "react-icons/fa";
import { FcGoogle } from "react-icons/fc";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

type AuthMode = "login" | "register";

export function AuthForm() {
  const [mode, setMode] = useState<AuthMode>("login");
  const t = useTranslations("auth");

  return (
    <div className="flex flex-col gap-6 w-full">
      <div className="flex border border-border rounded-lg p-1 bg-muted">
        <Button
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

      {(mode === "register" && (
        /* Register form */
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-1.5">
            <Label htmlFor="username">{t("username")}</Label>
            <Input id="username" type="text" placeholder={t("usernamePlaceholder")} />
          </div>

          <div className="flex flex-col gap-1.5">
            <Label htmlFor="name">{t("fullName")}</Label>
            <Input id="name" type="text" placeholder={t("fullNamePlaceholder")} />
          </div>

          <div className="flex flex-col gap-1.5">
            <Label htmlFor="email">{t("email")}</Label>
            <Input id="email" type="email" placeholder={t("emailPlaceholder")} />
          </div>

          <div className="flex flex-col gap-1.5">
            <Label htmlFor="password">{t("password")}</Label>
            <Input id="password" type="password" placeholder={t("passwordPlaceholder")} />
          </div>
        </div>
      )) || (
        /* Login form */
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-1.5">
            <Label htmlFor="email">{t("emailOrUsername")}</Label>
            <Input id="email" type="email" placeholder={t("emailPlaceholder")} />
          </div>

          <div className="flex flex-col gap-1.5">
            <Label htmlFor="password">{t("password")}</Label>
            <Input id="password" type="password" placeholder={t("passwordPlaceholder")} />
          </div>
        </div>
      )}

      <Button className="w-full">{mode === "login" ? t("login") : t("register")}</Button>

      <hr />

      <div className="flex flex-col gap-2">
        <Button variant="outline" className="w-full gap-2">
          <FcGoogle size={16} />
          {t("continueWithGoogle")}
        </Button>
        <Button variant="outline" className="w-full gap-2">
          <FaGithub size={16} />
          {t("continueWithGithub")}
        </Button>
      </div>
    </div>
  );
}
