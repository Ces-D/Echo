import { Suspense } from "react";

import { getAuthorizationAccessToken } from "@/lib/server/utils";
import { pageMetadata } from "@/lib/utils";

import { AuthButton } from "./_components/AuthButton";
import HelloUserHeading from "./_components/HelloUserHeading";

export const metadata = pageMetadata({});

export default async function Home() {
  const accessToken = await getAuthorizationAccessToken();

  return (
    <div>
      <Suspense>
        {accessToken && <HelloUserHeading accessToken={accessToken} />}
      </Suspense>
      <video autoPlay loop muted className="w-96">
        <source src="/neon-circle-footage.mp4" />
      </video>

      <AuthButton accessToken={accessToken} />
    </div>
  );
}

export const dynamic = "force-dynamic";
