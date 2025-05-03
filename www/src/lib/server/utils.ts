import "server-only";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { cache } from "react";
import { echoClient } from "@/client";

/** Try to get an access token. Undefined if its not available */
export const getAuthorizationAccessToken = cache(async function (): Promise<
  string | undefined
> {
  const cookie = await cookies();
  if (process.env.SESSION_COOKIE_KEY === undefined) {
    throw new Error("SESSION_COOKIE_KEY is undefined");
  }

  const authorization = cookie.get(process.env.SESSION_COOKIE_KEY);
  if (authorization) {
    return authorization.value;
  }
});

/** Get the access token, or redirect the user to login if its not present
 *  USE FOR PROTECTED ROUTES
 * */
export const getRequiredAuthorizationAccessToken = cache(
  async (): Promise<string> => {
    const accessToken = await getAuthorizationAccessToken();
    if (typeof accessToken === "string") {
      return accessToken;
    } else {
      const client = echoClient(undefined);
      const authorizationUrl = await client.GET("/auth/spotify");
      if (authorizationUrl.data) {
        redirect(authorizationUrl.data.url);
      } else {
        throw new Error("Error getting authorization URL");
      }
    }
  },
);
