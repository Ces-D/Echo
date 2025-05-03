import { getAuthorizationAccessToken } from "@/lib/server/utils";

import "./globals.css";
import AuthButton from "./_components/authorization/AuthButton";
import UserProfileButton from "./_components/authorization/UserProfileButton";

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const accessTokenPromise = getAuthorizationAccessToken();
  return (
    <html lang="en">
      <body className="bg-background">
        <nav className="flex justify-between items-center px-1 h-20 shadow">
          <h3 className="text-accent">Echo</h3>
          <section className="flex gap-4">
            <UserProfileButton accessTokenPromise={accessTokenPromise} />
            <AuthButton accessTokenPromise={accessTokenPromise} />
          </section>
        </nav>
        {children}
      </body>
    </html>
  );
}
