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
      <body>
        {children}
        <Footer />
        <div className="bg-slate-600 h-20 text-white flex">
          <p className="m-auto">iwanhae@gmail.com</p>
          <a href="https://www.linkedin.com/in/iwanhae/" className="m-auto">
            LinkedIn
          </a>
        </div>
      </body>
    </html>
  );
}
