import { cookies } from "next/headers";

export async function getAuthorizationAccessToken(): Promise<string | undefined> {
  const cookie = await cookies();
  if (process.env.SESSION_COOKIE_KEY === undefined) {
    throw new Error("SESSION_COOKIE_KEY is undefined");
  }

  let authorization = cookie.get(process.env.SESSION_COOKIE_KEY);
  if (authorization) {
    return authorization.value;
  }
}
