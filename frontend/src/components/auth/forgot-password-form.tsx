"use client";

import { useTranslations } from "next-intl";
import { useActionState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

type ActionState = { error?: string; success?: boolean } | null;
type ForgotPasswordFormProps = {
  modal?: boolean;
};

async function forgotPasswordAction(
  _prevState: ActionState,
  formData: FormData
): Promise<ActionState> {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/forgot-password`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email: formData.get("email") as string }),
  });

  if (!response.ok) {
    const data = await response.json();
    return { error: data.error ?? "Something went wrong" };
  }

  return { success: true };
}

export function ForgotPasswordForm({ modal: _modal }: ForgotPasswordFormProps) {
  const t = useTranslations("auth");
  const [state, action, isPending] = useActionState(forgotPasswordAction, null);

  if (state?.success) {
    return (
      <div className="text-center flex flex-col gap-2">
        <p className="text-sm text-foreground font-medium">{t("forgotPasswordSuccessTitle")}</p>
        <p className="text-xs text-muted-foreground">{t("forgotPasswordSuccessDescription")}</p>
      </div>
    );
  }

  return (
    <form action={action} className="flex flex-col gap-6">
      <p className="text-sm text-muted-foreground mt-2">{t("forgotPasswordDescription")}</p>
      <div className="flex flex-col gap-1.5">
        <Label htmlFor="email">{t("email")}</Label>
        <Input id="email" name="email" type="email" placeholder={t("emailPlaceholder")} required />
      </div>

      {state?.error && <p className="text-destructive text-sm">{state.error}</p>}

      <Button type="submit" className="w-full" disabled={isPending}>
        {isPending ? "..." : t("sendResetLink")}
      </Button>
    </form>
  );
}
