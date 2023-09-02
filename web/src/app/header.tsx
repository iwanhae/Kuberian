"use client";
import { LinkExternalIcon, SearchIcon } from "@primer/octicons-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useDebounce } from "usehooks-ts";
import { useEffect, useState } from "react";
import Spinner from "./spinner";

interface Props {
  query?: string;
}

export default function Header(props: Props): JSX.Element {
  const [query, setQuery] = useState(props.query ?? "");
  const [loading, setLoading] = useState(false);
  const debouncedQuery = useDebounce(query, 300);
  const router = useRouter();

  // detect loading
  useEffect(() => {
    if (props.query !== debouncedQuery) setLoading(true);
    else setLoading(false);
  }, [props.query, debouncedQuery]);

  // push router
  useEffect(() => {
    if (debouncedQuery === "") router.push(`/`);
    else router.push(`/q/${debouncedQuery}`);
  }, [router, debouncedQuery]);

  return (
    <header className="h-72 bg-[#f7f8fc] border-b-[1px] p-2">
      <div className="max-w-3xl h-full m-auto pb-6 flex flex-col justify-end gap-2">
        <h3>
          <Link href={"/"} className="hover:text-sky-500 transition-all">
            Kubernetes Librarian : Kuberian
          </Link>
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
            value={query}
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
