"use client";

import Card from "@/components/card";
import { ArrowLeftIcon } from "@primer/octicons-react";
import { useRouter } from "next/navigation";

export default function GoBack(): JSX.Element {
  const router = useRouter();
  return (
    <Card
      className="flex justify-center gap-5 cursor-pointer hover:bg-slate-100"
      onClick={router.back}
    >
      <ArrowLeftIcon className="my-auto" />
      <button className="my-auto font-mono">
        Go back to the search results
      </button>
    </Card>
  );
}
