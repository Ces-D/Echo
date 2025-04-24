import { echoClient } from "@/client";
import type { AuthorizationRequired } from "@/client";

export default async function HelloUserHeading({
  accessToken,
}: AuthorizationRequired) {
  const client = echoClient(accessToken);
  const { data } = await client.GET("/v1/current_user");

  return (
    <h1>
      Hello, {data?.name ?? data?.spotify.display_name ?? data?.spotify.email}!
    </h1>
  );
}
