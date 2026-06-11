import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Helios Wallet",
  description: "Smart-account-native wallet for Stellar",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
