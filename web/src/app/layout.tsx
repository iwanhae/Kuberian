import Script from "next/script";
import Footer from "./footer";
import "./globals.css";
import type { Metadata } from "next";

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
      <Script src="/elastic-apm-rum.umd.min.js" />
      <body>
        {children}
        <Footer />
      </body>
    </html>
  );
}
