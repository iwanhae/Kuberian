import "./globals.css";
import type { Metadata } from "next";
import Header from "./header";

export const metadata: Metadata = {
  title: "Kuberian",
  description: "Kubernetes Librarian : Kuberian",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element {
  return (
    <html lang="en">
      <body>
        <Header />
        {children}
      </body>
    </html>
  );
}
