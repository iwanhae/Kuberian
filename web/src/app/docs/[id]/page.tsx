import FunctionCard from "@/components/functionCard";
import { SERVER_URL } from "@/config";
import { type DocResponse } from "@/types";
import GoBack from "./goBack";
import SearchReselt from "@/app/q/[query]/searchResult";
import Card from "@/components/card";

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
  params,
}: {
  params: Promise<{ id: number }>;
}): Promise<JSX.Element> {
  const props = await params;
  const data = await getData(props.id);
  return (
    <>
      <main className="flex flex-col">
        <div className="max-w-3xl w-full m-auto py-8 flex flex-col gap-5">
          <GoBack />
          <FunctionCard doc={data} />
          {data.summary != null && (
            <>
              <Card className="flex justify-center gap-5 hover:bg-slate-100">
                <p className="my-auto font-mono">Related top 10</p>
              </Card>
              <SearchReselt query={data.summary} />
            </>
          )}
          <GoBack />
        </div>
      </main>
    </>
  );
}
