import AuthorizationButton from "./_components/AuthorizationButton";
import { getAuthorizationAccessToken } from "@/utils";
import HelloUserHeading from "./_components/HelloUserHeading";

export default async function Home() {
  const accessToken = await getAuthorizationAccessToken();

  return (
    <div>
      {accessToken && <HelloUserHeading accessToken={accessToken} />}
      <AuthorizationButton accessToken={accessToken} />
    </div>
  );
}

export const dynamic = "force-dynamic";
