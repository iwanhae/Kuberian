import Card from "@/app/card";
import { SERVER_URL } from "@/config";
import { LinkExternalIcon } from "@primer/octicons-react";

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
        <div className="dummy border-sky-500 bg-sky-50 border-yellow-500 bg-yellow-50 border-orange-500 bg-orange-50 border-red-500 bg-red-50" />
        <div className="max-w-3xl w-full m-auto py-8 flex flex-col gap-5">
          {data.results
            .sort((a, b) => (a.score < b.score ? 1 : -1))
            .map((e) => {
              const doc = docs[e.id];
              let color = "gray";
              if (e.score > 0.8) color = "sky";
              else if (e.score > 0.6) color = "yellow";
              else if (e.score > 0.4) color = "orange";
              else color = "red";
              return (
                <Card key={e.id} className="flex flex-col overflow-x-auto">
                  <div className="flex gap-1 min-w-fit">
                    <a
                      className="text-2xl text-[#006bb8] hover:underline font-medium"
                      href={`https://github.com/kubernetes/kubernetes/blob/v1.27.4/${doc.file}#L${doc.line.start}-L${doc.line.end}`}
                      target="_blank"
                    >
                      {doc.name}
                    </a>
                    <LinkExternalIcon className="mt-auto mb-2" size={13} />
                    <div
                      className={`my-auto ml-auto border-2 border-${color}-500 bg-${color}-50 w-10 h-10 rounded-full flex`}
                    >
                      <p className="m-auto font-mono">
                        {(e.score * 100).toFixed()}
                      </p>
                    </div>
                  </div>
                  <p className="font-medium">{doc.file}</p>
                  <p>{doc.summary}</p>
                </Card>
              );
            })}
        </div>
      </main>
    </>
  );
}
