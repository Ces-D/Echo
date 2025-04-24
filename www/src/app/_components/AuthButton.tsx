"use client";

import { AccessToken, echoClient } from "@/client";
import { cn } from "@/lib/utils";
import { useRouter } from "next/navigation";

type EchoClientT = ReturnType<typeof echoClient>;
type AppRouterInstance = ReturnType<typeof useRouter>;
type AuthButtonProps = {
  accessToken: AccessToken;
  isMenu?: boolean;
};

async function login(client: EchoClientT, router: AppRouterInstance) {
  const { data } = await client.GET("/auth/spotify");
  if (data?.url) {
    router.push(data.url);
  } else {
    alert("Error logging in");
  }
}

async function logout() {
  // TODO
  alert("Logging out");
}

/** For the main page. Looks like a button */
export function AuthButton({ accessToken, isMenu }: AuthButtonProps) {
  const router = useRouter();
  const isLoggedIn = typeof accessToken === "string";
  const client = echoClient(accessToken);

  return (
    <form
      onSubmit={() => {
        if (isLoggedIn) {
          logout();
        } else {
          login(client, router);
        }
      }}
    >
      <button
        type="submit"
        className={cn("rounded px-2 py-1 cursor-pointer", {
          border: !isMenu,
          hidden: !isLoggedIn && !isMenu,
        })}
      >
        {isLoggedIn ? "Logout" : "Login"}
      </button>
    </form>
  );
}
