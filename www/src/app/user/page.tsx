import { getRequiredAuthorizationAccessToken } from "@/lib/server/utils";
import { echoClient } from "@/client";
import type { components } from "@/client/echo";
import PlaylistTable from "./_components/PlaylistsTable";
import { Suspense } from "react";

function UserProfile({
  user,
}: {
  user: components["schemas"]["CompleteUser"];
}) {
  const profileImage = user.spotify?.images[0];
  const displayName =
    user.name || user.spotify.display_name || user.spotify.email;

  return (
    <div className="flex gap-4 items-center">
      <img src={profileImage?.url} alt={`${displayName}'s profile image`} />
      <h1>{displayName}</h1>
    </div>
  );
}

export default async function UserPage() {
  const accessToken = await getRequiredAuthorizationAccessToken();
  const client = echoClient(accessToken);
  const user = await client.GET("/v1/current_user");
  const playlists = await client.GET("/v1/current_user/playlists");

  return (
    <main className="py-4 px-1 m-auto w-full md:w-11/12">
      {user.data ? (
        <>
          <UserProfile user={user.data} />
          <Suspense>
            <PlaylistTable playlists={playlists.data || []} />
          </Suspense>
        </>
      ) : (
        <p>Unable to get user information</p>
      )}
    </main>
  );
}
