"use client";

import { AccessToken } from "@/client";
import { ROUTE } from "@/constants";
import { cn } from "@/lib/utils";
import Image from "next/image";
import Link from "next/link";
import { use } from "react";

type UserProfileButtonProps = {
  accessTokenPromise: Promise<AccessToken>;
};
export default function UserProfileButton({
  accessTokenPromise,
}: UserProfileButtonProps) {
  const accessToken = use(accessTokenPromise);
  const isLoggedIn = typeof accessToken === "string";

  return (
    <Link
      href={ROUTE.user}
      aria-disabled={!isLoggedIn}
      className={cn("overflow-hidden relative w-16 h-16 rounded-full bg-prime")}
    >
      <Image
        src={isLoggedIn ? "/echo-logo.webp" : "/unknown-user.webp"}
        alt={isLoggedIn ? "user-profile" : "unknown-user-profile"}
        fill
        className="object-contain"
      />
    </Link>
  );
}
