import type { Metadata } from "next";
import Header from "@/app/header";

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
    <>
      <Header />
      {children}
    </>
  );
}
