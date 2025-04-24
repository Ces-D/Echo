import { getAuthorizationAccessToken } from "@/lib/server/utils";

import "./globals.css";
import { AuthButton } from "./_components/AuthButton";
import { PropsWithChildren } from "react";

function ProfileMenu({
  isLoggedIn = false,
  children,
}: PropsWithChildren<{ isLoggedIn: boolean }>) {
  return (
    <div className="dropdown dropdown-end">
      <div tabIndex={0} role="button" className="w-12 btn btn-square">
        <div className="avatar">
          <div className="w-full">
            <img src={isLoggedIn ? "/echo-logo.webp" : "/unknown-user.webp"} />
          </div>
        </div>
      </div>
      <ul
        tabIndex={0}
        className="p-2 w-52 shadow-sm dropdown-content menu bg-base-100 rounded-box z-1"
      >
        {children}
      </ul>
    </div>
  );
}

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const accessToken = await getAuthorizationAccessToken();

  return (
    <html lang="en">
      <body>
        <nav className="justify-between px-3 navbar">
          <h3 className="text-accent">Echo</h3>
          <ProfileMenu isLoggedIn={typeof accessToken === "string"}>
            <li>
              <AuthButton isMenu accessToken={accessToken} />
            </li>
          </ProfileMenu>
        </nav>
        {children}
      </body>
    </html>
  );
}
