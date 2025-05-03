"use client";

import { AccessToken, echoClient } from "@/client";
import { FormEventHandler, use } from "react";
import { useRouter } from "next/navigation";

type AuthButtonProps = {
  accessTokenPromise: Promise<AccessToken>;
};

/** Handle logging user in  */
export default function AuthButton({ accessTokenPromise }: AuthButtonProps) {
  const accessToken = use(accessTokenPromise);
  const router = useRouter();
  const client = echoClient(accessToken);
  const isLoggedIn = typeof accessToken === "string";

  const loginUser: FormEventHandler<HTMLFormElement> = async (e) => {
    e.preventDefault();
    let loginUrl = await client.GET("/auth/spotify");
    if (loginUrl.data) {
      console.log(loginUrl.data);
      router.push(loginUrl.data.url);
    } else {
      console.log("Error logging in");
    }
  };

  return (
    <form onSubmit={loginUser}>
      <button
        type="submit"
        disabled={isLoggedIn}
        className="py-1 px-2 h-16 rounded border cursor-pointer text-text-inverse-primary bg-prime"
      >
        {isLoggedIn ? "Logged In" : "Login"}
      </button>
    </form>
  );
}
