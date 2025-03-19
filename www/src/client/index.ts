import createClient, { Middleware } from "openapi-fetch";
import type { paths } from "./echo";

export type AccessToken = string | undefined;

export type AuthorizationRequired<T = unknown> = T & {
  accessToken: Required<AccessToken>;
};

/** Factory that creates the client for the echo server.
 * If an access token is provided, it will be used to authenticate the client for routes requiring it.
 * */
export const echoClient = (accessToken: AccessToken) => {
  const client = createClient<paths>({
    baseUrl: process.env.NEXT_PUBLIC_API_URL,
    credentials: "include",
  });

  if (typeof accessToken === "string") {
    client.use(authorizationHeader(accessToken));
  }

  return client;
};

const authorizationHeader = (accessToken: string): Middleware => {
  return {
    onRequest: async ({ request, schemaPath }) => {
      if (schemaPath.includes("v1")) {
        if (accessToken) {
          request.headers.set("Authorization", `Bearer ${accessToken}`);
        }
      }

      return request;
    },
  };
};
