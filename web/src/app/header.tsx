"use client";
import { LinkExternalIcon, SearchIcon } from "@primer/octicons-react";
import { usePathname, useRouter } from "next/navigation";
import { useDebounce } from "usehooks-ts";
import { useEffect, useRef, useState } from "react";
import Spinner from "./spinner";

export default function Header(): JSX.Element {
  const inputRef = useRef<HTMLInputElement>(null);
  const [query, setQuery] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const debouncedQuery = useDebounce(query, 500);
  const router = useRouter();
  const pathname = usePathname();

  // push router
  useEffect(() => {
    if (debouncedQuery === "") router.push(`/`);
    else if (debouncedQuery !== null)
      router.push(`/q/${encodeURIComponent(debouncedQuery)}`);
  }, [router, debouncedQuery]);

  // set Loading status
  useEffect(() => {
    const q = decodeURIComponent(
      pathname.substring(pathname.lastIndexOf("/") + 1),
    );
    if (q !== debouncedQuery) setLoading(true);
    else setLoading(false);
  }, [debouncedQuery, pathname]);

  // set input field only for a initial execution
  useEffect(() => {
    const url = document.URL;
    const q = decodeURIComponent(url.substring(url.lastIndexOf("/") + 1));
    if (inputRef.current != null) inputRef.current.value = q;
    setQuery(q);
  }, []);

  return (
    <header className="h-72 bg-[#f7f8fc] border-b-[1px] p-2">
      <div className="max-w-3xl h-full m-auto pb-6 flex flex-col justify-end gap-2">
        <h3>
          <a href={"/"} className="hover:text-sky-500 transition-all">
            Kubernetes Librarian : Kuberian
          </a>
        </h3>
        <h1 className="text-3xl sm:text-4xl font-bold">Find functions that</h1>
        <div></div>
        <div className="w-full h-12 sm:h-14 bg-white flex hover:border-b-2 focus-within:border-b-2 focus-within:border-sky-400 transition-all">
          <SearchIcon className="w-9 sm:w-10 m-auto" size={20} />
          <input
            type="text"
            className="w-full text-2xl sm:text-3xl focus-visible:outline-none my-auto"
            placeholder="describe whatever you want to find"
            autoFocus
            ref={inputRef}
            onChange={(e) => {
              setQuery(e.target.value);
            }}
          />
        </div>
        <div></div>
        <h1 className="text-3xl sm:text-4xl font-bold flex gap-2">
          <p>from</p>
          <a
            className="text-[#006bb8] font-bold flex gap-1"
            href="https://github.com/kubernetes/kubernetes/tree/v1.27.4"
            target="_blank"
          >
            <p>Kubernetes</p>
            <LinkExternalIcon className="mt-auto mb-1" />
          </a>
          {loading && <Spinner className="my-auto ml-auto" />}
        </h1>
      </div>
    </header>
  );
}
