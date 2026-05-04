"use client";

import { Eye, EyeOff } from "lucide-react";
import { useTranslations } from "next-intl";
import { useActionState, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

type ActionState = { error?: string; success?: boolean } | null;

async function resetPasswordAction(
  _prevState: ActionState,
  formData: FormData
): Promise<ActionState> {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/reset-password`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      token: formData.get("token") as string,
      password: formData.get("password") as string,
    }),
  });

  if (!response.ok) {
    const data = await response.json();
    return { error: data.error ?? "Something went wrong" };
  }

  return { success: true };
}

type ResetPasswordFormProps = {
  token: string;
};

export function ResetPasswordForm({ token }: ResetPasswordFormProps) {
  const t = useTranslations("auth");
  const [showPassword, setShowPassword] = useState(false);
  const [state, action, isPending] = useActionState(resetPasswordAction, null);

  if (state?.success) {
    return (
      <div className="text-center flex flex-col gap-2">
        <p className="text-sm text-foreground font-medium">{t("resetPasswordSuccessTitle")}</p>
        <p className="text-xs text-muted-foreground">{t("resetPasswordSuccessDescription")}</p>
      </div>
    );
  }

  return (
    <form action={action} className="flex flex-col gap-6">
      <input type="hidden" name="token" value={token} />

      <div className="flex flex-col gap-1.5">
        <Label htmlFor="password">{t("newPassword")}</Label>
        <div className="relative">
          <Input
            id="password"
            name="password"
            type={showPassword ? "text" : "password"}
            placeholder={t("passwordPlaceholder")}
            required
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
          >
            {showPassword ? <EyeOff size={14} /> : <Eye size={14} />}
          </button>
        </div>
      </div>

      {state?.error && <p className="text-destructive text-sm">{state.error}</p>}

      <Button type="submit" className="w-full" disabled={isPending}>
        {isPending ? "..." : t("resetPassword")}
      </Button>
    </form>
  );
}
