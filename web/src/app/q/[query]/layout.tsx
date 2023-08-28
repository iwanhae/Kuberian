import type { Metadata } from "next";
import Header from "../../header";

export const metadata: Metadata = {
  title: "Kuberian",
  description: "Kubernetes Librarian : Kuberian",
};

export default function RootLayout({
  children,
  params: { query },
}: {
  children: React.ReactNode;
  params: { query?: string };
}): JSX.Element {
  return (
    <>
      <Header query={decodeURI(query ?? "")} />
      {children}
    </>
  );
}
