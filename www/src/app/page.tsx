import { Suspense } from "react";

import { getAuthorizationAccessToken } from "@/utils";

import AuthorizationButton from "./_components/AuthorizationButton";
import HelloUserHeading from "./_components/HelloUserHeading";

export default async function Home() {
  const accessToken = await getAuthorizationAccessToken();

  return (
    <div>
      <Suspense>
        {accessToken && <HelloUserHeading accessToken={accessToken} />}
      </Suspense>
      <AuthorizationButton accessToken={accessToken} />
    </div>
  );
}

export const dynamic = "force-dynamic";
