"use client";

import { clsx } from "clsx";
import { FormEventHandler } from "react";
import { echoClient } from "@/client";
import type { AccessToken } from "@/client";
import { useRouter } from "next/navigation";

type AuthorizationButtonProps = {
  accessToken: AccessToken;
};

export default function AuthorizationButton({
  accessToken,
}: AuthorizationButtonProps) {
  const router = useRouter();
  const isAuthorized = typeof accessToken === "string";
  const client = echoClient(accessToken);

  const handleSubmit: FormEventHandler<HTMLFormElement> = async (event) => {
    event.preventDefault();
    if (isAuthorized) {
      const { data } = await client.GET("/auth/spotify");
      if (data?.url) {
        router.push(data.url);
      }
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <button
        type="submit"
        className={clsx("rounded px-2 py-1 cursor-pointer border", {
          hidden: isAuthorized,
        })}
      >
        Login
      </button>
    </form>
  );
}
