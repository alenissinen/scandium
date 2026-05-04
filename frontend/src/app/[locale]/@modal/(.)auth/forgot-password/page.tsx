"use client";

import { useRouter } from "next/navigation";
import { ForgotPasswordForm } from "@/components/auth/forgot-password-form";

export default function ForgotPasswordModal() {
  const router = useRouter();

  return (
    <div
      role="none"
      className="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4"
      onClick={() => router.back()}
    >
      <div
        role="none"
        className="bg-card border border-border rounded-xl p-8 w-full max-w-sm"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="text-center mb-6">
          <h2 className="text-lg font-bold tracking-widest text-foreground">
            SCAN<span className="text-primary">DIUM</span>
          </h2>
        </div>
        <ForgotPasswordForm modal />
      </div>
    </div>
  );
}
