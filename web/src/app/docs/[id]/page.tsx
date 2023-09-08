import FunctionCard from "@/components/functionCard";
import { SERVER_URL } from "@/config";
import { type DocResponse } from "@/types";
import GoBack from "./goBack";

export const runtime = "edge";

async function getData(query: number): Promise<DocResponse> {
  const res = await fetch(SERVER_URL(`/docs/${query}`), {
    next: { revalidate: 5 },
  });

  if (!res.ok) {
    throw new Error("Failed to fetch data");
  }

  return await res.json();
}

export default async function Page({
  params: { id },
}: {
  params: { id: number };
}): Promise<JSX.Element> {
  const data = await getData(id);
  return (
    <>
      <main className="flex flex-col">
        <div className="max-w-3xl w-full m-auto py-8 flex flex-col gap-5">
          <GoBack />
          <FunctionCard doc={data} />
        </div>
      </main>
    </>
  );
}
