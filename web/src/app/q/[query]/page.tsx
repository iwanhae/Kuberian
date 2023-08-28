import Card from "@/app/card";
import { SERVER_URL } from "@/config";

export const runtime = "edge";

interface Response {
  query: string;
  docs: Doc[];
  results: Result[];
}

interface Doc {
  id: number;
  name: string;
  signature: string;
  file: string;
  line: Line;
  summary: string;
}

interface Line {
  start: number;
  end: number;
}

interface Result {
  id: number;
  score: number;
}

async function getData(query: string): Promise<Response> {
  const res = await fetch(SERVER_URL(`/q/${query}`), {
    next: { revalidate: 5 },
  });

  if (!res.ok) {
    throw new Error("Failed to fetch data");
  }

  return await res.json();
}

export default async function Page({
  params: { query },
}: {
  params: { query: string };
}): Promise<JSX.Element> {
  const data = await getData(query);
  const docs: Record<string, Doc> = data.docs.reduce<Record<string, Doc>>(
    (prev, cur) => {
      prev[cur.id] = cur;
      return prev;
    },
    {},
  );
  return (
    <>
      <main className="flex flex-col">
        <div className="max-w-3xl w-full m-auto py-8 flex flex-col gap-5">
          {data.results
            .sort((a, b) => (a.score < b.score ? 1 : -1))
            .map((e) => {
              return (
                <Card key={e.id}>
                  {e.id} / {e.score} / {JSON.stringify(docs[e.id])}
                </Card>
              );
            })}
        </div>
      </main>
    </>
  );
}
